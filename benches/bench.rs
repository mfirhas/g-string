mod benches {
    pub mod construction;
}

use criterion::{Criterion, criterion_group, criterion_main};

fn criterion() -> Criterion {
    let output_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(".bench");

    Criterion::default().output_directory(&output_dir)
}

criterion_group! {
    name = construction;
    config = criterion();
    targets =
        benches::construction::bench_try_new,
        benches::construction::bench_gstring_macro,
        benches::construction::bench_macro_vs_runtime,
}

criterion_main!(construction,);
