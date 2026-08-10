//! Construction benchmarks comparing GString against String.
//!
//! Both types are constructed from the same `&str` input.
//!
//! Run:
//!   cargo bench -- string_construction

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
// GString vs String
//
// Both operations are benchmarked in the same group so Criterion displays
// the distributions next to each other for each input size.
//
// GString:
//   - fixed-size stack buffer
//   - copies input into the buffer
//   - performs bounds checking
//
// String:
//   - heap allocation
//   - allocates according to input size
//   - copies input into the allocation
// ---------------------------------------------------------------------------

pub fn bench_all(c: &mut Criterion) {
    bench_construction(c);
    bench_push_str(c);
}

pub fn bench_construction(c: &mut Criterion) {
    let mut g = c.benchmark_group("construction/string_vs_gstring");

    for (label, input) in [("5b", S5), ("43b", S43), ("48b", S64), ("unicode", UNI)] {
        g.bench_with_input(BenchmarkId::new("gstring_50", label), &input, |b, s| {
            b.iter(|| black_box(S50::try_new(*s).unwrap()))
        });

        g.bench_with_input(BenchmarkId::new("gstring_100", label), &input, |b, s| {
            b.iter(|| black_box(S100::try_new(*s).unwrap()))
        });

        g.bench_with_input(BenchmarkId::new("gstring_255", label), &input, |b, s| {
            b.iter(|| black_box(S255::try_new(*s).unwrap()))
        });

        g.bench_with_input(BenchmarkId::new("gstring_500", label), &input, |b, s| {
            b.iter(|| black_box(S500::try_new(*s).unwrap()))
        });

        g.bench_with_input(BenchmarkId::new("gstring_1000", label), &input, |b, s| {
            b.iter(|| black_box(S1000::try_new(*s).unwrap()))
        });

        g.bench_with_input(BenchmarkId::new("string", label), &input, |b, s| {
            b.iter(|| black_box(String::from(*s)))
        });
    }

    g.finish();
}

pub fn bench_push_str(c: &mut Criterion) {
    let mut g = c.benchmark_group("construction/string_vs_gstring__push_str");

    for (label, input) in [("5b", S5), ("43b", S43), ("48b", S64), ("unicode", UNI)] {
        g.bench_with_input(BenchmarkId::new("gstring_100", label), &input, |b, s| {
            b.iter(|| {
                let mut gs = black_box(S100::try_new(*s).unwrap());
                gs.push_str("new string").unwrap();
                black_box(gs)
            })
        });

        g.bench_with_input(BenchmarkId::new("gstring_255", label), &input, |b, s| {
            b.iter(|| {
                let mut gs = black_box(S255::try_new(*s).unwrap());
                gs.push_str("new string").unwrap();
                black_box(gs)
            })
        });

        g.bench_with_input(BenchmarkId::new("gstring_500", label), &input, |b, s| {
            b.iter(|| {
                let mut gs = black_box(S500::try_new(*s).unwrap());
                gs.push_str("new string").unwrap();
                black_box(gs)
            })
        });

        g.bench_with_input(BenchmarkId::new("gstring_1000", label), &input, |b, s| {
            b.iter(|| {
                let mut gs = black_box(S1000::try_new(*s).unwrap());
                gs.push_str("new string").unwrap();
                black_box(gs)
            })
        });

        g.bench_with_input(BenchmarkId::new("string", label), &input, |b, s| {
            b.iter(|| {
                let mut gs = black_box(String::from(*s));
                gs.push_str("new string");
                black_box(gs)
            })
        });
    }

    g.finish();
}
