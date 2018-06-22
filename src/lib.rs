#[macro_use]
extern crate slog;
extern crate time;
extern crate portus;
extern crate quick_protobuf;
use portus::{CongAlg, Config, Datapath, DatapathInfo, DatapathTrait, Report};
use portus::ipc::Ipc;
use portus::lang::Scope;
use std::fmt;
use std::fs;
use quick_protobuf::{MessageRead, BytesReader};
mod remy;

// parses a remy into a whisker tree given the path to the protobuf file
fn parse_remy(whiskers: String) -> remy::dna::WhiskerTree {
        let mut bytes: Vec<u8>;
        bytes = fs::read(whiskers).expect("Unable to read whisker protobuf");
        let mut reader = BytesReader::from_bytes(&bytes);
        remy::dna::WhiskerTree::from_reader(&mut reader, &bytes).expect("Cannot read whisker tree")
}

pub struct Remy<T: Ipc> {
    control_channel: Datapath<T>,
    logger: Option<slog::Logger>,
    sc: Scope,
    mss: u32,
    whisker_tree: remy::dna::WhiskerTree,
    curr_cwnd: f64, // (unrounded version of cwnd to do userspace calculations)
    curr_intersend: f64, // (unrounded version of intersend in order to do userspace calculations)
    training_linkrate: u32,
    linkrate: u32
}

pub struct RemyMeasurements {
    sendEwma: u32,
    slowRecvEwma: u32,
    fastRecvEwma: u32,
    rttRatio: u32,
    minRtt: u32,
    sendRate: u32,
    recvRate: u32,
    delivered: u32,
    delivered_pkts: u32,
}

#[derive(Clone)]
pub struct RemyConfig {
    pub input_whiskers: String, // location of Remy input whiskers file
    pub training_linkrate: u32,
    pub linkrate: u32,
    // TODO make more things configurable
}

impl Default for RemyConfig {
    fn default() -> Self {
        RemyConfig { input_whiskers: String::from("/home/ubuntu/remy/tests/RemyCC-2014-100x.dna"),
                     training_linkrate: 32,
                     linkrate: 32}
    }
}

impl<T: Ipc> Remy<T> {
    fn update_cwnd(&self) {
        self.control_channel.update_field( &self.sc, &[("Cwnd", self.curr_cwnd as u32)]).unwrap();
    }

    fn update_rate_cwnd(&self) {
        let rate = ((1.0/self.curr_intersend) * (self.mss as f64) * 1000.0);
        self.logger.as_ref().map(|log| {
            debug!(log, "updating rate and cwnd";
                "cwnd" => self.curr_cwnd,
                "rate" => rate,
                "rate in mbps" => rate/125000.0,
            );
        });
        self.control_channel.update_field( &self.sc, &[("Cwnd", (self.curr_cwnd as u32) * self.mss), ("Rate", rate as u32)]).unwrap();
    }
    // signals to measure: rec_ewma (with 1/8), rec_ewma (with 1/256), send_ewma(with 1/8)
    // rtt ratio
    // note for some of these variables, we are reporting them times 1000
    // note: Ack.ecn_bytes holds the rs->delivered * MTU * S_TO_US
    // the fastRecEwma and the slowRecEwma is scaled by 1000x
    fn install_fold(&self) -> Scope {
        println!("Installing fold");
        self.control_channel.install(
            b"
                (def
                    (Report
                        (minrtt +infinity)
                        (sendEwma 0)
                        (fastRecvEwma 0)
                        (slowRecvEwma 0)
                        (rttRatio 0)
                        (sendRate 0)
                        (recvRate 0)
                        (delivered 0)
                        (delivered_pkts 0)
                    )
                    (interReceive 0)
                    (interSend 0)
                    (delivered 0)
                    (normalizingFactor 0)
                    (numerator 0)
                )
                (when true
                    (:= Report.delivered Ack.ecn_bytes)
                    (:= Report.delivered_pkts Ack.ecn_packets)
                    (:= Report.sendRate Flow.rate_outgoing)
                    (:= Report.recvRate Flow.rate_incoming)
                    (:= delivered Ack.ecn_bytes)
                    (:= Report.minrtt (min Report.minrtt Flow.rtt_sample_us))
                    (:= Report.rttRatio (/ (* 1000 Flow.rtt_sample_us) Report.minrtt))
                    (:= interSend (/ numerator Flow.rate_incoming))
                    (:= interReceive (/ numerator Flow.rate_outgoing))
                    (:= Report.fastRecvEwma (/ (+ (* 7 Report.fastRecvEwma) (* 1 interReceive)) 8))
                    (:= Report.fastRecvEwma (* Report.fastRecvEwma normalizingFactor))
                    (:= Report.slowRecvEwma (/ (+ (* 255 Report.fastRecvEwma) (* 1 interReceive)) 256))
                    (:= Report.slowRecvEwma (* Report.slowRecvEwma normalizingFactor))
                    (:= Report.sendEwma (/ (+ (* 7 Report.sendEwma) (* 1 interSend)) 8))
                    (:= Report.sendEwma (* Report.sendEwma normalizingFactor))
                    (fallthrough)
                    (report)
                )",
                /*(when (> Micros Report.minrtt)
                    (:= Micros 0)
                    (report)
                )",*/
                Some(&[("normalizingFactor", (self.linkrate as f64/self.training_linkrate as f64) as u32),
                            ("numerator", (1000*1000*1500) as u32)][..])).unwrap()
    }

    fn get_fields(&mut self, m: &Report) -> RemyMeasurements {
        let sc = &self.sc;
        let minRtt = m.get_field(&String::from("Report.minrtt"), sc).expect(
            "expected minrtt field in returned measurement"
        ) as u32;

        let sendEwma = m.get_field(&String::from("Report.sendEwma"), sc).expect(
            "expected sendEwma field in returned measurement"
        ) as u32;

        let fastRecvEwma = m.get_field(&String::from("Report.fastRecvEwma"), sc).expect(
            "expected fastRecvEwma field in returned measurement"
        ) as u32;

        let slowRecvEwma = m.get_field(&String::from("Report.slowRecvEwma"), sc).expect(
            "expected slowRecvEwma field in returned measurement"
        ) as u32;

        let rttRatio = m.get_field(&String::from("Report.rttRatio"), sc).expect(
            "expected rttRatio field in returned measurement"
        ) as u32;
        
        let sendRate = m.get_field(&String::from("Report.sendRate"), sc).expect(
            "expected sendRate field in returned measurement"
        ) as u32;

        let recvRate = m.get_field(&String::from("Report.recvRate"), sc).expect(
            "expected recvRate field in returned measurement"
        ) as u32;

        let delivered = m.get_field(&String::from("Report.delivered"), sc).expect(
            "expected delivered field in returned measurement"
        ) as u32;

        let delivered_pkts = m.get_field(&String::from("Report.delivered_pkts"), sc).expect(
            "expected delivered pkts field in returned measurement"
        ) as u32;
        RemyMeasurements {
            sendEwma,
            slowRecvEwma,
            fastRecvEwma,
            rttRatio,
            minRtt,
            sendRate,
            recvRate,
            delivered,
            delivered_pkts
        }
    }

    fn bps_to_pkts_per_ms(&mut self, num: f64) -> f64 {
        return num/(1000.0 * self.mss as f64);
    }

    fn scale_measurements(&mut self, mem: RemyMeasurements) -> remy::dna::Memory {
        let rttRatio = (mem.rttRatio as f64)/1000.0;
        // for rates - convert back into packets/ms
        let sendEwma = (mem.sendEwma as f64)/1000.0;
        let fastRecvEwma = (mem.fastRecvEwma as f64)/(1000.0);
        let slowRecvEwma = (mem.slowRecvEwma as f64)/(1000.0);

        remy::dna::Memory{
           rec_send_ewma: Some(sendEwma),
           rec_rec_ewma: Some(fastRecvEwma),
           rtt_ratio: Some(rttRatio),
           slow_rec_rec_ewma: Some(slowRecvEwma),
           rtt_diff: None,
           queueing_delay: None
        }
    }
}

impl<T: Ipc> CongAlg<T> for Remy<T> {
    type Config = RemyConfig;

    fn name() -> String {
        String::from("remy")
    }
    
    fn create(control: Datapath<T>, cfg: Config<T, Remy<T>>, info: DatapathInfo) -> Self {
        let mut s = Self {
            control_channel: control,
            logger: cfg.logger,
            sc: Default::default(),
            mss: info.mss,
            whisker_tree: parse_remy(cfg.config.input_whiskers.clone()),
            curr_cwnd: 10.0,
            curr_intersend: 0.0, // remember not to set rate if intersend is 0
            training_linkrate: cfg.config.training_linkrate.clone(),
            linkrate: cfg.config.linkrate.clone()
        };
        
        s.logger.as_ref().map(|log| {
            debug!(log, "starting remy flow";);
        });
        //println!("whisker tree: {:?}", s.whisker_tree.clone());
        s.sc = s.install_fold();
        s
    }   

    fn on_report(&mut self, _sock_id: u32, m: Report) {
        let mut remy_ms = self.get_fields(&m);
        self.logger.as_ref().map(|log| {
            debug!(log, "got report";
                "minrtt" => remy_ms.minRtt,
                "rttRatio" => remy_ms.rttRatio,
                "sendEwma" => remy_ms.sendEwma,
                "fastRecvEwma" => remy_ms.fastRecvEwma,
                "slowRecvEwma" => remy_ms.slowRecvEwma,
                "sendRate" => remy_ms.sendRate,
                "send rate mbps" => remy_ms.sendRate as f64/125000.0,
                "recvRate" => remy_ms.recvRate,
                "recv rate mbps" => remy_ms.recvRate as f64/125000.0,
                "delivered" => remy_ms.delivered,
                "delivered pkts" => remy_ms.delivered_pkts
            );
        });

        let mut memory = self.scale_measurements(remy_ms);
        println!("memory: {:?}", memory.clone());
        let mut whisker = self.whisker_tree.find_whisker(memory).unwrap(); // TODO: don't unwrap this
        self.curr_cwnd = self.curr_cwnd * whisker.window_multiple() + (whisker.window_increment() as f64);
        self.curr_intersend = whisker.intersend();
        println!("whisker being used: {:?}", whisker);

        // TODO: update action
        self.update_rate_cwnd();

    }
}
