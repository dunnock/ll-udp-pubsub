use std::{
    any::Any,
    io::ErrorKind,
    net::{SocketAddr, UdpSocket},
    sync::atomic,
    sync::{atomic::AtomicBool, Arc},
    thread::JoinHandle,
    time::Duration, hint,
};

use crate::{timestamp, Handler, MTU};

const DEFAULT_READ_TIMEOUT: Duration = Duration::from_millis(1_000);

pub struct UdpSubscriberConfig {
    pub client_addr: SocketAddr,
    pub read_timeout: Duration,
}

impl UdpSubscriberConfig {
    pub fn new(client_addr: SocketAddr) -> Self {
        Self {
            client_addr,
            read_timeout: DEFAULT_READ_TIMEOUT,
        }
    }
}

pub struct UdpSubscriber<MessageHandler> {
    sock: UdpSocket,
    config: UdpSubscriberConfig,
    shutdown_flag: Arc<AtomicBool>,
    handler: MessageHandler,
}

pub struct UdpSubscriberHandle {
    handle: JoinHandle<()>,
    shutdown: Arc<AtomicBool>,
    sock: UdpSocket,
}

impl UdpSubscriberHandle {
    pub fn join(self) -> Result<(), Box<dyn Any + Send + 'static>> {
        self.handle.join()
    }
    pub fn shutdown(self) -> Result<(), Box<dyn Any + Send + 'static>> {
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

    pub fn run(mut self) {
        // Blocking way of polling socket
        assert!(!self.config.read_timeout.is_zero());
        self.sock
            .set_read_timeout(Some(self.config.read_timeout))
            .unwrap();

        let mut buf = [0u8; MTU];
        loop {
            match self.sock.recv(&mut buf) {
                Ok(len) => {
                    let ts = timestamp();
                    let msg: MessageHandler::Message = bincode::deserialize(&buf[..len]).unwrap();
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

            hint::spin_loop();
        }
    }
}

impl<MessageHandler: Handler + Send + 'static> UdpSubscriber<MessageHandler> {
    pub fn spawn(self) -> Result<UdpSubscriberHandle, std::io::Error> {
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
