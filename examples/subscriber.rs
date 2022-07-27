use std::{
    net::SocketAddr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
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
}

struct Msg {
    id: i64,
    sent_ts: i64,
    received_ts: i64,
}

#[derive(Default, Clone)]
struct Receiver {
    messages: Arc<Mutex<Vec<Msg>>>,
}

impl Handler for Receiver {
    type Message = i64;

    fn handle(&mut self, msg: Packet<Self::Message>, received_ts: i64) {
        self.messages.lock().unwrap().push(Msg {
            id: msg.data,
            sent_ts: msg.sent_ts,
            received_ts,
        })
    }
}

fn main() {
    let opts = Cmd::parse();
    let subscriber_config = UdpSubscriberConfig::new(opts.client_addr);
    let receiver = Receiver::default();
    let mut subscriber = UdpSubscriber::new(subscriber_config, receiver.clone()).unwrap();
    subscriber.set_nonblocking(opts.non_blocking).unwrap();
    let subscriber_handle = subscriber.spawn().unwrap();

    // Messy implementation of controller for subscriber, just for illustration purposes
    let shutdown_controller = Arc::new(AtomicBool::default());
    let shutdown = shutdown_controller.clone();
    let server_socket = subscriber_handle.socket().try_clone().unwrap();
    let controller_handle = std::thread::spawn(move || {
        if let Err(err) = server_socket.connect(opts.server_addr) {
            shutdown.store(true, Ordering::Relaxed);
            log::error!("Failed to connect to {} {err}", opts.server_addr);
        }
        let subscribe = bincode::serialize(&ControlMessage::Subscribe).unwrap();
        while !shutdown.load(Ordering::Relaxed) {
            if let Err(err) = server_socket.send(&subscribe) {
                shutdown.store(true, Ordering::Relaxed);
                log::error!("Failed to receive message {err}");
            }
            std::thread::sleep(Duration::from_secs(15));
        }
    });

    // Wait until client receives expected number of messages
    while receiver.messages.lock().unwrap().len() < opts.number {
        if shutdown_controller.load(std::sync::atomic::Ordering::Relaxed) {
            break;
        }
        std::thread::sleep(Duration::from_millis(1_000));
    }

    // Stop subscriber and controller
    shutdown_controller.store(true, Ordering::Relaxed);
    subscriber_handle.shutdown().unwrap();
    controller_handle.join().unwrap();

    // Print out csv with results
    println!("id,sent_ts,received_ts");
    for Msg {
        id,
        sent_ts,
        received_ts,
    } in receiver.messages.lock().unwrap().iter()
    {
        println!("{id},{sent_ts},{received_ts}");
    }
}
