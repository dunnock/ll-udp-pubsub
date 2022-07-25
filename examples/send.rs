use std::{net::SocketAddr, time::Duration};

use clap::Parser;
use ll_udp_pubsub::{publisher::UdpPublisher, timestamp};

#[derive(clap::Parser)]
/// Send counter with specified timeout
struct Cmd {
    /// Address to bind publisher to
    #[clap(short = 's')]
    server_addr: SocketAddr,
    /// Address where client is listening
    #[clap(short = 'c')]
    client_addr: SocketAddr,
    /// Timeout to send messages in microseconds
    #[clap(short = 't', default_value = "1000")]
    timeout_micros: u64,
    /// Number of messages to send
    #[clap(short = 'n', default_value = "100")]
    number: usize,
}

fn main() {
    let opts = Cmd::parse();
    let mut publisher = UdpPublisher::new(opts.server_addr).unwrap();
    let recipients = vec![opts.client_addr];
    let timeout = Duration::from_micros(opts.timeout_micros);
    for i in 1..=opts.number {
        publisher.send((i, timestamp()), recipients.iter()).unwrap();
        std::thread::sleep(timeout);
    }
}
