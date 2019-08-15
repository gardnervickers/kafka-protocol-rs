use crate::media::{
    mem::{MemoryListener, MemoryMedia, MemorySocket},
    TransportMedia,
};

use futures::{ready, Poll, Stream};
use std::io;
use std::net::SocketAddr;
use std::pin::Pin;
use std::task::Context;

impl TransportMedia for MemoryMedia {
    type Output = MemorySocket;
    type Error = std::io::Error;
    type Listener = Listener;
    type Inbound = futures::future::Ready<Result<Self::Output, Self::Error>>;
    type Outbound = futures::future::Ready<Result<Self::Output, Self::Error>>;
    fn bind<A: Into<SocketAddr>>(&self, addr: A) -> Result<Self::Listener, Self::Error>
    where
        Self: Sized,
    {
        unimplemented!()
    }
    fn dial<A: Into<SocketAddr>>(&self, addr: A) -> Result<Self::Outbound, Self::Error>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}

pub struct Listener {
    inner: MemoryListener,
}

impl Stream for Listener {
    type Item = Result<futures::future::Ready<Result<MemorySocket, io::Error>>, io::Error>;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut incoming = self.inner.incoming();
        let sock = ready!(Pin::new(&mut incoming).poll_next(cx));
        Poll::Ready(sock.map(|s| Ok(futures::future::ready(s))))
    }
}
