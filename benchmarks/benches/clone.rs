use std::hint::black_box;
use std::rc::Rc;
use std::sync::Arc;

use criterion::{BatchSize, BenchmarkId, Criterion, criterion_group, criterion_main};

const ITERATIONS: usize = 10_000;

macro_rules! clone {
    ($($name:expr, $setup:expr),+) => {
        fn clone(c: &mut Criterion) {
            let mut group = c.benchmark_group("Clone");
            let lengths = vec![0usize, 10, 20, 100, 500];

            for len in lengths {
                $(
                    let id = BenchmarkId::new($name, len);

                    group.bench_function(id, |b| {
                        b.iter_batched(|| $setup(len), |s| {
                            for _ in 0..ITERATIONS {
                                let s2 = s.clone();
                                black_box(&s);
                                black_box(&s2);
                            }
                        }, BatchSize::SmallInput)
                    });
                )+
            }

            group.finish();
        }
    };
}

clone!(
    "String",
    |len| "x".repeat(len),
    "Rc<str>",
    |len| -> Rc<str> { "x".repeat(len).into() },
    "Arc<str>",
    |len| -> Arc<str> { "x".repeat(len).into() },
    "FlexStr 0.9 (LocalStr)",
    |len| -> flexstr09::LocalStr { "x".repeat(len).into() },
    "FlexStr 0.9 (SharedStr)",
    |len| -> flexstr09::SharedStr { "x".repeat(len).into() },
    "InlineFlexStr 0.1 (InlineStr)",
    |len| -> inline_flexstr::InlineStr {
        let len = std::cmp::min(len, inline_flexstr::INLINE_CAPACITY);
        "x".repeat(len).as_str().try_into().unwrap()
    },
    "FlexStr 0.10 (LocalStr - Boxed)",
    |len| -> flexstr::LocalStr { "x".repeat(len).into() },
    "FlexStr 0.10 (SharedStr - Boxed)",
    |len| -> flexstr::SharedStr { "x".repeat(len).into() },
    "FlexStr 0.10 (LocalStr - Optimized)",
    |len| {
        let s: flexstr::LocalStr = "x".repeat(len).into();
        s.optimize()
    },
    "FlexStr 0.10 (SharedStr - Optimized)",
    |len| {
        let s: flexstr::SharedStr = "x".repeat(len).into();
        s.optimize()
    }
);

criterion_group!(benches, clone);
criterion_main!(benches);
