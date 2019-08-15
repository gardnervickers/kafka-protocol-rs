use futures::{Future, Stream};
use std::net::SocketAddr;

pub mod mem;

pub trait TransportMedia {
    type Output;
    type Error: std::error::Error + Send + Sync + 'static;
    type Listener: Stream<Item = Result<Self::Inbound, Self::Error>> + Send + Unpin;
    type Inbound: Future<Output = Result<Self::Output, Self::Error>> + Send;
    type Outbound: Future<Output = Result<Self::Output, Self::Error>> + Send;

    fn bind<A: Into<SocketAddr>>(&self, addr: A) -> Result<Self::Listener, Self::Error>
    where
        Self: Sized;
    fn dial<A: Into<SocketAddr>>(&self, addr: A) -> Result<Self::Outbound, Self::Error>
    where
        Self: Sized;
}
