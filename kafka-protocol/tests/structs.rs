#![allow(clippy::unit_arg, clippy::blacklisted_name)]
use kafka_protocol::KafkaRpcType;
use kafka_protocol_derive::KafkaRpc;
use proptest::{
    prelude::*,
    test_runner::{Config, TestRunner},
};
use proptest_derive::Arbitrary;
mod utils;
use utils::round_trip;

#[derive(Debug, PartialEq, KafkaRpc, Clone, Arbitrary)]
pub(crate) struct Simple {
    pub client_id: String,
    pub flags: Vec<i8>,
    pub ttl: i32,
    pub topics: Vec<String>,
    pub default: i32,
}

#[derive(Debug, PartialEq, KafkaRpc, Clone, Arbitrary)]
pub(crate) struct Nested {
    pub barfoo: bool,
    pub foobar: i32,
    pub nested: Simple,
    pub multinested: Vec<Simple>,
}

#[test]
fn check_simple_struct() {
    let mut runner: TestRunner = TestRunner::new(Config::with_cases(100));
    runner
        .run(
            &(any::<Simple>(), 0i16..10i16),
            |(simple_struct, version)| {
                let result = round_trip(&simple_struct, version);
                prop_assert!(result.is_ok());
                let unwrapped = result.unwrap();
                prop_assert_eq!(&simple_struct, &unwrapped);
                prop_assert_eq!(&simple_struct.size(version), &unwrapped.size(version));
                Ok(())
            },
        )
        .unwrap();
}
#[test]
fn check_nested_struct() {
    let mut runner: TestRunner = TestRunner::new(Config::with_cases(10));
    runner
        .run(
            &(any::<Nested>(), 0i16..10i16),
            |(nested_struct, version)| {
                let result = round_trip(&nested_struct, version);
                prop_assert!(result.is_ok());
                let unwrapped = result.unwrap();
                prop_assert_eq!(&nested_struct, &unwrapped);
                prop_assert_eq!(&nested_struct.size(version), &unwrapped.size(version));
                Ok(())
            },
        )
        .unwrap();
}
// Unit tests for past proptest failures
#[test]
fn failure1() {
    let s = Simple {
        client_id: "".to_owned(),
        flags: vec![],
        ttl: 0,
        topics: vec!["".to_owned()],
        default: 0,
    };
    let result = round_trip(&s, 1);
    assert!(result.is_ok());
    let unwrapped = result.unwrap();
    assert_eq!(s, unwrapped);
}
