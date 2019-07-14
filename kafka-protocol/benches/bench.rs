use criterion::{criterion_group, criterion_main};
pub(crate) mod primitives;

criterion_group!(benches, primitives::primitives_bench);
criterion_main!(benches);
