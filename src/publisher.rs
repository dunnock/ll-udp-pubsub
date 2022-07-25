use bytes::BufMut;
use serde::Serialize;
use std::net::{SocketAddr, UdpSocket};

use crate::MTU;

///
/// Send serializable binary message to multiple recipients
///
pub struct UdpPublisher {
    sock: UdpSocket,
    buf: [u8; MTU],
}

impl UdpPublisher {
    pub fn new(addr: SocketAddr) -> Result<Self, std::io::Error> {
        Ok(Self {
            sock: UdpSocket::bind(addr)?,
            buf: [0; MTU],
        })
    }

    pub fn set_nonblocking(&mut self, nonblocking: bool) -> Result<(), std::io::Error> {
        self.sock.set_nonblocking(nonblocking)
    }

    ///
    /// Serialize and Send messages
    ///
    /// Panicking if serialized message is bigger than MTU
    /// Panicking if socket was only able to send message partially
    ///
    pub fn send<'r, Message: Serialize, Recipients: Iterator<Item = &'r SocketAddr>>(
        &mut self,
        msg: Message,
        recipients: Recipients,
    ) -> Result<(), std::io::Error> {
        let mut writer = self.buf.writer();
        bincode::serialize_into(&mut writer, &msg).unwrap();
        let len = MTU - writer.get_ref().len();
        for addr in recipients {
            let sent = self.sock.send_to(&self.buf[..len], addr)?;
            assert_eq!(sent, len, "Expected whole packet to be sent");
        }
        Ok(())
    }
}
