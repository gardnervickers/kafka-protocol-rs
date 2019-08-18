//! API Types for the [Kafka Protocol](https://kafka.apache.org/protocol)
//!
//! The [Kafka Protocol](https://kafka.apache.org/protocol) does not include API type or version
//! information in responses. Because of this, clients must always supply an `api_type` and `api_version`
//! when decoding responses.
use crate::apikey::ApiKeys;
pub use kafka_protocol::{CodecError, KafkaRpc, KafkaRpcType};
use kafka_protocol::{DeserializeCtx, SerializeCtx};
use std::io;

pub mod api;
pub mod apikey;
pub mod errors;
mod util;
pub use util::AsApiVersionsResponseKey;

pub trait KafkaRequestBody: KafkaRpcType {
    fn api_key() -> apikey::ApiKeys;
}

pub trait KafkaResponseBody: KafkaRpcType {
    fn api_key() -> apikey::ApiKeys;
}

#[derive(Debug, Clone, PartialEq)]
pub struct KafkaRequest {
    pub api_key: apikey::ApiKeys,
    pub api_version: i16,
    pub correlation_id: i32,
    pub client_id: String,
    pub body: api::RequestBody,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KafkaResponse {
    pub api_version: i16,
    pub correlation_id: i32,
    pub body: api::ResponseBody,
}

impl KafkaRequest {
    pub fn read<R: io::Read>(reader: R) -> Result<Self, CodecError> {
        let mut ctx: DeserializeCtx<R> = DeserializeCtx::new(reader, 0);
        let api_key: i16 = KafkaRpcType::read(&mut ctx)?;
        let api_version: i16 = KafkaRpcType::read(&mut ctx)?;
        ctx.version = api_version;

        let correlation_id: i32 = KafkaRpcType::read(&mut ctx)?;
        let client_id: String = KafkaRpcType::read(&mut ctx)?;
        let body = api::RequestBody::read(&mut ctx, api_key)?;
        Ok(Self {
            api_key: api_key.into(),
            api_version,
            correlation_id,
            client_id,
            body,
        })
    }

    pub fn size(&self) -> usize {
        let version = self.api_version;
        (self.api_key as i16).size(version)
            + self.api_version.size(version)
            + self.correlation_id.size(version)
            + self.client_id.size(version)
            + self.body.size(version)
    }

    pub fn write<W: io::Write>(&self, writer: W) -> Result<(), CodecError> {
        let mut ctx: SerializeCtx<W> = SerializeCtx::new(writer, self.api_version);
        (self.api_key as i16).write(&mut ctx)?;
        self.api_version.write(&mut ctx)?;
        self.correlation_id.write(&mut ctx)?;
        self.client_id.write(&mut ctx)?;
        self.body.write(&mut ctx)?;

        Ok(())
    }

    pub fn new<C: Into<String> + Sized, B: Into<api::RequestBody> + Sized>(
        api_key: apikey::ApiKeys,
        api_version: i16,
        correlation_id: i32,
        client_id: C,
        body: B,
    ) -> Self {
        Self {
            api_key,
            api_version,
            correlation_id,
            client_id: client_id.into(),
            body: body.into(),
        }
    }
}

impl KafkaResponse {
    pub fn new(api_version: i16, correlation_id: i32, body: api::ResponseBody) -> Self {
        Self {
            api_version,
            correlation_id,
            body,
        }
    }

    pub fn read<R: io::Read>(
        reader: R,
        api_key: ApiKeys,
        api_version: i16,
    ) -> Result<Self, CodecError> {
        let mut ctx: DeserializeCtx<R> = DeserializeCtx::new(reader, api_version);
        let correlation_id: i32 = KafkaRpcType::read(&mut ctx)?;
        let body = api::ResponseBody::read(&mut ctx, api_key.into())?;
        Ok(KafkaResponse::new(api_version, correlation_id, body))
    }

    pub fn read2<R: io::Read, B: Into<api::ResponseBody> + KafkaRpcType>(
        reader: R,
        api_version: i16,
    ) -> Result<Self, CodecError> {
        let mut ctx: DeserializeCtx<R> = DeserializeCtx::new(reader, api_version);
        let correlation_id: i32 = KafkaRpcType::read(&mut ctx)?;
        let body = B::read(&mut ctx)?;
        Ok(KafkaResponse::new(api_version, correlation_id, body.into()))
    }

    pub fn size(&self) -> usize {
        self.correlation_id.size(self.api_version) + self.body.size(self.api_version)
    }

    pub fn write<W: io::Write>(&self, writer: W) -> Result<(), CodecError> {
        let mut ctx: SerializeCtx<W> = SerializeCtx::new(writer, self.api_version);
        self.correlation_id.write(&mut ctx)?;
        self.body.write(&mut ctx)?;
        Ok(())
    }
}
