//! A [Stream] of framed [BytesMut]
//!
//! # Overview
//!
//! Provides a type that implements [Stream] which can use a [BigEndian] i32 prefix to demarcate
//! frames over a byte stream. This is the same size prefix which the Kafka Protocol uses for both
//! requests and responses.
//!
//! ```
//! use kafka_transport::frame::FramedRead;
//! use futures::{Stream, StreamExt};
//! use bytes::BytesMut;
//! # /*
//! #[test]
//! # */
//! fn test() {
//!   let data: &[u8] = &[0, 0, 0, 3, b'f', b'o', b'o'];
//!   let mut decoder = FramedRead::new(data);
//!   let result = futures::executor::block_on(decoder.map(Result::unwrap).next());
//!   assert_eq!(result.unwrap(), BytesMut::from(vec![b'f', b'o', b'o']));
//! }
//!
//! # pub fn main() {
//! # test();
//! # }
//! ```
//! [Stream]: futures::Stream
//! [BytesMut]: bytes::BytesMut
//! [BigEndian]: bytes::BigEndian

#![allow(dead_code)]
use bytes::{BigEndian, ByteOrder, BytesMut};
use core::{
    pin::Pin,
    task::{Context, Poll},
};
use futures::{prelude::*, ready};

use pin_project::unsafe_project;

const INITIAL_CAPACITY: usize = 8 * 1024;

enum FillState {
    Filled,
    Eof,
}

#[unsafe_project(Unpin)]
pub struct FramedRead<R: AsyncRead> {
    #[pin]
    inner: R,
    buffer: BytesMut,
}

impl<R> FramedRead<R>
where
    R: AsyncRead,
{
    /// Creates a new `FramedRead` with a default initial buffer capacity. The default is currently
    /// 8 KB but this is subject to change.
    pub fn new(inner: R) -> FramedRead<R> {
        Self {
            inner,
            buffer: BytesMut::with_capacity(INITIAL_CAPACITY),
        }
    }
    /// Unwraps the `FramedRead`, returning the underlying reader.
    ///
    /// Note that any leftover data in the internal buffer is lost
    pub fn into_inner(self) -> R {
        self.inner
    }

    fn fill(
        self: Pin<&mut Self>,
        cx: &mut Context,
        target_size: usize,
    ) -> Poll<Result<FillState, futures::io::Error>> {
        let this = self.project();
        let mut reader: Pin<&mut R> = this.inner;

        if this.buffer.len() >= target_size {
            return Poll::Ready(Ok(FillState::Filled));
        }

        if this.buffer.capacity() < target_size {
            let needed = target_size - this.buffer.capacity();
            this.buffer.reserve(needed);
        }

        let existing = this.buffer.len();
        let mut need_filled = this.buffer.split_off(existing);
        let max = need_filled.capacity();
        unsafe { need_filled.set_len(max) };

        // Repeatedly poll until the buffer is full
        while this.buffer.len() < target_size {
            let bytes_read: usize = ready!(reader.as_mut().poll_read(cx, &mut need_filled[..]))?;
            // If we read 0 bytes, either we hit EOF for some reason, or the connection was broken.
            if bytes_read == 0 {
                if this.buffer.is_empty() {
                    return Poll::Ready(Ok(FillState::Eof));
                } else {
                    return Poll::Ready(Err(futures::io::Error::from(
                        futures::io::ErrorKind::BrokenPipe,
                    )));
                }
            }
            // Rejoin the read data back into the buffer
            let read = need_filled.split_to(bytes_read);
            this.buffer.unsplit(read);
        }
        Poll::Ready(Ok(FillState::Filled))
    }
}

impl<R> Stream for FramedRead<R>
where
    R: AsyncRead,
{
    type Item = Result<BytesMut, futures::io::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        match ready!(self.as_mut().fill(cx, 5)) {
            Err(e) => Poll::Ready(Some(Err(e))),
            Ok(FillState::Eof) => Poll::Ready(None),
            Ok(FillState::Filled) => {
                let message_size = BigEndian::read_i32(&self.buffer[..4]);
                match ready!(self.as_mut().fill(cx, message_size as usize + 4)) {
                    Err(e) => Poll::Ready(Some(Err(e))),
                    Ok(FillState::Eof) => Poll::Ready(None),
                    Ok(FillState::Filled) => {
                        let this = self.project();
                        let buffer: &mut BytesMut = this.buffer;
                        buffer.advance(4);
                        let buf = buffer.split_to(message_size as usize);
                        Poll::Ready(Some(Ok(buf)))
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures_test::io::AsyncReadTestExt;
    use futures_test::{assert_stream_done, assert_stream_next, assert_stream_pending};

    #[test]
    fn empty_read() {
        // Expected stream to be done when we hit EOF
        let data: &[u8] = &[];
        let mut decoder = FramedRead::new(data);
        assert_stream_done!(decoder);
    }

    #[test]
    fn not_enough_data() {
        // Expected stream to be done when we hit EOF, throwing an error because some data was read
        let data: &[u8] = &[0, 0, 0];
        let decoder = FramedRead::new(data);
        assert_stream_next!(decoder.map(|v| v.is_err()), true);
    }

    #[test]
    fn framed_read() {
        let data: &[u8] = &[0, 0, 0, 3, b'f', b'o', b'o'];
        let decoder = FramedRead::new(data);
        assert_stream_next!(
            decoder.map(Result::unwrap),
            BytesMut::from(vec![b'f', b'o', b'o'])
        );
    }

    #[test]
    fn zero_sized_read() {
        let data: &[u8] = &[0, 0, 0, 0];
        let decoder = FramedRead::new(data);
        assert_stream_next!(decoder.map(|v| v.is_err()), true);
    }

    #[test]
    fn oversized_read() {
        let data: &[u8] = &[0, 0, 0, 3, b'f'];
        let decoder = FramedRead::new(data);
        assert_stream_next!(decoder.map(|v| v.is_err()), true);
    }

    #[test]
    fn multiple_reads() {
        let data: &[u8] = &[
            0, 0, 0, 1, b'f', 0, 0, 0, 1, b'o', 0, 0, 0, 3, b'f', b'o', b'o',
        ];
        let mut decoder = Box::pin(FramedRead::new(data));
        assert_stream_next!(
            decoder.as_mut().map(Result::unwrap),
            BytesMut::from(vec![b'f'])
        );

        assert_stream_next!(
            decoder.as_mut().map(Result::unwrap),
            BytesMut::from(vec![b'o'])
        );

        assert_stream_next!(
            decoder.as_mut().map(Result::unwrap),
            BytesMut::from(vec![b'f', b'o', b'o'])
        );
    }

    #[test]
    fn pending_propagation() {
        let data: &[u8] = &[0, 0, 0, 3, b'f', b'o', b'o'];
        let reader = data.interleave_pending();
        let mut decoder = Box::pin(FramedRead::new(reader));

        assert_stream_pending!(decoder.as_mut());
        assert_stream_next!(
            decoder.as_mut().map(Result::unwrap),
            BytesMut::from(vec![b'f', b'o', b'o'])
        );
        assert_stream_pending!(decoder.as_mut());
        assert_stream_done!(decoder.as_mut());
    }
}
