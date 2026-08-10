extern crate alloc;

use alloc::string::String;
use criterion::{BenchmarkId, Criterion, black_box};
use g_string::GStringNV;

// ---------------------------------------------------------------------------
// Type aliases
// ---------------------------------------------------------------------------

type S50 = GStringNV<0, 50, false>;
type S100 = GStringNV<0, 100, false>;
type S255 = GStringNV<0, 255, false>;
type S500 = GStringNV<0, 500, false>;
type S1000 = GStringNV<0, 1000, false>;

// ---------------------------------------------------------------------------
// Input fixtures
// ---------------------------------------------------------------------------

//                            len (bytes)
const S5: &str = "hello"; //  5
const S43: &str = "the quick brown fox jumps over the lazy dog"; // 43
const S64: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQ789"; // 48
const UNI: &str = "héllo wörld 🌍";

// ---------------------------------------------------------------------------
// as_str
//
// Construction happens OUTSIDE the benchmark.
//
// We only measure:
//
//     GString::as_str()
//     String::as_str()
//
// The returned reference is passed through black_box so the compiler cannot
// eliminate the access entirely.
// ---------------------------------------------------------------------------

pub fn bench_all(c: &mut Criterion) {
    bench_as_str(c);
}

pub fn bench_as_str(c: &mut Criterion) {
    let mut g = c.benchmark_group("access/as_str/string_vs_gstring");

    for (label, input) in [("5b", S5), ("43b", S43), ("48b", S64), ("unicode", UNI)] {
        let g50 = S50::try_new(input).unwrap();
        let g100 = S100::try_new(input).unwrap();
        let g255 = S255::try_new(input).unwrap();
        let g500 = S500::try_new(input).unwrap();
        let g1000 = S1000::try_new(input).unwrap();

        let string = String::from(input);

        g.bench_function(BenchmarkId::new("gstring_50", label), |b| {
            b.iter(|| black_box(g50.as_str()));
        });

        g.bench_function(BenchmarkId::new("gstring_100", label), |b| {
            b.iter(|| black_box(g100.as_str()));
        });

        g.bench_function(BenchmarkId::new("gstring_255", label), |b| {
            b.iter(|| black_box(g255.as_str()));
        });

        g.bench_function(BenchmarkId::new("gstring_500", label), |b| {
            b.iter(|| black_box(g500.as_str()));
        });

        g.bench_function(BenchmarkId::new("gstring_1000", label), |b| {
            b.iter(|| black_box(g1000.as_str()));
        });

        g.bench_function(BenchmarkId::new("string", label), |b| {
            b.iter(|| black_box(string.as_str()));
        });
    }

    g.finish();
}
