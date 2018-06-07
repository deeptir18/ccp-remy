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
    curr_intersend: f64 // (unrounded version of intersend in order to do userspace calculations)
}

pub struct RemyMeasurements {
    sendEwma: u32,
    slowRecvEwma: u32,
    fastRecvEwma: u32,
    rttRatio: u32,
    minRtt: u32
}

#[derive(Clone)]
pub struct RemyConfig {
    pub input_whiskers: String, // location of Remy input whiskers file
    // TODO make more things configurable
}

impl Default for RemyConfig {
    fn default() -> Self {
        RemyConfig { input_whiskers: String::from("/home/ubuntu/remy/tests/RemyCC-2014-100x.dna") }
    }
}

impl<T: Ipc> Remy<T> {
    // set an initial congestion window of sorts
    fn set_initial_window(&mut self) {
        self.control_channel.update_field(&self.sc, &[("Cwnd", (10.0 * self.curr_cwnd) as u32)]);
    }

    // signals to measure: rec_ewma (with 1/8), rec_ewma (with 1/256), send_ewma(with 1/8)
    // rtt ratio
    // note for some of these variables, we are reporting them times 100
    // note: Ack.ecn_bytes holds the rs->delivered * MTU * S_TO_US
    // the fastRecEwma and the slowRecEwma is scaled by 1000x
    fn install_fold(&self) -> Scope {
        let program = 
            b"
                def (
                    Report (
                        (minrtt +infinity)
                        (sendEwma 0)
                        (fastRecvEwma 0)
                        (slowRecvEwma 0)
                        (rttRatio 0)
                    )
                    (interReceive 0)
                    (interSend 0)
                    (delivered 0)
                )
                (when true
                    (:= delivered Ack.ecn_bytes)
                    (:= Report.minrtt (min Report.minrtt Flow.rtt_sample_us))
                    (:= rttRatio (/ (* 100 Flow.rtt_sample_us) Report.minrtt))
                    (:= interSend (/ (* delivered 1000) Flow.rate_incoming))
                    (:= interReceive (/ (* delivered 1000) Flow.outgoing))
                    (:= fastRecvEwma (/ (+ (* 7 Report.fastRecvEwma) (* 1 interReceive)) 8)
                    (:= slowRecvEwma (/ (+ (* 255 Report.fastRecvEwma) (* 1 interReceive)) 256)
                    (:= sendEwma (/ (* 7 Report.sendEwma) (* 1 interSend) 8)
                )
                (when (> Micros Report.minrtt)
                    (:= Micros 0)
                    (report)
                )

            ";
        self.control_channel.install(program, None).unwrap()
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

        RemyMeasurements {
            sendEwma,
            slowRecvEwma,
            fastRecvEwma,
            rttRatio,
            minRtt,
        }
    }

    fn scale_measurements(&mut self, mem: RemyMeasurements) -> remy::dna::Memory {
        let rttRatio = (mem.rttRatio as f64)/1000.0;
        let sendEwma = (mem.sendEwma as f64)/1000.0;
        let fastRecvEwma = (mem.fastRecvEwma as f64)/1000.0;
        let slowRecvEwma = (mem.slowRecvEwma as f64)/1000.0;

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
        };
        
        s.logger.as_ref().map(|log| {
            debug!(log, "starting remy flow";);
        });

        {
            s.set_initial_window(); 
        }

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
            );
        });

        let mut memory = self.scale_measurements(remy_ms);
        println!("Corresponding memory!: {:?}", memory);
        let whisker = self.whisker_tree.find_whisker(memory);
        println!("Corresponding whisker: {:?}", whisker);

        // TODO: update action

    }
}
