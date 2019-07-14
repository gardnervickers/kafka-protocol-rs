use kafka_protocol::{CodecError, DeserializeCtx, KafkaRpcType, SerializeCtx};
use std::{fmt::Debug, io::Cursor};

pub fn round_trip<T: KafkaRpcType + PartialEq + Debug>(
    input: &T,
    version: i16,
) -> Result<T, CodecError> {
    let size = input.size(0);
    let buf = Vec::with_capacity(size);
    let mut sctx = SerializeCtx::new(Cursor::new(buf), version);
    input.write(&mut sctx)?;
    let buf = sctx.into_inner().into_inner();
    let mut dctx = DeserializeCtx::new(Cursor::new(buf), version);
    let result = T::read(&mut dctx)?;
    Ok(result)
}
