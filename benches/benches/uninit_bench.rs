use criterion::{BenchmarkId, Criterion, black_box};
use g_string::{GStringNV, NoValidation, uninit::GStringUninit};

const S5: &str = "hello"; //  5
const S43: &str = "the quick brown fox jumps over the lazy dog"; // 43
const S64: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQ789"; // 48 — fits S256, not S32
const UNI: &str = "héllo wörld 🌍"; // multi-byte UTF-8

type S50 = GStringNV<0, 50, false>;
type S100 = GStringNV<0, 100, false>;
type S255 = GStringNV<0, 255, false>;
type S500 = GStringNV<0, 500, false>;
type S1000 = GStringNV<0, 1000, false>;

type U50 = GStringUninit<NoValidation, 0, 50, false>;
type U100 = GStringUninit<NoValidation, 0, 100, false>;
type U255 = GStringUninit<NoValidation, 0, 255, false>;
type U500 = GStringUninit<NoValidation, 0, 500, false>;
type U1000 = GStringUninit<NoValidation, 0, 1000, false>;

pub fn bench_all(c: &mut Criterion) {
    bench_try_new_vs_string(c);
}

pub fn bench_try_new_vs_string(c: &mut Criterion) {
    let mut g = c.benchmark_group("construction/string_vs_gstring");

    for (label, input) in [("5b", S5), ("43b", S43), ("48b", S64), ("unicode", UNI)] {
        g.bench_with_input(BenchmarkId::new("gstring_50", label), &input, |b, s| {
            b.iter(|| black_box(S50::try_new(*s).unwrap()))
        });

        g.bench_with_input(
            BenchmarkId::new("gstring_uninit_50", label),
            &input,
            |b, s| b.iter(|| black_box(U50::try_new(*s).unwrap())),
        );

        g.bench_with_input(BenchmarkId::new("gstring_100", label), &input, |b, s| {
            b.iter(|| black_box(S100::try_new(*s).unwrap()))
        });

        g.bench_with_input(
            BenchmarkId::new("gstring_uninit_100", label),
            &input,
            |b, s| b.iter(|| black_box(U100::try_new(*s).unwrap())),
        );

        g.bench_with_input(BenchmarkId::new("gstring_255", label), &input, |b, s| {
            b.iter(|| black_box(S255::try_new(*s).unwrap()))
        });

        g.bench_with_input(
            BenchmarkId::new("gstring_uninit_255", label),
            &input,
            |b, s| b.iter(|| black_box(U255::try_new(*s).unwrap())),
        );

        g.bench_with_input(BenchmarkId::new("gstring_500", label), &input, |b, s| {
            b.iter(|| black_box(S500::try_new(*s).unwrap()))
        });

        g.bench_with_input(
            BenchmarkId::new("gstring_uninit_500", label),
            &input,
            |b, s| b.iter(|| black_box(U500::try_new(*s).unwrap())),
        );

        g.bench_with_input(BenchmarkId::new("gstring_1000", label), &input, |b, s| {
            b.iter(|| black_box(S1000::try_new(*s).unwrap()))
        });

        g.bench_with_input(
            BenchmarkId::new("gstring_uninit_1000", label),
            &input,
            |b, s| b.iter(|| black_box(U1000::try_new(*s).unwrap())),
        );

        g.bench_with_input(BenchmarkId::new("string", label), &input, |b, s| {
            b.iter(|| black_box(String::from(*s)))
        });
    }

    g.finish();
}
