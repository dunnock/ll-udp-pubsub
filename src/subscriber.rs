use std::{
    any::Any,
    io::ErrorKind,
    net::{SocketAddr, UdpSocket},
    sync::atomic,
    sync::{atomic::AtomicBool, Arc},
    thread::JoinHandle,
    time::Duration,
};

use crate::{timestamp, Handler, Packet, MTU};

const DEFAULT_READ_TIMEOUT: Duration = Duration::from_millis(1_000);

pub struct UdpSubscriberConfig {
    pub client_addr: SocketAddr,
    pub read_timeout: Duration,
    /// Drop events which came out of sequence
    pub drop_out_of_sequence: bool,
}

impl UdpSubscriberConfig {
    pub fn new(client_addr: SocketAddr) -> Self {
        Self {
            client_addr,
            read_timeout: DEFAULT_READ_TIMEOUT,
            drop_out_of_sequence: false,
        }
    }
}

pub struct UdpSubscriber<MessageHandler> {
    sock: UdpSocket,
    config: UdpSubscriberConfig,
    shutdown_flag: Arc<AtomicBool>,
    handler: MessageHandler,
}

pub struct UdpSubscriberHandle<MessageHandler> {
    handle: JoinHandle<MessageHandler>,
    shutdown: Arc<AtomicBool>,
    sock: UdpSocket,
}

impl<MessageHandler> UdpSubscriberHandle<MessageHandler> {
    pub fn join(self) -> Result<MessageHandler, Box<dyn Any + Send + 'static>> {
        self.handle.join()
    }
    pub fn shutdown(self) -> Result<MessageHandler, Box<dyn Any + Send + 'static>> {
        self.shutdown.store(true, atomic::Ordering::Relaxed);
        self.join()
    }
    pub fn socket(&self) -> &UdpSocket {
        &self.sock
    }
}

impl<MessageHandler: Handler> UdpSubscriber<MessageHandler> {
    pub fn new(
        config: UdpSubscriberConfig,
        handler: MessageHandler,
    ) -> Result<Self, std::io::Error> {
        let client_addr = config.client_addr;
        let sock = UdpSocket::bind(client_addr)?;
        Ok(Self {
            config,
            sock: sock.try_clone()?,
            shutdown_flag: Arc::default(),
            handler,
        })
    }

    pub fn set_nonblocking(&mut self, nonblocking: bool) -> Result<(), std::io::Error> {
        self.sock.set_nonblocking(nonblocking)
    }

    pub fn run(mut self) -> MessageHandler {
        // Blocking way of polling socket
        assert!(!self.config.read_timeout.is_zero());
        self.sock
            .set_read_timeout(Some(self.config.read_timeout))
            .unwrap();

        let mut buf = [0u8; MTU];
        let last_server_ts = 0;
        loop {
            match self.sock.recv(&mut buf) {
                Ok(len) => {
                    let ts = timestamp();
                    let msg: Packet<MessageHandler::Message> =
                        bincode::deserialize(&buf[..len]).unwrap();
                    if self.config.drop_out_of_sequence && last_server_ts > msg.sent_ts {
                        continue;
                    }
                    // Pass message packet to handler
                    self.handler.handle(msg, ts);
                }
                Err(err)
                    if err.kind() == ErrorKind::WouldBlock || err.kind() == ErrorKind::TimedOut => {
                }
                Err(err) => todo!("Receive failed: {err}"),
            }

            if self.shutdown_flag.load(atomic::Ordering::Relaxed) {
                break;
            }

            #[cfg(feature="cooperative_waiting")]
            std::thread::yield_now();
            #[cfg(not(feature="cooperative_waiting"))]
            std::hint::spin_loop();
        }

        self.handler
    }
}

impl<MessageHandler: Handler + Send + 'static> UdpSubscriber<MessageHandler> {
    pub fn spawn(self) -> Result<UdpSubscriberHandle<MessageHandler>, std::io::Error> {
        let shutdown = self.shutdown_flag.clone();
        let sock = self.sock.try_clone().unwrap();

        let handle = std::thread::Builder::new()
            .name(format!("UDP subscriber {}", self.config.client_addr))
            .spawn(move || self.run())?;

        Ok(UdpSubscriberHandle {
            handle,
            shutdown,
            sock,
        })
    }
}
