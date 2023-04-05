use std::{
    io::ErrorKind,
    net::{SocketAddr, UdpSocket},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

use scc::ebr::Barrier;
use serde::Serialize;

use crate::{
    publisher::UdpPublisher, timestamp, ChannelMetrics, ControlMessage, ControllerHandle,
    Recipients,
};

pub struct PublisherControllerConfig {
    pub subscription_timeout: Duration,
    pub control_timeout: Duration,
    pub non_blocking: bool,
    pub addr: SocketAddr,
}

pub struct ManagedPublisher {
    publisher: UdpPublisher,
    recipients: Recipients,
}

pub struct PublisherController {
    recipients: Arc<scc::HashIndex<SocketAddr, ChannelMetrics>>,
    config: PublisherControllerConfig,
    sock: UdpSocket,
}

impl ManagedPublisher {
    pub fn send<'r, Message: Serialize>(&mut self, msg: Message) -> Result<(), std::io::Error> {
        let barrier = Barrier::new();
        self.publisher
            .send(msg, self.recipients.iter(&barrier).map(|(addr, _)| addr))
    }
}

impl PublisherController {
    pub fn create(
        config: PublisherControllerConfig,
    ) -> Result<(Self, ManagedPublisher), std::io::Error> {
        let recipients: Recipients = Default::default();
        let publisher = UdpPublisher::new(config.addr)?;
        Ok((
            Self {
                config,
                recipients: recipients.clone(),
                sock: publisher.sock().try_clone()?,
            },
            ManagedPublisher {
                publisher,
                recipients,
            },
        ))
    }

    pub fn spawn(self) -> Result<ControllerHandle, std::io::Error> {
        let shutdown: Arc<AtomicBool> = Default::default();
        let shutdown_copy = shutdown.clone();

        let handle = std::thread::Builder::new()
            .name(format!("UDP subscriber {}", self.config.addr))
            .spawn(move || self.run(shutdown_copy))?;

        Ok(ControllerHandle { handle, shutdown })
    }

    pub fn run(&self, stop: Arc<AtomicBool>) {
        self.sock
            .set_read_timeout(Some(self.config.control_timeout))
            .unwrap();
        let control_timeout_ns = self.config.control_timeout.as_nanos() as i64;
        log::info!(
            "Started publisher control at {}",
            self.sock.local_addr().unwrap()
        );
        let mut buf = [0u8; 1024];
        let mut last_subscriptions_check = timestamp();
        loop {
            match self.sock.recv_from(&mut buf).map(|(size, addr)| {
                (
                    bincode::deserialize(&buf[..size]).expect("Failed to parse control message"),
                    addr,
                )
            }) {
                Ok((ControlMessage::Subscribe, addr)) => {
                    let ts = timestamp();
                    let last_submit = if let Some(last_submit) = self
                        .recipients
                        .read(&addr, |_, metrics| metrics.last_submit.clone())
                    {
                        log::debug!("client={addr} subscription confirmed");
                        last_submit
                    } else {
                        log::info!("client={addr} subscribed");
                        let metrics = ChannelMetrics::default();
                        self.recipients.insert(addr, metrics.clone()).unwrap();
                        metrics.last_submit
                    };
                    // We can store with Relaxed since we are reading later in the same thread
                    last_submit.store(ts, Ordering::Relaxed);
                    self.expire_subscriptions(ts);
                }
                Ok((ControlMessage::Unsubscribe, addr)) => {
                    if let Some(metrics) = self.recipients.read(&addr, |_, v| v.clone()) {
                        self.recipients.remove(&addr);
                        log::info!("client={addr} unsubscribed, {metrics:?}");
                    }
                }
                // NOTE: as noted in the doc return code might vary on diffrent systems
                Err(err)
                    if err.kind() == ErrorKind::WouldBlock || err.kind() == ErrorKind::TimedOut =>
                {
                    let ts = timestamp();
                    if ts - last_subscriptions_check > control_timeout_ns {
                        self.expire_subscriptions(ts);
                        last_subscriptions_check = ts;
                        if stop.load(Ordering::Relaxed) {
                            log::info!("Shutting down udp publisher control thread");
                            return;
                        }
                    } else if self.config.non_blocking {
                        std::thread::sleep(self.config.control_timeout);
                    }
                }
                Err(err) => {
                    log::error!("Error processing socket {err}");
                }
            }
        }
    }

    fn expire_subscriptions(&self, ts: i64) {
        let barrier = Barrier::new();
        let subscription_timeout_ns = self.config.subscription_timeout.as_nanos() as i64;
        for (addr, metrics) in self.recipients.iter(&barrier) {
            let client_timeout = ts - metrics.last_submit.load(Ordering::Relaxed);
            if client_timeout > subscription_timeout_ns {
                self.recipients.remove(addr);
                log::info!("client={addr}, timeout={client_timeout} unsubscribed with timeout, {metrics:?}");
            }
        }
    }
}
