use std::rc::Rc;
use std::sync::Arc;

use criterion::{criterion_group, criterion_main, Criterion};
use flexstr::{IntoFlexStr, ToAFlexStr, ToFlexStr};

const SMALL_STR: &str = "Inline";
const NORMAL_STR: &str = "This is a normal type string. It is a typical size for a basic message.";
// A little over 900 chars
const LARGE_STR: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod \
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud \
exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in \
reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint \
occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum. \
Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore \
et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut \
aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse \
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in \
culpa qui officia deserunt mollit anim id est laborum.";

pub fn clone(c: &mut Criterion) {
    // Static and Inline
    let static_str = NORMAL_STR.into_flex_str();
    assert!(static_str.is_static());
    let inline_str = SMALL_STR.to_flex_str();
    assert!(inline_str.is_inlined());

    c.bench_function("clone_static_normal", |b| b.iter(|| static_str.clone()));
    c.bench_function("clone_inline_small", |b| b.iter(|| inline_str.clone()));

    // Heap
    let heap_str = NORMAL_STR.to_flex_str();
    assert!(heap_str.is_heap());
    c.bench_function("clone_heap_normal", |b| b.iter(|| heap_str.clone()));

    // Heap (Arc)
    let a_heap_str = NORMAL_STR.to_a_flex_str();
    assert!(a_heap_str.is_heap());
    c.bench_function("clone_heap_arc_normal", |b| b.iter(|| a_heap_str.clone()));

    // String
    let sm_string = SMALL_STR.to_string();
    let string = NORMAL_STR.to_string();
    let lg_string = LARGE_STR.to_string();

    c.bench_function("clone_string_small", |b| b.iter(|| sm_string.clone()));
    c.bench_function("clone_string_normal", |b| b.iter(|| string.clone()));
    c.bench_function("clone_string_large", |b| b.iter(|| lg_string.clone()));

    // Rc
    let rc: Rc<str> = Rc::from(NORMAL_STR);
    c.bench_function("clone_rc_normal", |b| b.iter(|| Rc::clone(&rc)));

    // Arc
    let arc: Arc<str> = Arc::from(NORMAL_STR);
    c.bench_function("clone_arc_normal", |b| b.iter(|| Arc::clone(&arc)));
}

criterion_group!(benches, clone);
criterion_main!(benches);
