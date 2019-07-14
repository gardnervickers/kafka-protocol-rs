//! Implementations of [KafkaRpcType](crate::KafkaRpcType) for common primitive types.
use crate::{CodecError, DeserializeCtx, KafkaRpcType, SerializeCtx};
use byteorder::{ReadBytesExt, WriteBytesExt, BE};
use std::io;

macro_rules! impl_rpc_type_num {
    ($ty:ty, $read_m:ident, $write_m:ident, $sz:expr) => {
        impl KafkaRpcType for $ty {
            #[inline(always)]
            fn read<R: io::Read>(ctx: &mut DeserializeCtx<R>) -> Result<Self, CodecError> {
                Ok(ctx.reader.$read_m::<BE>().map_err(CodecError::Io)?)
            }
            #[inline(always)]
            fn size(&self, _version: i16) -> usize {
                $sz
            }
            #[inline(always)]
            fn write<W: io::Write>(&self, ctx: &mut SerializeCtx<W>) -> Result<(), CodecError> {
                Ok(ctx.writer.$write_m::<BE>(*self).map_err(CodecError::Io)?)
            }
        }
    };
}

impl_rpc_type_num!(i16, read_i16, write_i16, 2);
impl_rpc_type_num!(i32, read_i32, write_i32, 4);
impl_rpc_type_num!(i64, read_i64, write_i64, 8);

impl KafkaRpcType for i8 {
    #[inline(always)]
    fn read<R: io::Read>(ctx: &mut DeserializeCtx<R>) -> Result<Self, CodecError> {
        Ok(ctx.reader.read_i8().map_err(CodecError::Io)?)
    }
    #[inline(always)]
    fn size(&self, _version: i16) -> usize {
        1
    }

    #[inline(always)]
    fn write<W: io::Write>(&self, ctx: &mut SerializeCtx<W>) -> Result<(), CodecError> {
        Ok(ctx.writer.write_i8(*self).map_err(CodecError::Io)?)
    }
}

impl KafkaRpcType for Option<i8> {
    fn read<R: io::Read>(ctx: &mut DeserializeCtx<R>) -> Result<Self, CodecError> {
        Ok(Some(ctx.reader.read_i8().map_err(CodecError::Io)?))
    }

    fn size(&self, _version: i16) -> usize {
        1
    }

    fn write<W: io::Write>(&self, ctx: &mut SerializeCtx<W>) -> Result<(), CodecError> {
        match self {
            None => Ok(()),
            Some(v) => Ok(ctx.writer.write_i8(*v).map_err(CodecError::Io)?),
        }
    }
}

impl KafkaRpcType for bool {
    #[inline(always)]
    fn read<R: io::Read>(ctx: &mut DeserializeCtx<R>) -> Result<Self, CodecError> {
        match <i8 as KafkaRpcType>::read(ctx)? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(CodecError::InvalidValue(String::from(
                "expected type boolean to be either 1 or 0",
            ))),
        }
    }
    #[inline(always)]
    fn size(&self, _version: i16) -> usize {
        1
    }
    #[inline(always)]
    fn write<W: io::Write>(&self, ctx: &mut SerializeCtx<W>) -> Result<(), CodecError> {
        let n: i8 = if *self { 1 } else { 0 };
        n.write(ctx)
    }
}

impl KafkaRpcType for String {
    #[inline(always)]
    fn read<R: io::Read>(ctx: &mut DeserializeCtx<R>) -> Result<Self, CodecError> {
        let len: i16 = KafkaRpcType::read(ctx)?;
        if len < 0 {
            return Err(CodecError::NegativeLength);
        }
        let mut strbuf = vec![0; len as usize];
        ctx.reader
            .read_exact(&mut strbuf[0..len as usize])
            .map_err(CodecError::Io)?;
        Ok(String::from_utf8(strbuf).map_err(CodecError::Utf8)?)
    }
    #[inline(always)]
    fn size(&self, version: i16) -> usize {
        self.len() + <i16 as KafkaRpcType>::size(&0, version)
    }
    #[inline(always)]
    fn write<W: io::Write>(&self, ctx: &mut SerializeCtx<W>) -> Result<(), CodecError> {
        let len = self.len();
        if len > std::i16::MAX as usize {
            Err(CodecError::InvalidValue(String::from(
                "String larger than i16::MAX",
            )))
        } else {
            KafkaRpcType::write(&(len as i16), ctx)?;
            ctx.writer
                .write_all(self.as_bytes())
                .map_err(CodecError::Io)?;
            Ok(())
        }
    }
}

impl<T: KafkaRpcType> KafkaRpcType for Vec<T> {
    fn read<R: io::Read>(ctx: &mut DeserializeCtx<R>) -> Result<Self, CodecError> {
        let len: i32 = KafkaRpcType::read(ctx)?;
        if len < 0 {
            return Err(CodecError::NegativeLength);
        }
        let mut result = Vec::with_capacity(len as usize);
        for _ in 0..len {
            result.push(<T as KafkaRpcType>::read(ctx)?);
        }
        Ok(result)
    }

    fn size(&self, version: i16) -> usize {
        let sum: usize = self.iter().map(|v| KafkaRpcType::size(v, version)).sum();
        sum + 4
    }

    fn write<W: io::Write>(&self, ctx: &mut SerializeCtx<W>) -> Result<(), CodecError> {
        let len = self.len();
        if len > std::i32::MAX as usize {
            Err(CodecError::InvalidValue(String::from(
                "Vec length larger than i32::MAX",
            )))
        } else {
            KafkaRpcType::write(&(len as i32), ctx)?;
            for item in self.iter() {
                item.write(ctx)?;
            }
            Ok(())
        }
    }
}

impl KafkaRpcType for Option<String> {
    #[inline(always)]
    fn read<R: io::Read>(ctx: &mut DeserializeCtx<R>) -> Result<Self, CodecError> {
        let len: i16 = KafkaRpcType::read(ctx)?;
        if len == -1 {
            Ok(None)
        } else if len < -1 {
            Err(CodecError::InvalidValue(String::from(
                "length for Option<String> was < -1",
            )))
        } else {
            let mut strbuf = vec![0; len as usize];
            ctx.reader
                .read_exact(&mut strbuf[0..len as usize])
                .map_err(CodecError::Io)?;
            Ok(Some(String::from_utf8(strbuf).map_err(CodecError::Utf8)?))
        }
    }
    #[inline(always)]
    fn size(&self, version: i16) -> usize {
        match self {
            Some(string) => string.len() + <i16 as KafkaRpcType>::size(&0, version),
            None => <i16 as KafkaRpcType>::size(&0, version),
        }
    }
    #[inline(always)]
    fn write<W: io::Write>(&self, ctx: &mut SerializeCtx<W>) -> Result<(), CodecError> {
        match self {
            Some(string) => {
                let len = string.len();
                KafkaRpcType::write(&(len as i16), ctx)?;
                ctx.writer
                    .write_all(string.as_bytes())
                    .map_err(CodecError::Io)?;
                Ok(())
            }
            None => KafkaRpcType::write(&(-1 as i16), ctx),
        }
    }
}

impl<T: KafkaRpcType> KafkaRpcType for Option<Vec<T>> {
    fn read<R: io::Read>(ctx: &mut DeserializeCtx<R>) -> Result<Self, CodecError> {
        let len: i32 = KafkaRpcType::read(ctx)?;
        if len == -1 {
            Ok(None)
        } else if len < -1 {
            Err(CodecError::InvalidValue(String::from(
                "length for Option<Vec<_>> was < -1",
            )))
        } else {
            let mut result = Vec::with_capacity(len as usize);
            for _ in 0..len {
                result.push(<T as KafkaRpcType>::read(ctx)?);
            }
            Ok(Some(result))
        }
    }

    fn size(&self, version: i16) -> usize {
        let sum: usize = self.iter().map(|v| KafkaRpcType::size(v, version)).sum();
        sum + 4
    }

    fn write<W: io::Write>(&self, ctx: &mut SerializeCtx<W>) -> Result<(), CodecError> {
        match self {
            Some(coll) => {
                let len = coll.len();
                if len > std::i32::MAX as usize {
                    Err(CodecError::InvalidValue(String::from(
                        "Vec length larger than i32::MAX",
                    )))
                } else {
                    KafkaRpcType::write(&(len as i32), ctx)?;
                    for item in coll.iter() {
                        item.write(ctx)?;
                    }
                    Ok(())
                }
            }
            None => KafkaRpcType::write(&(-1i32), ctx),
        }
    }
}

impl KafkaRpcType for Vec<u8> {
    fn read<R: io::Read>(ctx: &mut DeserializeCtx<R>) -> Result<Self, CodecError> {
        let len: i32 = KafkaRpcType::read(ctx)?;
        if len < 0 {
            Err(CodecError::InvalidValue(String::from(
                "length for Vec<u8> was < 0",
            )))
        } else if len == 0 {
            Ok(Vec::with_capacity(0))
        } else {
            let mut result = Vec::with_capacity(len as usize);
            ctx.reader.read_exact(&mut result).map_err(CodecError::Io)?;
            Ok(result)
        }
    }

    fn size(&self, _version: i16) -> usize {
        self.len() + 4
    }

    fn write<W: io::Write>(&self, ctx: &mut SerializeCtx<W>) -> Result<(), CodecError> {
        let len = self.len();
        KafkaRpcType::write(&(len as i32), ctx)?;
        ctx.writer.write_all(&self[..]).map_err(CodecError::Io)?;
        Ok(())
    }
}

impl KafkaRpcType for Option<Vec<u8>> {
    fn read<R: io::Read>(ctx: &mut DeserializeCtx<R>) -> Result<Self, CodecError> {
        let len: i32 = KafkaRpcType::read(ctx)?;
        if len == -1 {
            Ok(None)
        } else if len < -1 {
            Err(CodecError::InvalidValue(String::from(
                "length for Option<Vec<u8>> was < -1",
            )))
        } else {
            let mut result = Vec::with_capacity(len as usize);
            ctx.reader.read_exact(&mut result).map_err(CodecError::Io)?;
            Ok(Some(result))
        }
    }

    fn size(&self, _version: i16) -> usize {
        match self {
            None => 4,
            Some(inner) => inner.len() + 4,
        }
    }

    fn write<W: io::Write>(&self, ctx: &mut SerializeCtx<W>) -> Result<(), CodecError> {
        match self {
            None => {
                KafkaRpcType::write(&(-1 as i32), ctx)?;
            }
            Some(inner) => {
                ctx.writer.write_all(&inner[..]).map_err(CodecError::Io)?;
            }
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use proptest::test_runner::{Config, TestRunner};
    use std::fmt::Debug;
    use std::io::Cursor;

    fn round_trip<T: KafkaRpcType + PartialEq + Debug>(input: &T) -> Result<T, CodecError> {
        let size = input.size(0);
        let buf = Vec::with_capacity(size);
        let mut sctx = SerializeCtx::new(Cursor::new(buf), 0);
        input.write(&mut sctx)?;
        let buf = sctx.into_inner().into_inner();
        let mut dctx = DeserializeCtx::new(Cursor::new(buf), 0);
        let result = T::read(&mut dctx)?;
        Ok(result)
    }

    fn check_round_trip<T: KafkaRpcType + PartialEq + Debug>(
        input: T,
    ) -> Result<(), proptest::prelude::TestCaseError> {
        let result = round_trip(&input);
        prop_assert!(result.is_ok());
        let unwrapped = result.unwrap();
        prop_assert_eq!(input, unwrapped);
        Ok(())
    }

    #[test]
    fn check_numerics() {
        TestRunner::new(Config::default())
            .run(&(any::<bool>()), check_round_trip)
            .unwrap();

        TestRunner::new(Config::default())
            .run(&(any::<i8>()), check_round_trip)
            .unwrap();

        TestRunner::new(Config::default())
            .run(&(any::<i16>()), check_round_trip)
            .unwrap();

        TestRunner::new(Config::default())
            .run(&(any::<i32>()), check_round_trip)
            .unwrap();

        TestRunner::new(Config::default())
            .run(&(any::<i64>()), check_round_trip)
            .unwrap();
    }

    #[test]
    fn check_optional_types() {
        TestRunner::new(Config::default())
            .run(&(any::<Option<String>>()), check_round_trip)
            .unwrap();

        TestRunner::new(Config::default())
            .run(&(any::<Option<Vec<i32>>>()), check_round_trip)
            .unwrap();
    }

    #[test]
    fn check_variable_sized_types() {
        TestRunner::new(Config::default())
            .run(&(any::<String>()), check_round_trip)
            .unwrap();

        TestRunner::new(Config::default())
            .run(&(any::<Vec<i32>>()), check_round_trip)
            .unwrap();
    }

    #[test]
    fn check_nested_types() {
        TestRunner::new(Config::with_cases(10))
            .run(&(any::<Vec<Vec<i8>>>()), check_round_trip)
            .unwrap();

        TestRunner::new(Config::with_cases(10))
            .run(&(any::<Vec<Option<String>>>()), check_round_trip)
            .unwrap();
    }
}
