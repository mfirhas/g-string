//! Construction benchmarks for GString.
//!
//! Key distinction this file probes:
//!
//!   `gstring!("...")` → const-evaluated at compile time via `__new`
//!                       The benchmark measures copy cost only (stack-to-stack).
//!
//!   `GString::try_new(s)` → runs at runtime: memcpy + bounds check + ASCII
//!                            check + validator call.
//!
//! Run:
//!   cargo bench -- construction
//!
//! Compare baselines:
//!   cargo bench -- --save-baseline before construction
//!   cargo bench -- --baseline    before construction

use core::fmt;
use criterion::{BenchmarkId, Criterion, Throughput, black_box};
use g_string::{GString, GStringNV, NoValidation, gstring};

pub fn bench_all(c: &mut Criterion) {
    bench_try_new(c);
    bench_gstring_macro(c);
    bench_macro_vs_runtime(c);
    bench_try_new_vs_string(c);
}

// ---------------------------------------------------------------------------
// Validator fixture — cheap (one contains-check) so we can see its marginal
// cost above the no-validation baseline.
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct MissingAt;

impl fmt::Display for MissingAt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("missing '@'")
    }
}

impl std::error::Error for MissingAt {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct AtValidator; // accepts any string containing '@'

impl g_string::Validator for AtValidator {
    type Error = MissingAt;
    fn validate(s: impl AsRef<str>) -> Result<(), Self::Error> {
        if s.as_ref().contains('@') {
            Ok(())
        } else {
            Err(MissingAt)
        }
    }
}

// ---------------------------------------------------------------------------
// Type aliases
// ---------------------------------------------------------------------------

type S256 = GStringNV<0, 256, false>; // no validation, UTF-8
type S256Ascii = GStringNV<0, 256, true>; // no validation, ASCII-only
type S256Val = GString<AtValidator, 0, 256, false>; // custom validator
type S32 = GStringNV<0, 32, false>; // tight capacity for overflow bench

// ---------------------------------------------------------------------------
// Input fixtures
// ---------------------------------------------------------------------------

//                            len (bytes)
const S5: &str = "hello"; //  5
const S43: &str = "the quick brown fox jumps over the lazy dog"; // 43
const S64: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQ789"; // 48 — fits S256, not S32
const UNI: &str = "héllo wörld 🌍"; // multi-byte UTF-8
const EMAIL: &str = "user@example.com"; // passes AtValidator

// ---------------------------------------------------------------------------
// 1. try_new — runtime construction
// ---------------------------------------------------------------------------

pub fn bench_try_new(c: &mut Criterion) {
    let mut g = c.benchmark_group("construction/try_new");

    // ── input-size sweep (no validation, UTF-8) ──────────────────────────
    for (label, input) in [("5b", S5), ("43b", S43), ("48b", S64), ("uni", UNI)] {
        g.throughput(Throughput::Bytes(input.len() as u64));
        g.bench_with_input(BenchmarkId::new("no_validation", label), input, |b, s| {
            b.iter(|| S256::try_new(black_box(s)).unwrap())
        });
    }

    // ── ASCII_ONLY flag — same inputs (only ASCII strings will succeed) ───
    for (label, input) in [("5b", S5), ("43b", S43), ("48b", S64)] {
        g.throughput(Throughput::Bytes(input.len() as u64));
        g.bench_with_input(BenchmarkId::new("ascii_only", label), input, |b, s| {
            b.iter(|| S256Ascii::try_new(black_box(s)).unwrap())
        });
    }

    // ── Custom validator overhead ─────────────────────────────────────────
    g.throughput(Throughput::Bytes(EMAIL.len() as u64));
    g.bench_function("with_validator/email_16b", |b| {
        b.iter(|| S256Val::try_new(black_box(EMAIL)).unwrap())
    });

    // ── Error paths ───────────────────────────────────────────────────────
    // These should be cheap — they bail out before the full memcpy.
    g.bench_function("err/too_long", |b| {
        b.iter(|| S32::try_new(black_box(S64)).unwrap_err())
    });
    g.bench_function("err/not_ascii", |b| {
        b.iter(|| S256Ascii::try_new(black_box(UNI)).unwrap_err())
    });
    g.bench_function("err/validation_fail", |b| {
        b.iter(|| S256Val::try_new(black_box("no-at-sign")).unwrap_err())
    });

    g.finish();
}

// ---------------------------------------------------------------------------
// 2. gstring! macro — compile-time construction
//
// `gstring!("...")` resolves to a `const` block.  By the time this benchmark
// runs, `__new` has already executed; the body is just a copy of the
// already-computed `[u8; MAX]` + `usize` from the binary's rodata into a
// local stack slot.
//
// Expected result: near-zero, dominated by the stack copy of the array.
// Compare with `try_new` times to see the compile-time vs runtime split.
// ---------------------------------------------------------------------------

pub fn bench_gstring_macro(c: &mut Criterion) {
    let mut g = c.benchmark_group("construction/gstring_macro");

    // The consts are evaluated once; black_box prevents the compiler from
    // hoisting them out of the loop entirely.
    g.bench_function("short_5b", |b| b.iter(|| black_box(gstring!("hello"))));

    g.bench_function("medium_43b", |b| {
        b.iter(|| black_box(gstring!("the quick brown fox jumps over the lazy dog")))
    });

    // With explicit MAX so the array size matches the string tightly.
    g.bench_function("tight_max_5b", |b| {
        b.iter(|| black_box(gstring!("hello", NoValidation, 0usize, 5usize, false)))
    });

    g.bench_function("ascii_only_flag_5b", |b| {
        b.iter(|| black_box(gstring!("hello", NoValidation, 0usize, 256usize, true)))
    });

    g.finish();
}

// ---------------------------------------------------------------------------
// 3. try_new vs gstring! — direct apples-to-apples for the same string
//
// Puts both in one group so Criterion's HTML report overlays them.
// ---------------------------------------------------------------------------

pub fn bench_macro_vs_runtime(c: &mut Criterion) {
    let mut g = c.benchmark_group("construction/macro_vs_runtime");

    g.bench_function("try_new/5b", |b| {
        b.iter(|| S256::try_new(black_box(S5)).unwrap())
    });

    g.bench_function("gstring_macro/5b", |b| {
        b.iter(|| black_box(gstring!("hello")))
    });

    g.bench_function("try_new/43b", |b| {
        b.iter(|| S256::try_new(black_box(S43)).unwrap())
    });

    g.bench_function("gstring_macro/43b", |b| {
        b.iter(|| black_box(gstring!("the quick brown fox jumps over the lazy dog")))
    });

    g.finish();
}

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
type S50 = GStringNV<0, 50, false>;
type S100 = GStringNV<0, 100, false>;
type S255 = GStringNV<0, 255, false>;
type S500 = GStringNV<0, 500, false>;
type S1000 = GStringNV<0, 1000, false>;

pub fn bench_try_new_vs_string(c: &mut Criterion) {
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
