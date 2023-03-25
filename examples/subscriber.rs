use std::{
    net::SocketAddr,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

use clap::Parser;
use ll_udp_pubsub::{
    subscriber::{UdpSubscriber, UdpSubscriberConfig},
    ControlMessage, Handler, Packet,
};

#[derive(clap::Parser)]
/// Receive counters with specified timeout
struct Cmd {
    /// Address where publisher controller is listening
    #[clap(short = 's')]
    server_addr: SocketAddr,
    /// Address where client is listening
    #[clap(short = 'c')]
    client_addr: SocketAddr,
    /// Number of messages to receive
    #[clap(short = 'n', default_value = "100")]
    number: usize,
    /// Non-blocking receive
    #[clap(long = "non-blocking")]
    non_blocking: bool,
    /// Pin subscriber to core
    #[arg(long = "core", env = "SUBSCRIBER_CORE")]
    core: Option<usize>,
}

struct Msg {
    id: i64,
    sent_ts: i64,
    received_ts: i64,
}

#[derive(Default)]
struct Receiver {
    messages: Vec<Msg>,
    count: Arc<AtomicUsize>,
}

impl Handler for Receiver {
    type Message = i64;

    fn handle(&mut self, msg: Packet<Self::Message>, received_ts: i64) {
        self.count.fetch_add(1, Ordering::Relaxed);
        self.messages.push(Msg {
            id: msg.data,
            sent_ts: msg.sent_ts,
            received_ts,
        })
    }
}

fn main() {
    env_logger::try_init().ok();
    let opts = Cmd::parse();
    let subscriber_config = UdpSubscriberConfig::new(opts.client_addr);
    let receiver = Receiver::default();
    let messages_count = receiver.count.clone();
    let mut subscriber = UdpSubscriber::new(subscriber_config, receiver).unwrap();
    subscriber.set_nonblocking(opts.non_blocking).unwrap();
    let controller_handle = subscriber.spawn_controller(opts.server_addr).unwrap();
    let subscriber_handle = subscriber.spawn(opts.core).unwrap();

    // Wait until client receives expected number of messages
    while messages_count.load(Ordering::Relaxed) < opts.number {
        log::debug!(
            "Received {} messages",
            messages_count.load(Ordering::Relaxed)
        );
        if shutdown_controller.load(std::sync::atomic::Ordering::Relaxed) {
            break;
        }
        std::thread::sleep(Duration::from_millis(1000));
    }

    // Stop subscriber and controller
    shutdown_controller.store(true, Ordering::Relaxed);
    let result = subscriber_handle.shutdown().unwrap();
    log::info!("Subscriber was shutdown");
    controller_handle.join().unwrap();
    log::info!("Controller was shutdown");

    // Print out csv with results
    println!("id,sent_ts,received_ts");
    for Msg {
        id,
        sent_ts,
        received_ts,
    } in result.messages
    {
        println!("{id},{sent_ts},{received_ts}");
    }
}
