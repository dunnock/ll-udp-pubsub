use std::{net::SocketAddr, time::Duration};

use clap::Parser;
use ll_udp_pubsub::publisher_controller::{PublisherController, PublisherControllerConfig};

#[derive(clap::Parser)]
/// Send counter with specified timeout
struct Cmd {
    /// Address to bind publisher to
    #[clap(short = 's')]
    server_addr: SocketAddr,
    /// Timeout to send messages in microseconds
    #[clap(short = 't', default_value = "1000")]
    timeout_micros: u64,
    /// Non-blocking socket for publisher
    #[clap(long = "non-blocking")]
    non_blocking: bool,
}

fn main() {
    env_logger::try_init().ok();
    let opts = Cmd::parse();
    let config = PublisherControllerConfig {
        subscription_timeout: Duration::from_secs(30),
        control_timeout: Duration::from_secs(5),
        non_blocking: opts.non_blocking,
        addr: opts.server_addr,
    };
    let (ctl, mut publisher) = PublisherController::create(config).unwrap();
    let handle = ctl.spawn().unwrap();

    let timeout = Duration::from_micros(opts.timeout_micros);
    for i in 0..i64::MAX {
        publisher.send(i).unwrap();
        std::thread::sleep(timeout);
    }

    handle.shutdown().unwrap();
}
