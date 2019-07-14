use kafka_protocol_derive::KafkaRpc;

#[derive(KafkaRpc, PartialEq, Debug)]
struct SimpleStruct {
    #[kafka(added = 0)]
    client_id: String,
    #[kafka(added = 1, default = "32")]
    speed: i32,
    #[kafka(default = "32")]
    speed2: i32,
    #[kafka(removed = 3, default = "32")]
    speed3: i32,
    #[kafka(added = 0, removed = 3, default = "32")]
    speed4: i32,
}

#[test]
fn test_defaults() {
    let default = SimpleStruct::default();
    assert_eq!(default.speed, 32);
    assert_eq!(default.speed2, 32);
    assert_eq!(default.speed3, 32);
    assert_eq!(default.speed4, 32);
}
