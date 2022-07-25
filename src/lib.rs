use serde::Deserialize;

const MTU: usize = 1500;

pub mod publisher;
pub mod subscriber;

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
    fn handle(&mut self, msg: Self::Message, received_ts: i64);
}
