use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use flexstr::{a_flex_fmt, flex_fmt, IntoFlexStr, ToAFlexStr, ToFlexStr};

const TINY_STR: &str = "a";
const SMALL_STR: &str = "Inline";
const NORMAL_STR: &str = "This is a normal type string. It is a typical size for a basic message.";

pub fn flex_ops(c: &mut Criterion) {
    // Format
    c.bench_function("format_inline_short", |b| {
        b.iter(|| flex_fmt!("a{}d{}g{}", "bc", "ef", "hij"))
    });
    c.bench_function("format_heap_rc_long", |b| {
        b.iter(|| flex_fmt!("a{}d{}g{}", "bc", "ef", NORMAL_STR))
    });
    c.bench_function("format_heap_arc_long", |b| {
        b.iter(|| a_flex_fmt!("a{}d{}g{}", "bc", "ef", NORMAL_STR))
    });

    // Add
    let static_str = SMALL_STR.into_flex_str();
    c.bench_function("add_static_small", |b| {
        b.iter_batched(|| static_str.clone(), |s| s + "abc", BatchSize::SmallInput)
    });

    let inline_str = SMALL_STR.to_flex_str();
    c.bench_function("add_inline_small", |b| {
        b.iter_batched(|| inline_str.clone(), |s| s + "abc", BatchSize::SmallInput)
    });

    let heap_str = NORMAL_STR.to_flex_str();
    c.bench_function("add_heap_rc_normal", |b| {
        b.iter_batched(
            || heap_str.clone(),
            |s| s + NORMAL_STR,
            BatchSize::SmallInput,
        )
    });

    let a_heap_str = NORMAL_STR.to_a_flex_str();
    c.bench_function("add_heap_arc_normal", |b| {
        b.iter_batched(
            || a_heap_str.clone(),
            |s| s + NORMAL_STR,
            BatchSize::SmallInput,
        )
    });

    // Repeat
    let tiny_str = TINY_STR.into_flex_str(); // Starts as static, but ends up inline
    c.bench_function("repeat_inline_tiny10", |b| b.iter(|| tiny_str.repeat(10)));

    c.bench_function("repeat_heap_rc_normal10", |b| {
        b.iter(|| heap_str.repeat(10))
    });
    c.bench_function("repeat_heap_arc_normal10", |b| {
        b.iter(|| a_heap_str.repeat(10))
    });
}

pub fn string_ops(c: &mut Criterion) {
    // Format
    c.bench_function("format_string_short", |b| {
        b.iter(|| format!("a{}d{}g{}", "bc", "ef", "hij"))
    });
    c.bench_function("format_string_long", |b| {
        b.iter(|| format!("a{}d{}g{}", "bc", "ef", NORMAL_STR))
    });

    // Add
    let small_str = SMALL_STR.to_string();
    c.bench_function("add_string_small", |b| {
        b.iter_batched(|| small_str.clone(), |s| s + "abc", BatchSize::SmallInput)
    });

    let normal_str = NORMAL_STR.to_string();
    c.bench_function("add_string_normal", |b| {
        b.iter_batched(
            || normal_str.clone(),
            |s| s + NORMAL_STR,
            BatchSize::SmallInput,
        )
    });

    // Repeat
    let tiny_str = TINY_STR.to_string();
    c.bench_function("repeat_string_tiny10", |b| b.iter(|| tiny_str.repeat(10)));
    c.bench_function("repeat_string_normal10", |b| {
        b.iter(|| normal_str.repeat(10))
    });
}

criterion_group!(benches, flex_ops, string_ops);
criterion_main!(benches);
