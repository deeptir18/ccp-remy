//! Automatically generated rust module for 'dna.proto' file
//! https://github.com/tafia/quick-protobuf

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use std::io::Write;
use quick_protobuf::{MessageRead, MessageWrite, BytesReader, Writer, Result};
use quick_protobuf::sizeofs::*;
use super::*;
#[derive(Debug, Default, PartialEq, Clone)]
pub struct WhiskerTree {
    pub domain: Option<dna::MemoryRange>,
    pub children: Vec<dna::WhiskerTree>,
    pub leaf: Option<dna::Whisker>,
    pub config: Option<dna::ConfigRange>,
    pub optimizer: Option<dna::OptimizationSettings>,
    pub configvector: Option<dna::ConfigVector>,
}

impl<'a> MessageRead<'a> for WhiskerTree {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.domain = Some(r.read_message::<dna::MemoryRange>(bytes)?),
                Ok(18) => msg.children.push(r.read_message::<dna::WhiskerTree>(bytes)?),
                Ok(26) => msg.leaf = Some(r.read_message::<dna::Whisker>(bytes)?),
                Ok(34) => msg.config = Some(r.read_message::<dna::ConfigRange>(bytes)?),
                Ok(42) => msg.optimizer = Some(r.read_message::<dna::OptimizationSettings>(bytes)?),
                Ok(50) => msg.configvector = Some(r.read_message::<dna::ConfigVector>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for WhiskerTree {
    fn get_size(&self) -> usize {
        0
        + self.domain.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.children.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.leaf.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.config.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.optimizer.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.configvector.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.domain { w.write_with_tag(10, |w| w.write_message(s))?; }
        for s in &self.children { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.leaf { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.config { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.optimizer { w.write_with_tag(42, |w| w.write_message(s))?; }
        if let Some(ref s) = self.configvector { w.write_with_tag(50, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct FinTree {
    pub domain: Option<dna::MemoryRange>,
    pub children: Vec<dna::FinTree>,
    pub leaf: Option<dna::Fin>,
    pub config: Option<dna::ConfigRange>,
    pub optimizer: Option<dna::OptimizationSettings>,
    pub configvector: Option<dna::ConfigVector>,
}

impl<'a> MessageRead<'a> for FinTree {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(722) => msg.domain = Some(r.read_message::<dna::MemoryRange>(bytes)?),
                Ok(730) => msg.children.push(r.read_message::<dna::FinTree>(bytes)?),
                Ok(738) => msg.leaf = Some(r.read_message::<dna::Fin>(bytes)?),
                Ok(746) => msg.config = Some(r.read_message::<dna::ConfigRange>(bytes)?),
                Ok(754) => msg.optimizer = Some(r.read_message::<dna::OptimizationSettings>(bytes)?),
                Ok(762) => msg.configvector = Some(r.read_message::<dna::ConfigVector>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for FinTree {
    fn get_size(&self) -> usize {
        0
        + self.domain.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.children.iter().map(|s| 2 + sizeof_len((s).get_size())).sum::<usize>()
        + self.leaf.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.config.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.optimizer.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.configvector.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.domain { w.write_with_tag(722, |w| w.write_message(s))?; }
        for s in &self.children { w.write_with_tag(730, |w| w.write_message(s))?; }
        if let Some(ref s) = self.leaf { w.write_with_tag(738, |w| w.write_message(s))?; }
        if let Some(ref s) = self.config { w.write_with_tag(746, |w| w.write_message(s))?; }
        if let Some(ref s) = self.optimizer { w.write_with_tag(754, |w| w.write_message(s))?; }
        if let Some(ref s) = self.configvector { w.write_with_tag(762, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct MemoryRange {
    pub lower: Option<dna::Memory>,
    pub upper: Option<dna::Memory>,
    pub active_axis: Vec<dna::mod_MemoryRange::Axis>,
}

impl<'a> MessageRead<'a> for MemoryRange {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(90) => msg.lower = Some(r.read_message::<dna::Memory>(bytes)?),
                Ok(98) => msg.upper = Some(r.read_message::<dna::Memory>(bytes)?),
                Ok(104) => msg.active_axis.push(r.read_enum(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for MemoryRange {
    fn get_size(&self) -> usize {
        0
        + self.lower.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.upper.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.active_axis.iter().map(|s| 1 + sizeof_varint(*(s) as u64)).sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.lower { w.write_with_tag(90, |w| w.write_message(s))?; }
        if let Some(ref s) = self.upper { w.write_with_tag(98, |w| w.write_message(s))?; }
        for s in &self.active_axis { w.write_with_tag(104, |w| w.write_enum(*s as i32))?; }
        Ok(())
    }
}

pub mod mod_MemoryRange {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Axis {
    SEND_EWMA = 0,
    REC_EWMA = 1,
    RTT_RATIO = 2,
    SLOW_REC_EWMA = 3,
    RTT_DIFF = 4,
    QUEUEING_DELAY = 5,
}

impl Default for Axis {
    fn default() -> Self {
        Axis::SEND_EWMA
    }
}

impl From<i32> for Axis {
    fn from(i: i32) -> Self {
        match i {
            0 => Axis::SEND_EWMA,
            1 => Axis::REC_EWMA,
            2 => Axis::RTT_RATIO,
            3 => Axis::SLOW_REC_EWMA,
            4 => Axis::RTT_DIFF,
            5 => Axis::QUEUEING_DELAY,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for Axis {
    fn from(s: &'a str) -> Self {
        match s {
            "SEND_EWMA" => Axis::SEND_EWMA,
            "REC_EWMA" => Axis::REC_EWMA,
            "RTT_RATIO" => Axis::RTT_RATIO,
            "SLOW_REC_EWMA" => Axis::SLOW_REC_EWMA,
            "RTT_DIFF" => Axis::RTT_DIFF,
            "QUEUEING_DELAY" => Axis::QUEUEING_DELAY,
            _ => Self::default(),
        }
    }
}

}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Memory {
    pub rec_send_ewma: Option<f64>,
    pub rec_rec_ewma: Option<f64>,
    pub rtt_ratio: Option<f64>,
    pub slow_rec_rec_ewma: Option<f64>,
    pub rtt_diff: Option<f64>,
    pub queueing_delay: Option<f64>,
}

impl<'a> MessageRead<'a> for Memory {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(169) => msg.rec_send_ewma = Some(r.read_double(bytes)?),
                Ok(177) => msg.rec_rec_ewma = Some(r.read_double(bytes)?),
                Ok(185) => msg.rtt_ratio = Some(r.read_double(bytes)?),
                Ok(193) => msg.slow_rec_rec_ewma = Some(r.read_double(bytes)?),
                Ok(201) => msg.rtt_diff = Some(r.read_double(bytes)?),
                Ok(209) => msg.queueing_delay = Some(r.read_double(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Memory {
    fn get_size(&self) -> usize {
        0
        + self.rec_send_ewma.as_ref().map_or(0, |_| 2 + 8)
        + self.rec_rec_ewma.as_ref().map_or(0, |_| 2 + 8)
        + self.rtt_ratio.as_ref().map_or(0, |_| 2 + 8)
        + self.slow_rec_rec_ewma.as_ref().map_or(0, |_| 2 + 8)
        + self.rtt_diff.as_ref().map_or(0, |_| 2 + 8)
        + self.queueing_delay.as_ref().map_or(0, |_| 2 + 8)
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.rec_send_ewma { w.write_with_tag(169, |w| w.write_double(*s))?; }
        if let Some(ref s) = self.rec_rec_ewma { w.write_with_tag(177, |w| w.write_double(*s))?; }
        if let Some(ref s) = self.rtt_ratio { w.write_with_tag(185, |w| w.write_double(*s))?; }
        if let Some(ref s) = self.slow_rec_rec_ewma { w.write_with_tag(193, |w| w.write_double(*s))?; }
        if let Some(ref s) = self.rtt_diff { w.write_with_tag(201, |w| w.write_double(*s))?; }
        if let Some(ref s) = self.queueing_delay { w.write_with_tag(209, |w| w.write_double(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Whisker {
    pub window_increment: Option<i32>,
    pub window_multiple: Option<f64>,
    pub intersend: Option<f64>,
    pub domain: Option<dna::MemoryRange>,
}

impl<'a> MessageRead<'a> for Whisker {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(248) => msg.window_increment = Some(r.read_sint32(bytes)?),
                Ok(257) => msg.window_multiple = Some(r.read_double(bytes)?),
                Ok(265) => msg.intersend = Some(r.read_double(bytes)?),
                Ok(274) => msg.domain = Some(r.read_message::<dna::MemoryRange>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Whisker {
    fn get_size(&self) -> usize {
        0
        + self.window_increment.as_ref().map_or(0, |m| 2 + sizeof_sint32(*(m)))
        + self.window_multiple.as_ref().map_or(0, |_| 2 + 8)
        + self.intersend.as_ref().map_or(0, |_| 2 + 8)
        + self.domain.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.window_increment { w.write_with_tag(248, |w| w.write_sint32(*s))?; }
        if let Some(ref s) = self.window_multiple { w.write_with_tag(257, |w| w.write_double(*s))?; }
        if let Some(ref s) = self.intersend { w.write_with_tag(265, |w| w.write_double(*s))?; }
        if let Some(ref s) = self.domain { w.write_with_tag(274, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Fin {
    pub lambda: Option<f64>,
    pub domain: Option<dna::MemoryRange>,
}

impl<'a> MessageRead<'a> for Fin {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(297) => msg.lambda = Some(r.read_double(bytes)?),
                Ok(306) => msg.domain = Some(r.read_message::<dna::MemoryRange>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Fin {
    fn get_size(&self) -> usize {
        0
        + self.lambda.as_ref().map_or(0, |_| 2 + 8)
        + self.domain.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.lambda { w.write_with_tag(297, |w| w.write_double(*s))?; }
        if let Some(ref s) = self.domain { w.write_with_tag(306, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct OptimizationSetting {
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub min_change: Option<f64>,
    pub max_change: Option<f64>,
    pub multiplier: Option<f64>,
    pub default_value: Option<f64>,
}

impl<'a> MessageRead<'a> for OptimizationSetting {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(329) => msg.min_value = Some(r.read_double(bytes)?),
                Ok(337) => msg.max_value = Some(r.read_double(bytes)?),
                Ok(345) => msg.min_change = Some(r.read_double(bytes)?),
                Ok(353) => msg.max_change = Some(r.read_double(bytes)?),
                Ok(361) => msg.multiplier = Some(r.read_double(bytes)?),
                Ok(369) => msg.default_value = Some(r.read_double(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for OptimizationSetting {
    fn get_size(&self) -> usize {
        0
        + self.min_value.as_ref().map_or(0, |_| 2 + 8)
        + self.max_value.as_ref().map_or(0, |_| 2 + 8)
        + self.min_change.as_ref().map_or(0, |_| 2 + 8)
        + self.max_change.as_ref().map_or(0, |_| 2 + 8)
        + self.multiplier.as_ref().map_or(0, |_| 2 + 8)
        + self.default_value.as_ref().map_or(0, |_| 2 + 8)
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.min_value { w.write_with_tag(329, |w| w.write_double(*s))?; }
        if let Some(ref s) = self.max_value { w.write_with_tag(337, |w| w.write_double(*s))?; }
        if let Some(ref s) = self.min_change { w.write_with_tag(345, |w| w.write_double(*s))?; }
        if let Some(ref s) = self.max_change { w.write_with_tag(353, |w| w.write_double(*s))?; }
        if let Some(ref s) = self.multiplier { w.write_with_tag(361, |w| w.write_double(*s))?; }
        if let Some(ref s) = self.default_value { w.write_with_tag(369, |w| w.write_double(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct OptimizationSettings {
    pub window_increment: Option<dna::OptimizationSetting>,
    pub window_multiple: Option<dna::OptimizationSetting>,
    pub intersend: Option<dna::OptimizationSetting>,
    pub lambda: Option<dna::OptimizationSetting>,
}

impl<'a> MessageRead<'a> for OptimizationSettings {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(410) => msg.window_increment = Some(r.read_message::<dna::OptimizationSetting>(bytes)?),
                Ok(418) => msg.window_multiple = Some(r.read_message::<dna::OptimizationSetting>(bytes)?),
                Ok(426) => msg.intersend = Some(r.read_message::<dna::OptimizationSetting>(bytes)?),
                Ok(434) => msg.lambda = Some(r.read_message::<dna::OptimizationSetting>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for OptimizationSettings {
    fn get_size(&self) -> usize {
        0
        + self.window_increment.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.window_multiple.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.intersend.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.lambda.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.window_increment { w.write_with_tag(410, |w| w.write_message(s))?; }
        if let Some(ref s) = self.window_multiple { w.write_with_tag(418, |w| w.write_message(s))?; }
        if let Some(ref s) = self.intersend { w.write_with_tag(426, |w| w.write_message(s))?; }
        if let Some(ref s) = self.lambda { w.write_with_tag(434, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Range {
    pub low: Option<f64>,
    pub high: Option<f64>,
    pub incr: Option<f64>,
}

impl<'a> MessageRead<'a> for Range {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(489) => msg.low = Some(r.read_double(bytes)?),
                Ok(497) => msg.high = Some(r.read_double(bytes)?),
                Ok(505) => msg.incr = Some(r.read_double(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Range {
    fn get_size(&self) -> usize {
        0
        + self.low.as_ref().map_or(0, |_| 2 + 8)
        + self.high.as_ref().map_or(0, |_| 2 + 8)
        + self.incr.as_ref().map_or(0, |_| 2 + 8)
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.low { w.write_with_tag(489, |w| w.write_double(*s))?; }
        if let Some(ref s) = self.high { w.write_with_tag(497, |w| w.write_double(*s))?; }
        if let Some(ref s) = self.incr { w.write_with_tag(505, |w| w.write_double(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ConfigRange {
    pub link_packets_per_ms: Option<dna::Range>,
    pub rtt: Option<dna::Range>,
    pub num_senders: Option<dna::Range>,
    pub buffer_size: Option<dna::Range>,
    pub mean_off_duration: Option<dna::Range>,
    pub mean_on_duration: Option<dna::Range>,
    pub simulation_ticks: Option<u32>,
    pub stochastic_loss_rate: Option<dna::Range>,
}

impl<'a> MessageRead<'a> for ConfigRange {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(570) => msg.link_packets_per_ms = Some(r.read_message::<dna::Range>(bytes)?),
                Ok(578) => msg.rtt = Some(r.read_message::<dna::Range>(bytes)?),
                Ok(586) => msg.num_senders = Some(r.read_message::<dna::Range>(bytes)?),
                Ok(594) => msg.buffer_size = Some(r.read_message::<dna::Range>(bytes)?),
                Ok(602) => msg.mean_off_duration = Some(r.read_message::<dna::Range>(bytes)?),
                Ok(610) => msg.mean_on_duration = Some(r.read_message::<dna::Range>(bytes)?),
                Ok(616) => msg.simulation_ticks = Some(r.read_uint32(bytes)?),
                Ok(626) => msg.stochastic_loss_rate = Some(r.read_message::<dna::Range>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ConfigRange {
    fn get_size(&self) -> usize {
        0
        + self.link_packets_per_ms.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.rtt.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.num_senders.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.buffer_size.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.mean_off_duration.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.mean_on_duration.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
        + self.simulation_ticks.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.stochastic_loss_rate.as_ref().map_or(0, |m| 2 + sizeof_len((m).get_size()))
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.link_packets_per_ms { w.write_with_tag(570, |w| w.write_message(s))?; }
        if let Some(ref s) = self.rtt { w.write_with_tag(578, |w| w.write_message(s))?; }
        if let Some(ref s) = self.num_senders { w.write_with_tag(586, |w| w.write_message(s))?; }
        if let Some(ref s) = self.buffer_size { w.write_with_tag(594, |w| w.write_message(s))?; }
        if let Some(ref s) = self.mean_off_duration { w.write_with_tag(602, |w| w.write_message(s))?; }
        if let Some(ref s) = self.mean_on_duration { w.write_with_tag(610, |w| w.write_message(s))?; }
        if let Some(ref s) = self.simulation_ticks { w.write_with_tag(616, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.stochastic_loss_rate { w.write_with_tag(626, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct NetConfig {
    pub mean_on_duration: Option<f64>,
    pub mean_off_duration: Option<f64>,
    pub num_senders: Option<u32>,
    pub link_ppt: Option<f64>,
    pub delay: Option<f64>,
    pub buffer_size: Option<u32>,
    pub stochastic_loss_rate: Option<f64>,
}

impl<'a> MessageRead<'a> for NetConfig {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(9) => msg.mean_on_duration = Some(r.read_double(bytes)?),
                Ok(17) => msg.mean_off_duration = Some(r.read_double(bytes)?),
                Ok(24) => msg.num_senders = Some(r.read_uint32(bytes)?),
                Ok(33) => msg.link_ppt = Some(r.read_double(bytes)?),
                Ok(41) => msg.delay = Some(r.read_double(bytes)?),
                Ok(48) => msg.buffer_size = Some(r.read_uint32(bytes)?),
                Ok(57) => msg.stochastic_loss_rate = Some(r.read_double(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for NetConfig {
    fn get_size(&self) -> usize {
        0
        + self.mean_on_duration.as_ref().map_or(0, |_| 1 + 8)
        + self.mean_off_duration.as_ref().map_or(0, |_| 1 + 8)
        + self.num_senders.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.link_ppt.as_ref().map_or(0, |_| 1 + 8)
        + self.delay.as_ref().map_or(0, |_| 1 + 8)
        + self.buffer_size.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.stochastic_loss_rate.as_ref().map_or(0, |_| 1 + 8)
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.mean_on_duration { w.write_with_tag(9, |w| w.write_double(*s))?; }
        if let Some(ref s) = self.mean_off_duration { w.write_with_tag(17, |w| w.write_double(*s))?; }
        if let Some(ref s) = self.num_senders { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.link_ppt { w.write_with_tag(33, |w| w.write_double(*s))?; }
        if let Some(ref s) = self.delay { w.write_with_tag(41, |w| w.write_double(*s))?; }
        if let Some(ref s) = self.buffer_size { w.write_with_tag(48, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.stochastic_loss_rate { w.write_with_tag(57, |w| w.write_double(*s))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ConfigVector {
    pub config: Vec<dna::NetConfig>,
}

impl<'a> MessageRead<'a> for ConfigVector {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(650) => msg.config.push(r.read_message::<dna::NetConfig>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ConfigVector {
    fn get_size(&self) -> usize {
        0
        + self.config.iter().map(|s| 2 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: Write>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.config { w.write_with_tag(650, |w| w.write_message(s))?; }
        Ok(())
    }
}

