#![allow(clippy::blacklisted_name)]
use kafka_protocol::{KafkaRpc, KafkaRpcType};
use kafka_protocol_derive::KafkaRpc;

#[derive(KafkaRpc)]
#[kafka(added = 0, removed = 10)]
struct FullyVersioned {}

#[derive(KafkaRpc)]
#[kafka(added = 0)]
struct PartialVersion {}

#[derive(KafkaRpc)]
#[kafka(added = 0)]
struct SizedVersion {
    #[kafka(removed = 5)]
    foo: i32,
    #[kafka(added = 1)]
    bar: Vec<String>,
    nested: PartialVersion,
}

fn main() {
    assert_eq!(0, <FullyVersioned as KafkaRpc>::version_added());
    assert_eq!(Some(10), <FullyVersioned as KafkaRpc>::version_removed());

    assert_eq!(0, <PartialVersion as KafkaRpc>::version_added());
    assert_eq!(None, <PartialVersion as KafkaRpc>::version_removed());

    let zero_sized = PartialVersion {};
    assert_eq!(0, zero_sized.size(0));

    let sized_version = SizedVersion {
        foo: 0,
        bar: vec![String::from("Hello")],
        nested: PartialVersion {},
    };
    assert_eq!(
        15,
        sized_version.size(1),
        "expected V1 to include all fields"
    );
    assert_eq!(
        4,
        sized_version.size(0),
        "expected V0 to exclude all fields except foo"
    );
    assert_eq!(11, sized_version.size(5), "expected V5 to exclude foo");
}
