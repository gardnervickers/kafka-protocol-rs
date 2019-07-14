use kafka_protocol::{DeserializeCtx, KafkaRpcType, SerializeCtx};
use kafka_protocol_derive::KafkaRpc;
use std::fmt::Debug;
use std::io::Cursor;
use std::ops::Range;

fn round_trip<T: KafkaRpcType + PartialEq + Debug>(input: T, versions: Range<i16>) {
    for version in versions {
        let size = input.size(0);
        let buf = Vec::with_capacity(size);
        let mut sctx = SerializeCtx::new(Cursor::new(buf), version);
        input.write(&mut sctx).unwrap();
        let buf = sctx.into_inner().into_inner();
        let mut dctx = DeserializeCtx::new(Cursor::new(buf), version);
        let result = T::read(&mut dctx).unwrap();
        assert_eq!(input, result, "expected {:?} but found {:?}", input, result)
    }
}

#[derive(KafkaRpc, PartialEq, Debug)]
struct SimpleStruct {
    #[kafka(added = 0)]
    client_id: String,
    #[kafka(added = 1, default = "32")]
    speed: i32,
}

#[derive(KafkaRpc, PartialEq, Debug)]
struct Nestee {
    variable: Vec<i32>,
    fixed: i32,
}

#[derive(KafkaRpc, PartialEq, Debug)]
struct NestedStruct {
    fixed: i32,
    variable: String,
    #[kafka(added = 1)]
    nestee: Nestee,
}

fn test_round_trips() {
    round_trip(
        SimpleStruct {
            ..Default::default()
        },
        0..2,
    );

    round_trip(
        NestedStruct {
            ..Default::default()
        },
        0..2,
    );
}

fn test_simple_struct_defaults() {
    let simple_struct_bytesv0 = vec![0, 3, b'f', b'o', b'o'];
    let simple_struct_bytesv1 = vec![0, 3, b'f', b'o', b'o', 0, 0, 0, 1];

    let mut ctxv0 = DeserializeCtx::new(Cursor::new(simple_struct_bytesv0), 0);
    let mut ctxv1 = DeserializeCtx::new(Cursor::new(simple_struct_bytesv1), 1);

    assert_eq!(
        32,
        <SimpleStruct as KafkaRpcType>::read(&mut ctxv0)
            .unwrap()
            .speed,
        "expected the specified default value to be applied if \
         the field was added after the current version"
    );
    assert_eq!(
        1,
        <SimpleStruct as KafkaRpcType>::read(&mut ctxv1)
            .unwrap()
            .speed,
        "expected the default value to be overridden if the \
         field was added for the version range"
    );
}

fn test_nested_struct_defaults() {
    let nested_struct_bytes = vec![
        0, 0, 0, 1, 0, 3, b'f', b'o', b'o', 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1,
    ];
    let mut ctx = DeserializeCtx::new(Cursor::new(nested_struct_bytes), 0);
    assert_eq!(
        Nestee {
            variable: vec![],
            fixed: 0
        },
        <NestedStruct as KafkaRpcType>::read(&mut ctx)
            .unwrap()
            .nestee,
        "expected defaulted nested types to be used if they were added after the current version"
    );

    let mut ctx = DeserializeCtx::new(Cursor::new(ctx.into_inner().into_inner()), 1);
    assert_eq!(
        Nestee {
            variable: vec![1],
            fixed: 1
        },
        <NestedStruct as KafkaRpcType>::read(&mut ctx)
            .unwrap()
            .nestee,
        "expected nested types to be used if they were added for this current verision"
    );
}

fn main() {
    test_simple_struct_defaults();
    test_round_trips();
    test_nested_struct_defaults();
}
