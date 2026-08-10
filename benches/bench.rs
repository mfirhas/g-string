use criterion::{Criterion, criterion_group, criterion_main};

mod benches {
    pub fn noop_bench(_: &mut criterion::Criterion) {}
    pub mod construction_bench;
    pub mod mutation_bench;
    pub mod query_bench;
}

fn criterion() -> Criterion {
    Criterion::default()
}

criterion_group! {
    name = g_string;
    config = criterion();
    targets =
        benches::noop_bench,
        benches::construction_bench::bench_all,
        benches::mutation_bench::bench_all,
        benches::query_bench::bench_all,
}

criterion_main!(g_string,);
