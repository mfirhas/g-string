use criterion::{Criterion, criterion_group, criterion_main};

mod benches {
    pub fn noop_bench(_: &mut criterion::Criterion) {}
    pub mod construction;
    pub mod string_construction;
}

fn criterion() -> Criterion {
    Criterion::default()
}

criterion_group! {
    name = g_string;
    config = criterion();
    targets =
        benches::noop_bench,
        benches::construction::bench_all,
        benches::string_construction::bench_all,
}

criterion_main!(g_string,);
