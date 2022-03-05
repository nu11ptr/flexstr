use criterion::{criterion_group, criterion_main, Criterion};
use flexstr::ToFlexStr;

macro_rules! min_nums {
    ($($name:ident, $type:ty),+) => {
        $(const $name: $type = <$type>::MIN;)+
    }
}

min_nums!(I8, i8, I16, i16, I32, i32, I64, i64, I128, i128, F32, f32, F64, f64);

const BOOL: bool = false;
const CHAR: char = '☺';

macro_rules! flex_convert {
    ($($name:literal, $var:ident),+) => {
        pub fn flex_convert(c: &mut Criterion) {
            $(c.bench_function($name, |b| {
                b.iter(|| $var.to_flex_str())
            });)+
        }
    };
}

flex_convert!(
    "convert_bool",
    BOOL,
    "convert_char",
    CHAR,
    "convert_i8",
    I8,
    "convert_i16",
    I16,
    "convert_i32",
    I32,
    "convert_i64",
    I64,
    "convert_i128",
    I128,
    "convert_f32",
    F32,
    "convert_f64",
    F64
);

macro_rules! string_convert {
    ($($name:literal, $var:ident),+) => {
        pub fn string_convert(c: &mut Criterion) {
            $(c.bench_function($name, |b| {
                b.iter(|| $var.to_string())
            });)+
        }
    };
}

string_convert!(
    "convert_string_bool",
    BOOL,
    "convert_string_char",
    CHAR,
    "convert_string_i8",
    I8,
    "convert_string_i16",
    I16,
    "convert_string_i32",
    I32,
    "convert_string_i64",
    I64,
    "convert_string_i128",
    I128,
    "convert_string_f32",
    F32,
    "convert_string_f64",
    F64
);

criterion_group!(benches, flex_convert, string_convert);
criterion_main!(benches);
