# Benchmarks

## Table of Contents

- [Environment](#environment)
- [Third Party Crates](#third-party-crates)
- [Benchmark Pitfalls](#benchmark-pitfalls)
- [Benchmark Results](#benchmark-results)
    - [Create and Destroy - Literal](#create-and-destroy---literal)
    - [Create and Destroy - Computed](#create-and-destroy---computed)
    - [Clone - Literal](#clone---literal)
    - [Clone - Computed](#clone---computed)
    - [Convert](#convert)

## Environment

Benchmarks were run on:

```bash
$ cargo +nightly version -v
cargo 1.61.0-nightly (65c8266 2022-03-09)
release: 1.61.0-nightly
commit-hash: 65c82664263feddc5fe2d424be0993c28d46377a
commit-date: 2022-03-09
host: x86_64-unknown-linux-gnu
libgit2: 1.4.1 (sys:0.14.1 vendored)
libcurl: 7.80.0-DEV (sys:0.4.51+curl-7.80.0 vendored ssl:OpenSSL/1.1.1m)
os: Pop!_OS 20.04 (focal) [64-bit]
```

## Third Party Crates

I decided to include some popular 3rd party crates to include in the create/clone benchmarks. I try to be as fair as
possible, but I'm obviously inherently biased and even the choice of tests is biased. I'm primarily doing this as yet
another measuring stick on whether `FlexStr` is performing adequately or not (and now I see some places it is not).

Here are the 3rd party crates and their versions under test:

```
compact_str v0.3.1
flexstr v0.8.0
kstring v1.0.6
smartstring v1.0.0
smol_str v0.1.21
```


## Benchmark Pitfalls

The more testing I do, the more I'm convinced that microbenchmarks are nearly impossible to do accurately. I find
that simply moving the order of the benchmarks can make LARGE differences. I also see sudden large jumps and drop offs
even without doing anything at all. Using `black_box` seems just as sketchy, and without writing a book, find it creates
the same problems just skewed slightly differently. Nevermind `nightly` vs `stable`. Due to all this, please take these
with a grain of salt (maybe 5). Many of these are inaccurate or just plain wrong.

That said, they are not totally worthless. We can look for trends and patterns, and benchmarking has revealed some
performance surprises that resulted in beneficial code changes. The key things to watch for are (these will also give clues
when benchmarks are impossibly wrong):

* At size 0, we would expect "empty string" detection to kick in - should be as fast as a constant more or less
* For literal tests, we are just testing how fast we can return the item - all work done at compile time
* For sizes 22 and under (on 64-bit), these are all inlined - we would expect these to be faster than heap allocations
* For static and inlining, there are zero code differences between `FlexStr` and `AFlexStr` and we shouldn't expect any
performance difference at all (even though large differences are shown often!)

## Benchmark Results

### Create and Destroy - Literal

This just demonstrates the benefits of having a constant vs. heap allocating the constant as `String` is forced to do

|          | `String`                | `AFlexStr`                     | `FlexStr`                        |
|:---------|:------------------------|:-------------------------------|:-------------------------------- |
| **`40`** | `8.21 ns` (1.00x)       | `1.06 ns` (✅ **7.74x faster**) | `0.57 ns` (✅ **14.31x faster**)  |

### Create and Destroy - Computed

* String sizes of 10 and 20 are inlining, so gets a boost
* An ever so slight penalty for the wrapper on heap allocations
* String sizes of 0 are just empty string constants and any variation here is likely not meaningful
* Overall I'm happy with how `FlexStr` performs here on inline string creation. I suspect the difference, however, to
 `CompactStr` and probably `SmartString` is just noise

|             | `String`                  | `Rc<str>`                        | `Arc<str>`                       | `FlexStr`                        | `AFlexStr`                       | `CompactStr`                     | `KString`                        | `SmartString`                    | `SmolStr`                         |
|:------------|:--------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:--------------------------------- |
| **`0`**     | `0.43 ns` (1.00x)         | `10.15 ns` (❌ *23.78x slower*)   | `10.62 ns` (❌ *24.87x slower*)   | `1.05 ns` (❌ *2.47x slower*)     | `0.64 ns` (❌ *1.50x slower*)     | `1.27 ns` (❌ *2.99x slower*)     | `6.79 ns` (❌ *15.91x slower*)    | `7.19 ns` (❌ *16.84x slower*)    | `11.02 ns` (❌ *25.82x slower*)    |
| **`10`**    | `9.47 ns` (1.00x)         | `9.96 ns` (❌ *1.05x slower*)     | `10.58 ns` (❌ *1.12x slower*)    | `4.87 ns` (✅ **1.94x faster**)   | `6.59 ns` (✅ **1.44x faster**)   | `7.15 ns` (✅ **1.32x faster**)   | `7.46 ns` (✅ **1.27x faster**)   | `9.33 ns` (✅ **1.02x faster**)   | `13.59 ns` (❌ *1.43x slower*)     |
| **`20`**    | `9.38 ns` (1.00x)         | `9.70 ns` (❌ *1.03x slower*)     | `10.35 ns` (❌ *1.10x slower*)    | `6.27 ns` (✅ **1.49x faster**)   | `6.57 ns` (✅ **1.43x faster**)   | `6.27 ns` (✅ **1.50x faster**)   | `9.79 ns` (❌ *1.04x slower*)     | `9.20 ns` (✅ **1.02x faster**)   | `13.83 ns` (❌ *1.47x slower*)     |
| **`100`**   | `9.81 ns` (1.00x)         | `10.21 ns` (❌ *1.04x slower*)    | `10.99 ns` (❌ *1.12x slower*)    | `10.55 ns` (❌ *1.08x slower*)    | `10.76 ns` (❌ *1.10x slower*)    | `12.40 ns` (❌ *1.26x slower*)    | `10.42 ns` (❌ *1.06x slower*)    | `16.40 ns` (❌ *1.67x slower*)    | `20.04 ns` (❌ *2.04x slower*)     |
| **`1000`**  | `13.28 ns` (1.00x)        | `13.94 ns` (❌ *1.05x slower*)    | `14.40 ns` (❌ *1.08x slower*)    | `14.24 ns` (❌ *1.07x slower*)    | `14.34 ns` (❌ *1.08x slower*)    | `14.97 ns` (❌ *1.13x slower*)    | `13.70 ns` (❌ *1.03x slower*)    | `22.58 ns` (❌ *1.70x slower*)    | `25.00 ns` (❌ *1.88x slower*)     |
| **`16384`** | `135.56 ns` (1.00x)       | `136.80 ns` (❌ *1.01x slower*)   | `316.11 ns` (❌ *2.33x slower*)   | `142.12 ns` (❌ *1.05x slower*)   | `194.22 ns` (❌ *1.43x slower*)   | `188.73 ns` (❌ *1.39x slower*)   | `136.72 ns` (❌ *1.01x slower*)   | `197.77 ns` (❌ *1.46x slower*)   | `200.70 ns` (❌ *1.48x slower*)    |

### Clone - Literal

This again just demonstrates the benefits of having a constant vs. heap allocating the constant as `String` is forced to do.
`AFlexStr` being much slower here is likely not correct in the real world as the code is identical to `FlexStr`

|          | `String`                 | `FlexStr`                      | `AFlexStr`                       |
|:---------|:-------------------------|:-------------------------------|:-------------------------------- |
| **`40`** | `11.85 ns` (1.00x)       | `4.29 ns` (✅ **2.76x faster**) | `11.93 ns` (❌ *1.01x slower*)    |

### Clone - Computed

* The benefits of simply copying a wrapper and possibly a ref count increment are apparent in `FlexStr`
* The 10 and 20 sizes being 4x slower makes zero sense - this is compiler derived `Clone` code that literally does one
less step than `Rc` derived `Clone` code, so we would expect it to be the same or faster. We also don't see this deviation
 in `AFlexStr`
* `AFlexStr` is nothing more than an enum wrapper over `Arc<str>` for sizes 100 and above, so it being ~5x slower
than a plain `Arc<str>` is very odd to say the least
* `FlexStr` falls pretty hard to `CompactStr` and `SmartString` on inline cloning. I'm not sure why, but I will be looking
into this
* At higher string sizes, the benefits of `Rc` and `Arc` used in `FlexStr` is very benefitical

|             | `String`                  | `Rc<str>`                       | `Arc<str>`                      | `FlexStr`                        | `AFlexStr`                       | `CompactStr`                     | `KString`                        | `SmartString`                    | `SmolStr`                         |
|:------------|:--------------------------|:--------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:--------------------------------- |
| **`0`**     | `5.83 ns` (1.00x)         | `0.69 ns` (✅ **8.47x faster**)  | `4.46 ns` (✅ **1.31x faster**)  | `8.01 ns` (❌ *1.37x slower*)     | `12.33 ns` (❌ *2.12x slower*)    | `3.56 ns` (✅ **1.64x faster**)   | `14.96 ns` (❌ *2.57x slower*)    | `3.64 ns` (✅ **1.60x faster**)   | `12.33 ns` (❌ *2.12x slower*)     |
| **`10`**    | `11.62 ns` (1.00x)        | `0.77 ns` (✅ **15.07x faster**) | `4.84 ns` (✅ **2.40x faster**)  | `8.12 ns` (✅ **1.43x faster**)   | `12.40 ns` (❌ *1.07x slower*)    | `3.50 ns` (✅ **3.32x faster**)   | `14.99 ns` (❌ *1.29x slower*)    | `2.47 ns` (✅ **4.71x faster**)   | `12.47 ns` (❌ *1.07x slower*)     |
| **`20`**    | `11.42 ns` (1.00x)        | `0.84 ns` (✅ **13.66x faster**) | `4.78 ns` (✅ **2.39x faster**)  | `8.15 ns` (✅ **1.40x faster**)   | `12.39 ns` (❌ *1.08x slower*)    | `3.49 ns` (✅ **3.27x faster**)   | `15.48 ns` (❌ *1.36x slower*)    | `2.41 ns` (✅ **4.74x faster**)   | `12.42 ns` (❌ *1.09x slower*)     |
| **`100`**   | `12.61 ns` (1.00x)        | `1.49 ns` (✅ **8.48x faster**)  | `5.11 ns` (✅ **2.47x faster**)  | `2.00 ns` (✅ **6.29x faster**)   | `12.97 ns` (❌ *1.03x slower*)    | `14.63 ns` (❌ *1.16x slower*)    | `17.73 ns` (❌ *1.41x slower*)    | `16.70 ns` (❌ *1.32x slower*)    | `16.76 ns` (❌ *1.33x slower*)     |
| **`1000`**  | `53.93 ns` (1.00x)        | `3.06 ns` (✅ **17.61x faster**) | `7.52 ns` (✅ **7.17x faster**)  | `2.89 ns` (✅ **18.65x faster**)  | `13.02 ns` (✅ **4.14x faster**)  | `42.74 ns` (✅ **1.26x faster**)  | `56.43 ns` (❌ *1.05x slower*)    | `56.79 ns` (❌ *1.05x slower*)    | `16.79 ns` (✅ **3.21x faster**)   |
| **`16384`** | `534.58 ns` (1.00x)       | `6.20 ns` (✅ **86.21x faster**) | `7.72 ns` (✅ **69.23x faster**) | `2.60 ns` (✅ **205.65x faster**) | `13.16 ns` (✅ **40.63x faster**) | `467.72 ns` (✅ **1.14x faster**) | `506.98 ns` (✅ **1.05x faster**) | `498.55 ns` (✅ **1.07x faster**) | `18.33 ns` (✅ **29.17x faster**)  |

### Convert

Thanks mostly to `ryu` and `itoa`, our primitive conversions handily outperforms `String`

|            | `String`                  | `AFlexStr`                      | `FlexStr`                        |
|:-----------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`bool`** | `16.58 ns` (1.00x)        | `1.06 ns` (✅ **15.63x faster**) | `0.67 ns` (✅ **24.83x faster**)  |
| **`char`** | `10.67 ns` (1.00x)        | `11.46 ns` (❌ *1.07x slower*)   | `13.45 ns` (❌ *1.26x slower*)    |
| **`i8`**   | `13.32 ns` (1.00x)        | `8.95 ns` (✅ **1.49x faster**)  | `10.05 ns` (✅ **1.32x faster**)  |
| **`i16`**  | `20.76 ns` (1.00x)        | `18.05 ns` (✅ **1.15x faster**) | `18.10 ns` (✅ **1.15x faster**)  |
| **`i32`**  | `31.73 ns` (1.00x)        | `14.64 ns` (✅ **2.17x faster**) | `14.55 ns` (✅ **2.18x faster**)  |
| **`i64`**  | `38.11 ns` (1.00x)        | `19.27 ns` (✅ **1.98x faster**) | `19.30 ns` (✅ **1.97x faster**)  |
| **`i128`** | `65.98 ns` (1.00x)        | `37.99 ns` (✅ **1.74x faster**) | `37.86 ns` (✅ **1.74x faster**)  |
| **`f32`**  | `112.65 ns` (1.00x)       | `24.85 ns` (✅ **4.53x faster**) | `25.05 ns` (✅ **4.50x faster**)  |
| **`f64`**  | `191.50 ns` (1.00x)       | `30.81 ns` (✅ **6.22x faster**) | `30.01 ns` (✅ **6.38x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

