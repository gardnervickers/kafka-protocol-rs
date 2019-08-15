//! In-memory transport with optional failure injection.
use bytes::{Buf, Bytes, IntoBuf};
use futures::{
    channel::mpsc::{self, UnboundedReceiver, UnboundedSender},
    io,
    io::{Error, ErrorKind},
    lock::Mutex,
    ready,
    stream::{FusedStream, Stream},
    task::{Context, Poll},
    AsyncRead, AsyncWrite,
};
use std::{
    net::{Ipv4Addr, SocketAddr},
    num::NonZeroU16,
    pin::Pin,
};

mod hub;
mod transport;

enum FailureStrategy {
    None,
}

/// In-memory transport media, useful for testing only.
pub struct MemoryMedia {
    failure_strategy: FailureStrategy,
    connections: Mutex<hub::ConnectionMapping>,
}

impl MemoryMedia {
    pub fn new() -> Self {
        Self {
            failure_strategy: FailureStrategy::None,
            connections: Mutex::new(hub::ConnectionMapping::new()),
        }
    }
    pub async fn connect(&mut self, port: u16) -> Result<MemorySocket, Error> {
        let mut connhub = self.connections.lock().await;
        let port = NonZeroU16::new(port).ok_or::<Error>(ErrorKind::AddrNotAvailable.into())?;
        let sender = connhub.get_sender(port)?;
        let (socket_a, socket_b) = MemorySocket::new_pair();
        sender.unbounded_send(socket_a).map_err(|e| {
            if e.is_disconnected() {
                ErrorKind::AddrNotAvailable.into()
            } else {
                ErrorKind::NotFound
            }
        })?;
        Ok(socket_b)
    }

    pub async fn bind(&self, port: u16) -> Result<MemoryListener, Error> {
        let mut connhub = self.connections.lock().await;
        let port = connhub.free_port(port)?;
        let (tx, rx) = mpsc::unbounded();
        connhub.register_sender(port, tx);
        Ok(MemoryListener { incoming: rx, port })
    }
}

#[derive(Debug)]
pub struct Incoming<'a> {
    inner: &'a mut MemoryListener,
}
#[derive(Debug)]
pub struct MemoryListener {
    incoming: UnboundedReceiver<MemorySocket>,
    port: NonZeroU16,
}

impl MemoryListener {
    pub fn local_addr(&self) -> u16 {
        self.port.get()
    }
    pub fn incoming(&mut self) -> Incoming<'_> {
        Incoming { inner: self }
    }
    fn poll_accept(&mut self, cx: &mut Context) -> Poll<Result<MemorySocket, Error>> {
        match Pin::new(&mut self.incoming).poll_next(cx) {
            Poll::Ready(Some(sock)) => Poll::Ready(Ok(sock)),
            Poll::Ready(None) => Poll::Ready(Err(ErrorKind::BrokenPipe.into())),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl Stream for MemoryListener {
    type Item = io::Result<MemorySocket>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let mut incoming = self.incoming();
        Pin::new(&mut incoming).poll_next(cx)
    }
}

impl<'a> Stream for Incoming<'a> {
    type Item = Result<MemorySocket, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let sock = ready!(self.inner.poll_accept(cx)?);
        Poll::Ready(Some(Ok(sock)))
    }
}

#[derive(Debug)]
pub struct MemorySocket {
    incoming: UnboundedReceiver<Bytes>,
    outgoing: UnboundedSender<Bytes>,
    current: Option<<Bytes as IntoBuf>::Buf>,
    eof: bool,
    local_addr: Option<SocketAddr>,
    peer_addr: Option<SocketAddr>,
}

impl MemorySocket {
    fn new_pair_with_port(port: Option<NonZeroU16>) -> (Self, Self) {
        let (a_tx, a_rx) = mpsc::unbounded();
        let (b_tx, b_rx) = mpsc::unbounded();
        let addr = port.map(|p| SocketAddr::new(Ipv4Addr::LOCALHOST.into(), p.get()));
        let a = Self {
            incoming: a_rx,
            outgoing: b_tx,
            current: None,
            eof: false,
            local_addr: addr,
            peer_addr: addr,
        };
        let b = Self {
            incoming: b_rx,
            outgoing: a_tx,
            current: None,
            eof: false,
            local_addr: addr,
            peer_addr: addr,
        };
        (a, b)
    }
    pub fn new_pair() -> (Self, Self) {
        MemorySocket::new_pair_with_port(None)
    }
}

impl AsyncRead for MemorySocket {
    fn poll_read(
        mut self: Pin<&mut Self>,
        mut cx: &mut Context,
        buf: &mut [u8],
    ) -> Poll<Result<usize, Error>> {
        if self.incoming.is_terminated() {
            if self.eof {
                return Poll::Ready(Err(ErrorKind::UnexpectedEof.into()));
            } else {
                self.eof = true;
                return Poll::Ready(Ok(0));
            }
        }
        let mut bytes_read = 0;
        loop {
            if bytes_read == buf.len() {
                return Poll::Ready(Ok(bytes_read));
            }
            match self.current {
                Some(ref mut current_buffer) if current_buffer.has_remaining() => {
                    let bytes_to_read =
                        std::cmp::min(buf.len() - bytes_read, current_buffer.remaining());
                    Buf::take(current_buffer, bytes_to_read)
                        .copy_to_slice(&mut buf[bytes_read..(bytes_read + bytes_to_read)]);
                    bytes_read += bytes_to_read;
                }
                _ => {
                    self.current = {
                        match Pin::new(&mut self.incoming).poll_next(&mut cx) {
                            Poll::Pending => {
                                if bytes_read > 0 {
                                    return Poll::Ready(Ok(bytes_read));
                                } else {
                                    return Poll::Pending;
                                }
                            }
                            Poll::Ready(Some(buf)) => Some(buf.into_buf()),
                            Poll::Ready(None) => return Poll::Ready(Ok(bytes_read)),
                        }
                    }
                }
            }
        }
    }
}

impl AsyncWrite for MemorySocket {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &[u8],
    ) -> Poll<Result<usize, Error>> {
        let len = buf.len();
        match self.outgoing.poll_ready(cx) {
            Poll::Ready(Err(e)) => {
                if e.is_disconnected() {
                    return Poll::Ready(Err(ErrorKind::BrokenPipe.into()));
                } else {
                    unreachable!()
                }
            }
            Poll::Pending => return Poll::Pending,
            Poll::Ready(Ok(())) => {
                if let Err(e) = self.outgoing.start_send(buf.into()) {
                    if e.is_disconnected() {
                        return Poll::Ready(Err(ErrorKind::BrokenPipe.into()));
                    } else {
                        unreachable!()
                    }
                }
            }
        }
        Poll::Ready(Ok(len))
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Error>> {
        self.outgoing.close_channel();
        Poll::Ready(Ok(()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::{AsyncReadExt, AsyncWriteExt, StreamExt};
    use futures_test::future::FutureTestExt;
    use futures::executor::block_on;


    #[test]
    fn echo() {
        block_on(async {
            let mut media = MemoryMedia::new();

            let mut listener = media.bind(0).await.unwrap();
            let echo_port = listener.port;
            // Spawn a simple echo server which accepts a new connection and echos back the bytes.
            let echo_server = async move {
                //let mut futs = vec![];
                while let Some(conn) = listener.incoming.next().await {
                    let h = async move {
                        let (rh, mut wh) = conn.split();
                        rh.copy_into(&mut wh).await.unwrap();
                    };
                    h.run_in_background();
                    //futs.push(h);
                }
                //futures::future::join_all(futs).await;
            };
            echo_server.run_in_background();

            let mut conn = media.connect(echo_port.into()).await.unwrap();
            let data = String::from("Hello World!");
            conn.write_all(data.as_bytes()).await.unwrap();
            let mut read_target = vec![0; data.len()];
            conn.read_exact(&mut read_target).await.unwrap();
            let echo_result = String::from_utf8(read_target).unwrap();
            assert_eq!(data, echo_result);
        });
    }

}
