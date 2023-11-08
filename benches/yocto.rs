use crdt_bench_native::{entry, YOctoDoc};
use criterion::criterion_main;

pub fn yrs() {
    entry::<YOctoDoc>("y-octo");
}

criterion_main!(yrs);
