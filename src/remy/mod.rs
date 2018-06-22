/// This file contains the userspace implementation of the Remy search tree.
/// The struct definitions are in dna.rs, along with the protobuf serialization code.
/// These methods are modeled after the Remy simulator code located at
/// https://github.com/tcpexmachina/remy.
// TODO: don't use unwrap everywhere - be better at error handling

pub mod dna;

impl dna::Whisker {
    pub fn window_increment(&mut self) -> i32 {
        return self.window_increment.unwrap();
    }

    pub fn window_multiple(&mut self) -> f64 {
        return self.window_multiple.unwrap();
    }

    pub fn intersend(&mut self) -> f64 {
        return self.intersend.unwrap();
    }
}

impl dna::WhiskerTree {
    /*Given a memory, returns the whisker associated with this memory*/
    pub fn find_whisker(&mut self, memory: dna::Memory) -> Option<dna::Whisker> {
        if !self.domain.clone().unwrap().contains(memory.clone()) {
            return None;
        }
        
        match self.leaf {
            Some(ref w) => { return Some(w.clone()); },
            None => {},
        };

        for child in &self.children {
            match child.clone().find_whisker(memory.clone()) {
                None => {},
                Some(w) => { return Some(w.clone()); },
            }
        }

        unreachable!() 
    }
}

impl dna::Memory {
    fn field(&mut self, signal: &dna::mod_MemoryRange::Axis) -> f64 {
        let ret = match *signal {
            dna::mod_MemoryRange::Axis::SEND_EWMA => self.rec_send_ewma.unwrap(),
            dna::mod_MemoryRange::Axis::REC_EWMA => self.rec_rec_ewma.unwrap(),
            dna::mod_MemoryRange::Axis::RTT_RATIO => self.rtt_ratio.unwrap(),
            dna::mod_MemoryRange::Axis::SLOW_REC_EWMA => self.slow_rec_rec_ewma.unwrap(),
            dna::mod_MemoryRange::Axis::RTT_DIFF => self.rtt_diff.unwrap(),
            dna::mod_MemoryRange::Axis::QUEUEING_DELAY => self.queueing_delay.unwrap()
        };
        ret
    }
}

impl dna::MemoryRange {
    fn contains(&mut self, mut query: dna::Memory) -> bool {
        let mut lower = self.lower.clone().unwrap();
        let mut upper = self.upper.clone().unwrap();
        //let mut active_axis = Vec::new();
        //vec.push(dna::mod_MemoryRange::Axis::SEND_EWMA);
        let active_axis = vec![dna::mod_MemoryRange::Axis::SEND_EWMA, 
                           dna::mod_MemoryRange::Axis::REC_EWMA,
                           dna::mod_MemoryRange::Axis::RTT_RATIO,
                           dna::mod_MemoryRange::Axis::SLOW_REC_EWMA];
        for signal in &active_axis {
            if !(query.field(signal) >= lower.field(signal) && query.field(signal) <= upper.field(signal)) {
                return false;
            }
        }
        true
    }
}
