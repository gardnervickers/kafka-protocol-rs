use std::io;
mod primitives;

pub trait KafkaRpc: KafkaRpcType {
    fn version_added() -> i16;
    fn version_removed() -> Option<i16>;
    fn apikey() -> i16;
}

pub struct DeserializeCtx<R> {
    pub reader: R,
    pub version: i16,
}

pub struct SerializeCtx<W> {
    pub writer: W,
    pub version: i16,
}

impl<W> SerializeCtx<W> {
    pub fn new(writer: W, version: i16) -> Self {
        SerializeCtx { writer, version }
    }
    pub fn into_inner(self) -> W {
        self.writer
    }
}

impl<R> DeserializeCtx<R> {
    pub fn new(reader: R, version: i16) -> Self {
        DeserializeCtx { reader, version }
    }
    pub fn into_inner(self) -> R {
        self.reader
    }
}

#[derive(Debug)]
pub enum CodecError {
    Io(std::io::Error),
    InvalidValue(String),
    Utf8(std::string::FromUtf8Error),
    /// A variably sized element has a negative length
    NegativeLength,
    /// The requested version is out of range for this RPC.
    InvalidVersion {
        min: i16,
        max: Option<i16>,
        version: i16,
    },
}

pub trait KafkaRpcType: Sized {
    fn read<R: io::Read>(ctx: &mut DeserializeCtx<R>) -> Result<Self, CodecError>;
    fn size(&self, version: i16) -> usize;
    fn write<W: io::Write>(&self, ctx: &mut SerializeCtx<W>) -> Result<(), CodecError>;
}
