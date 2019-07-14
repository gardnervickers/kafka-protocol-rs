#![allow(clippy::unit_arg, clippy::blacklisted_name)]
use kafka_protocol_derive::KafkaRpc;
use proptest::prelude::*;
use proptest::test_runner::{Config, TestRunner};
use proptest_derive::Arbitrary;
mod utils;
use utils::round_trip;

#[derive(Debug, PartialEq, KafkaRpc, Clone, Arbitrary)]
struct Versioned {
    #[kafka(added = 0, removed = 2)]
    foo: String,
    #[kafka(added = 1, removed = 2, default = "32")]
    bar: i32,
    baz: bool,
}

#[test]
fn check_version_aware_serialization() {
    let default: Versioned = Default::default();
    TestRunner::new(Config::default())
        .run(&(any::<Versioned>(), 0i16..3i16), |(sample, version)| {
            let result = round_trip(&sample, version);
            prop_assert!(result.is_ok());
            let unwrapped = result.unwrap();
            if version == 0 {
                prop_assert_eq!(&sample.foo, &unwrapped.foo);
                prop_assert_eq!(&default.bar, &unwrapped.bar);
                prop_assert_eq!(&sample.baz, &sample.baz);
            } else if version == 1 {
                prop_assert_eq!(&sample.foo, &unwrapped.foo);
                prop_assert_eq!(&sample.bar, &unwrapped.bar);

                prop_assert_eq!(&sample.baz, &sample.baz);
            } else if version == 2 {
                prop_assert_eq!(&default.foo, &unwrapped.foo);
                prop_assert_eq!(&default.foo, &unwrapped.foo);
                prop_assert_eq!(&sample.baz, &unwrapped.baz);
            }
            Ok(())
        })
        .unwrap();
}
