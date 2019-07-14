/// Utility to check version of a kafka request and parse
fn version_check<T: kafka_protocol::KafkaRpc>(
    version: i16,
) -> Result<(), kafka_protocol::CodecError> {
    let version_added = <T as kafka_protocol::KafkaRpc>::version_added();
    let version_removed = <T as kafka_protocol::KafkaRpc>::version_removed();
    if version < version_added || version_removed.map(|vr| version >= vr).unwrap_or(false) {
        Err(kafka_protocol::CodecError::InvalidVersion)
    } else {
        Ok(())
    }
}

pub fn version_check_read<T: kafka_protocol::KafkaRpc, R: std::io::Read>(
    ctx: &mut kafka_protocol::DeserializeCtx<R>,
) -> Result<T, kafka_protocol::CodecError> {
    version_check::<T>(ctx.version)?;
    T::read(ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use kafka_protocol_derive::KafkaRpc;

    #[test]
    fn version_checks() {
        #[derive(KafkaRpc)]
        #[kafka(added = 1i16, removed = 2i16)]
        struct __SomeStruct {}
        assert!(version_check::<__SomeStruct>(1).is_ok());
        assert!(version_check::<__SomeStruct>(2).is_err());
        assert!(version_check::<__SomeStruct>(0).is_err());
    }
}
