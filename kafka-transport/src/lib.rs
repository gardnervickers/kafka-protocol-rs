#![recursion_limit = "128"]
#![feature(async_await)]
#![allow(dead_code, unused_variables)]
#![allow(clippy::needless_lifetimes)]

use bytes::{BigEndian, Buf, ByteOrder, BytesMut, IntoBuf};
use futures::prelude::*;
use kafka_api::{KafkaRequest, KafkaResponse};

use crate::error::TransportError;
use kafka_api::apikey::ApiKeys;
use std::marker::PhantomData;

mod error;
pub mod frame;

pub struct Transport<In, Out> {
    reader: Box<dyn Stream<Item = Result<BytesMut, std::io::Error>> + Unpin + Sync + Send>,
    writer: Box<dyn AsyncWrite + Unpin + Sync + Send>,
    _out_marker: PhantomData<In>,
    _in_marker: PhantomData<Out>,
}

pub fn new_client_transport<T>(inner: T) -> Transport<KafkaResponse, KafkaRequest>
where
    T: AsyncRead + AsyncWrite + Sized + Send + 'static,
{
    Transport::new(inner)
}

pub fn new_server_transport<T>(inner: T) -> Transport<KafkaRequest, KafkaResponse>
where
    T: AsyncRead + AsyncWrite + Sized + Send + 'static,
{
    Transport::new(inner)
}

impl<In, Out> Transport<In, Out> {
    pub fn new<T>(inner: T) -> Self
    where
        T: AsyncRead + AsyncWrite + Sized + Send + 'static,
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

impl Transport<KafkaRequest, KafkaResponse> {
    async fn write_response(
        &mut self,
        response: KafkaResponse,
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
        let framed_read: BytesMut = self
            .reader
            .next()
            .await
            .ok_or_else(TransportError::broken_pipe)?
            .map_err(error::TransportError::Io)?;

        KafkaRequest::read(framed_read.freeze().into_buf().reader())
            .map_err(error::TransportError::Codec)
    }
}

impl Transport<KafkaResponse, KafkaRequest> {
    async fn read_response(
        &mut self,
        api_key: ApiKeys,
        api_version: i16,
    ) -> Result<KafkaResponse, error::TransportError> {
        let response_buf: BytesMut = self
            .reader
            .next()
            .await
            .ok_or_else(TransportError::broken_pipe)?
            .map_err(TransportError::Io)?;
        KafkaResponse::read(
            response_buf.freeze().into_buf().reader(),
            api_key,
            api_version,
        )
        .map_err(error::TransportError::Codec)
    }

    async fn write_request(&mut self, request: KafkaRequest) -> Result<(), error::TransportError> {
        let request_size: usize = request.size();
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
}

#[cfg(test)]
mod tests {
    use super::*;

    use kafka_api::KafkaRequest;
    use kafka_api::KafkaResponse;

    use futures::select;
    use kafka_api::api::{
        MetadataRequest, MetadataRequestTopic, MetadataResponse, MetadataResponseBroker,
        MetadataResponsePartition, MetadataResponseTopic, RequestBody, ResponseBody,
    };
    use memsocket::MemorySocket;
    use runtime;

    /// Create a mock metadata server which responds to metadata requests with a fixed metadata response.
    async fn metadata_server(
        mut transport: Transport<KafkaRequest, KafkaResponse>,
        shutdown: futures::channel::oneshot::Receiver<()>,
    ) {
        let mut cancel = shutdown.fuse();

        loop {
            let mut handle = async {
                let request = transport.read_request().await.unwrap();
                assert_eq!(request.api_key, ApiKeys::Metadata);
                let body = ResponseBody::MetadataResponse(MetadataResponse {
                    throttle_time_ms: 20,
                    brokers: vec![MetadataResponseBroker {
                        node_id: 0,
                        host: String::from("somehost"),
                        port: 4242,
                        rack: None,
                    }],
                    cluster_id: None,
                    controller_id: 20,
                    topics: vec![MetadataResponseTopic {
                        error_code: 0,
                        name: String::from("foo"),
                        is_internal: false,
                        partitions: vec![MetadataResponsePartition {
                            error_code: 0,
                            partition_index: 0,
                            leader_id: 0,
                            leader_epoch: 0,
                            replica_nodes: vec![0, 1, 2],
                            isr_nodes: vec![0, 1, 2],
                            offline_replicas: vec![0, 1, 2],
                        }],
                    }],
                });
                let response =
                    KafkaResponse::new(request.api_version, request.correlation_id, body);
                transport
                    .write_response(response)
                    .await
                    .expect("could not write response");
            }
                .boxed()
                .fuse();

            select! {
              request = handle => {} // continue
              _ = cancel => { return; } // cancellation recieved, exit.
            }
        }
    }

    #[runtime::test]
    async fn request_response() {
        let (c, s) = MemorySocket::new_pair();
        let server = new_server_transport(s);
        let mut client = new_client_transport(c);
        let (shutdown_tx, shutdown_rx) = futures::channel::oneshot::channel::<()>();
        let metadata_server_join_handle =
            runtime::task::spawn(metadata_server(server, shutdown_rx));

        let metadata_request_body = RequestBody::MetadataRequest(MetadataRequest {
            topics: Some(vec![MetadataRequestTopic {
                name: String::from("foo"),
            }]),
            allow_auto_topic_creation: false,
        });
        let correlation_id = 5;
        let api_version = 1;
        let metadata_request = KafkaRequest::new(
            ApiKeys::Metadata,
            api_version,
            correlation_id,
            "client",
            metadata_request_body,
        );
        client
            .write_request(metadata_request)
            .await
            .expect("failed to write request");
        let response = client
            .read_response(ApiKeys::Metadata, api_version)
            .await
            .expect("could not read response");
        assert_eq!(correlation_id, response.correlation_id);
        if let ResponseBody::MetadataResponse(resp) = response.body {
            assert_eq!(1, resp.topics.len());
            assert_eq!(String::from("foo"), resp.topics[0].name)
        } else {
            assert!(false);
        }

        shutdown_tx.send(()).expect("shutdown initiated");
        metadata_server_join_handle.await;
    }

    #[runtime::test]
    async fn multiple_requests() {
        let (c, s) = MemorySocket::new_pair();
        let server = new_server_transport(s);
        let mut client = new_client_transport(c);
        let (shutdown_tx, shutdown_rx) = futures::channel::oneshot::channel::<()>();
        let metadata_server_join_handle =
            runtime::task::spawn(metadata_server(server, shutdown_rx));

        fn new_metadata_request(correlation_id: i32) -> KafkaRequest {
            let metadata_request_body = RequestBody::MetadataRequest(MetadataRequest {
                topics: Some(vec![MetadataRequestTopic {
                    name: String::from("foo"),
                }]),
                allow_auto_topic_creation: false,
            });
            KafkaRequest::new(
                ApiKeys::Metadata,
                1,
                correlation_id,
                "client",
                metadata_request_body,
            )
        }

        // Write out 3 metadata requests
        client
            .write_request(new_metadata_request(1))
            .await
            .expect("failed to write request");
        client
            .write_request(new_metadata_request(2))
            .await
            .expect("failed to write request");
        client
            .write_request(new_metadata_request(3))
            .await
            .expect("failed to write request");

        let resp = client
            .read_response(ApiKeys::Metadata, 1)
            .await
            .expect("expected to read response");
        assert_eq!(resp.correlation_id, 1);
        let resp = client
            .read_response(ApiKeys::Metadata, 1)
            .await
            .expect("expected to read response");
        assert_eq!(resp.correlation_id, 2);
        let resp = client
            .read_response(ApiKeys::Metadata, 1)
            .await
            .expect("expected to read response");
        assert_eq!(resp.correlation_id, 3);

        shutdown_tx.send(()).expect("shutdown initiated");
        metadata_server_join_handle.await;
    }
}
