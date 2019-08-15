use crate::media::mem::MemorySocket;
use futures::channel::mpsc::UnboundedSender;
use futures::io::{Error, ErrorKind};
use std::collections::HashMap;
use std::num::NonZeroU16;

pub struct ConnectionMapping {
    bound_ports: HashMap<NonZeroU16, UnboundedSender<MemorySocket>>,
    next_port: u16,
}

impl ConnectionMapping {
    pub fn new() -> Self {
        Self {
            bound_ports: HashMap::new(),
            next_port: 1,
        }
    }
    fn gc_closed(&mut self) {
        self.bound_ports.retain(|_, v| !v.is_closed());
    }
    /// Check if the provided port is in use or not. If `port` is 0, assign a new
    /// port.
    pub(crate) fn free_port(&mut self, port: u16) -> Result<NonZeroU16, Error> {
        self.gc_closed();
        let port = if let Some(port) = NonZeroU16::new(port) {
            if self.bound_ports.contains_key(&port) {
                return Err(ErrorKind::AddrInUse.into());
            } else {
                return Ok(port);
            }
        } else {
            loop {
                let port = match NonZeroU16::new(self.next_port) {
                    Some(p) => p,
                    None => unreachable!(),
                };
                self.next_port += 1;
                if !self.bound_ports.contains_key(&port) {
                    break port;
                }
            }
        };
        Ok(port)
    }

    pub(crate) fn get_sender(
        &mut self,
        port: NonZeroU16,
    ) -> Result<&mut UnboundedSender<MemorySocket>, Error> {
        self.gc_closed();
        self.bound_ports
            .get_mut(&port)
            .ok_or::<Error>(ErrorKind::AddrNotAvailable.into())
    }

    pub(crate) fn register_sender(
        &mut self,
        port: NonZeroU16,
        send: UnboundedSender<MemorySocket>,
    ) {
        self.gc_closed();
        self.bound_ports.insert(port, send);
    }
}
