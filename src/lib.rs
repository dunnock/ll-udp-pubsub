use std::{
    net::SocketAddr,
    sync::{atomic::AtomicI64, Arc},
};

use serde::{Deserialize, Serialize};

const MTU: usize = 1500;

pub mod publisher;
pub mod publisher_controller;
pub mod subscriber;

#[derive(Serialize, Deserialize)]
pub enum ControlMessage {
    Subscribe,
    Unsubscribe,
}

#[derive(Serialize, Deserialize)]
pub struct Packet<Message> {
    pub data: Message,
    pub sent_ts: i64,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct ChannelMetrics {
    last_submit: Arc<AtomicI64>,
}

type Recipients = Arc<scc::HashIndex<SocketAddr, ChannelMetrics>>;

///
/// Get current system time in nanoseconds.
///
pub fn timestamp() -> i64 {
    let mut time = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    unsafe { libc::clock_gettime(libc::CLOCK_REALTIME, &mut time) };
    time.tv_sec * 1_000_000_000 + time.tv_nsec
}

/// Message handler for subscriber
pub trait Handler {
    type Message: for<'de> Deserialize<'de>;
    fn handle(&mut self, msg: Packet<Self::Message>, received_ts: i64);
}

/// Pin current thread to core
/// Note: all the created threads will inherit taskset
pub fn pin_to_core(core_idx: usize) {
    let cores = core_affinity::get_core_ids().expect("Failed to get list of cores");
    assert!(
        cores.len() > core_idx,
        "{core_idx} core is not available, cores.len()={}",
        cores.len()
    );
    core_affinity::set_for_current(cores[core_idx]);
}
