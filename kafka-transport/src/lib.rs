#![feature(async_await)]
#![allow(dead_code, unused_variables)]
#![allow(clippy::needless_lifetimes)]

use bytes::{BigEndian, Buf, ByteOrder, BytesMut, IntoBuf};
use futures::prelude::*;
use kafka_api::{KafkaRequest, KafkaResponse, KafkaResponseBody, KafkaRpcType};

use crate::error::TransportError;
use kafka_api::apikey::ApiKeys;
use std::marker::PhantomData;

mod error;
pub mod frame;

/// Represents an in-flight request to a Kafka broker.
/// The Kafka protocol does not specify a return type for messages, as they are expected to match  
/// the message sent, and to be handled in order.
#[derive(Debug, Copy, Clone)]
struct CorrelationMarker {
    api_key: ApiKeys,
    api_version: i16,
}

/// Handles communication with a Kafka broker.
pub struct ClientTransport {
    reader: Box<dyn Stream<Item = Result<BytesMut, std::io::Error>> + Unpin>,
    writer: Box<dyn AsyncWrite + Unpin>,
}

pub struct Transport<In, Out> {
    reader: Box<dyn Stream<Item = Result<BytesMut, std::io::Error>> + Unpin>,
    writer: Box<dyn AsyncWrite + Unpin>,
    _out_marker: PhantomData<In>,
    _in_marker: PhantomData<Out>,
}

impl<In, Out> Transport<In, Out> {
    pub fn new<T>(inner: T) -> Self
    where
        T: AsyncRead + AsyncWrite + Sized + 'static,
    {
        let (r, write) = inner.split();
        let reader = Box::new(frame::FramedRead::new(r));
        let writer = Box::new(write);
        Self {
            reader,
            writer,
            _out_marker: PhantomData,
            _in_marker: PhantomData,
        }
    }
}

impl<B: KafkaResponseBody + Sized> Transport<KafkaRequest, KafkaResponse<B>> {
    async fn write_response(
        &mut self,
        response: &KafkaResponse<B>,
    ) -> Result<(), error::TransportError> {
        let response_size: usize = response.size();
        let mut buf: Vec<u8> = Vec::with_capacity(response_size + 4);
        unsafe { buf.set_len(response_size + 4) };
        BigEndian::write_u32(&mut buf, response_size as u32);
        response
            .write(&mut buf[4..])
            .map_err(error::TransportError::Codec)?;
        self.writer
            .write_all(&buf)
            .await
            .map_err(error::TransportError::Io)
    }
    async fn read_request(&mut self) -> Result<KafkaRequest, error::TransportError> {
        KafkaRequest::read(
            self.reader
                .next()
                .await
                .ok_or_else(TransportError::broken_pipe)?
                .map_err(TransportError::Io)?
                .freeze()
                .into_buf()
                .reader(),
        )
        .map_err(error::TransportError::Codec)
    }
}

impl<B: KafkaResponseBody + Sized> Transport<KafkaResponse<B>, KafkaRequest> {
    async fn read_response(
        &mut self,
        api_version: i16,
    ) -> Result<KafkaResponse<B>, error::TransportError> {
        let response_buf: BytesMut = self
            .reader
            .next()
            .await
            .ok_or_else(TransportError::broken_pipe)?
            .map_err(TransportError::Io)?;
        KafkaResponse::read(response_buf.freeze().into_buf().reader(), api_version)
            .map_err(error::TransportError::Codec)
    }
}

impl ClientTransport {
    pub fn new<T>(inner: T) -> Self
    where
        T: AsyncRead + AsyncWrite + Sized + 'static,
    {
        let (r, write) = inner.split();
        let reader = Box::new(frame::FramedRead::new(r));
        let writer = Box::new(write);
        Self { reader, writer }
    }
    async fn write_request(
        &mut self,
        request: &kafka_api::KafkaRequest,
    ) -> Result<(), error::TransportError> {
        let request_size = request.size();
        let mut buf: Vec<u8> = Vec::with_capacity(request_size + 4);
        unsafe { buf.set_len(request_size + 4) };
        BigEndian::write_u32(&mut buf, request_size as u32);
        request
            .write(&mut buf[4..])
            .map_err(error::TransportError::Codec)?;
        self.writer
            .write_all(&buf)
            .await
            .map_err(error::TransportError::Io)
    }
    async fn read_response<B: KafkaResponseBody>(
        &mut self,
        api_version: i16,
    ) -> Result<kafka_api::KafkaResponse<B>, error::TransportError> {
        let response_buf: BytesMut = self
            .reader
            .next()
            .await
            .ok_or_else(TransportError::broken_pipe)?
            .map_err(TransportError::Io)?;
        let response_buf_reader = response_buf.freeze().into_buf().reader();
        let mut ctx = kafka_protocol::DeserializeCtx::new(response_buf_reader, api_version);
        let correlation_id = i32::read(&mut ctx).map_err(TransportError::Codec)?;
        let body: B = B::read(&mut ctx).map_err(TransportError::Codec)?;
        Ok(kafka_api::KafkaResponse::new(
            api_version,
            correlation_id,
            body,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::{BigEndian, ByteOrder};

    #[test]
    fn successful_read() {
        let response = kafka_api::KafkaResponse::new(
            0,
            0,
            kafka_api::api::ApiVersionsResponse {
                error_code: kafka_api::errors::KafkaCode::None.key(),
                api_keys: vec![],
                throttle_time_ms: 10,
            },
        );
        let response_size = response.size();
        let mut buf: Vec<u8> = vec![0; response_size + 4];

        BigEndian::write_u32(&mut buf, response_size as u32);
        response.write(&mut buf).unwrap();
        let cursor = std::io::Cursor::new(buf);
        let socket = futures::io::AllowStdIo::new(cursor);
        let mut transport = ClientTransport::new(socket);
        let result: kafka_api::KafkaResponse<kafka_api::api::ApiVersionsResponse> =
            futures::executor::block_on(transport.read_response(0)).unwrap();
        assert_eq!(response.body.error_code, result.body.error_code);
    }
}

mod mock {}
