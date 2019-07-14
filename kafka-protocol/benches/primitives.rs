#![allow(dead_code)]
use criterion::{Bencher, Benchmark, Criterion, Throughput};

use kafka_protocol::{DeserializeCtx, KafkaRpcType, SerializeCtx};

pub(crate) fn serialize<T: KafkaRpcType>(b: &mut Bencher, item: &T) {
    let size = item.size(0);
    let mut buf = vec![0; size];
    b.iter(|| {
        item.write(&mut SerializeCtx::new(&mut buf[0..size], 0))
            .unwrap()
    })
}

pub(crate) fn deserialize<T: KafkaRpcType>(b: &mut Bencher, item: &T) {
    let size = item.size(0);
    let mut buf = vec![0; size];
    let mut ctx = SerializeCtx::new(&mut buf[..size], 0);
    item.write(&mut ctx).unwrap();
    b.iter(|| <T as KafkaRpcType>::read(&mut DeserializeCtx::new(&buf[..], 0)))
}

pub(crate) fn primitives_bench(c: &mut Criterion) {
    c.bench_functions(
        "Strings",
        vec![
            criterion::Fun::new("serialize", |b, i| serialize(b, i)),
            criterion::Fun::new("deserialize", |b, i| deserialize(b, i)),
        ],
        vec![
            String::from_utf8(vec![b'a'; 100]).unwrap(),
            String::from_utf8(vec![b'a'; 1000]).unwrap(),
            String::from_utf8(vec![b'a'; 10000]).unwrap(),
        ],
    )
    .bench_functions(
        "Option<String>",
        vec![
            criterion::Fun::new("serialize", |b, i| serialize(b, i)),
            criterion::Fun::new("deserialize", |b, i| deserialize(b, i)),
        ],
        vec![
            Some(String::from_utf8(vec![b'a'; 100]).unwrap()),
            Some(String::from_utf8(vec![b'a'; 1000]).unwrap()),
            Some(String::from_utf8(vec![b'a'; 10000]).unwrap()),
        ],
    )
    .bench_functions(
        "Vec<i32>",
        vec![
            criterion::Fun::new("serialize", |b, i| serialize(b, i)),
            criterion::Fun::new("deserialize", |b, i| deserialize(b, i)),
        ],
        vec![vec![0i32; 100], vec![0i32; 1000], vec![0i32; 10000]],
    )
    .bench_functions(
        "Vec<String>",
        vec![
            criterion::Fun::new("serialize", |b, i| serialize(b, i)),
            criterion::Fun::new("deserialize", |b, i| deserialize(b, i)),
        ],
        vec![
            vec![vec![b'a'; 100]; 100]
                .into_iter()
                .map(|v| String::from_utf8(v).unwrap())
                .collect::<Vec<_>>(),
            vec![vec![b'a'; 100]; 1000]
                .into_iter()
                .map(|v| String::from_utf8(v).unwrap())
                .collect::<Vec<_>>(),
            vec![vec![b'a'; 100]; 10000]
                .into_iter()
                .map(|v| String::from_utf8(v).unwrap())
                .collect::<Vec<_>>(),
        ],
    )
    .bench(
        "Vec<String>",
        Benchmark::new("100MB", |b| {
            serialize(
                b,
                &vec![vec![b'a'; std::i16::MAX as usize]; 1000]
                    .into_iter()
                    .map(|v| String::from_utf8(v).unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .throughput(Throughput::Bytes(std::i16::MAX as u32 * 1000)),
    )
    .bench(
        "Vec<u8>",
        Benchmark::new("100MB", |b| {
            serialize(
                b,
                &vec![vec![0; std::i16::MAX as usize]; 1000]
                    .into_iter()
                    .map(|v| String::from_utf8(v).unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .throughput(Throughput::Bytes(std::i16::MAX as u32 * 1000)),
    )
    .bench_functions(
        "i32s",
        vec![
            criterion::Fun::new("serialize", |b, i| serialize(b, i)),
            criterion::Fun::new("deserialize", |b, i| deserialize(b, i)),
        ],
        vec![std::i32::MIN],
    );
}
