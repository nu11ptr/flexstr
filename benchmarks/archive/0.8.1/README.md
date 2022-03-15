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

|          | `String`                | `FlexStr`                       | `AFlexStr`                       |
|:---------|:------------------------|:--------------------------------|:-------------------------------- |
| **`40`** | `7.98 ns` (1.00x)       | `0.55 ns` (🚀 **14.39x faster**) | `0.56 ns` (🚀 **14.32x faster**)  |

### Create and Destroy - Computed

* String sizes of 10 and 20 are inlining, so gets a boost
* An ever so slight penalty for the wrapper on heap allocations
* String sizes of 0 are just empty string constants and any variation here is likely not meaningful
* Overall I'm happy with how `FlexStr` performs here on inline string creation. I suspect the difference, however, to
 `CompactStr` and probably `SmartString` is just noise

|             | `String`                  | `Rc<str>`                        | `Arc<str>`                       | `FlexStr`                        | `AFlexStr`                       | `CompactStr`                     | `KString`                        | `SmartString`                    | `SmolStr`                         |
|:------------|:--------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:--------------------------------- |
| **`0`**     | `0.43 ns` (1.00x)         | `10.05 ns` (❌ *23.57x slower*)   | `10.80 ns` (❌ *25.34x slower*)   | `0.44 ns` (❌ *1.04x slower*)     | `0.43 ns` (❌ *1.02x slower*)     | `1.06 ns` (❌ *2.49x slower*)     | `6.36 ns` (❌ *14.92x slower*)    | `7.20 ns` (❌ *16.89x slower*)    | `11.02 ns` (❌ *25.85x slower*)    |
| **`10`**    | `10.04 ns` (1.00x)        | `10.09 ns` (❌ *1.01x slower*)    | `10.73 ns` (❌ *1.07x slower*)    | `6.11 ns` (✅ **1.64x faster**)   | `6.08 ns` (✅ **1.65x faster**)   | `6.95 ns` (✅ **1.44x faster**)   | `8.53 ns` (✅ **1.18x faster**)   | `9.26 ns` (✅ **1.08x faster**)   | `13.61 ns` (❌ *1.36x slower*)     |
| **`20`**    | `9.59 ns` (1.00x)         | `9.83 ns` (❌ *1.02x slower*)     | `10.44 ns` (❌ *1.09x slower*)    | `4.91 ns` (✅ **1.95x faster**)   | `4.90 ns` (✅ **1.96x faster**)   | `6.32 ns` (✅ **1.52x faster**)   | `9.87 ns` (❌ *1.03x slower*)     | `9.26 ns` (✅ **1.04x faster**)   | `13.58 ns` (❌ *1.42x slower*)     |
| **`100`**   | `10.58 ns` (1.00x)        | `10.60 ns` (❌ *1.00x slower*)    | `11.27 ns` (❌ *1.07x slower*)    | `11.09 ns` (❌ *1.05x slower*)    | `10.98 ns` (❌ *1.04x slower*)    | `12.01 ns` (❌ *1.13x slower*)    | `10.46 ns` (✅ **1.01x faster**)  | `16.78 ns` (❌ *1.59x slower*)    | `20.29 ns` (❌ *1.92x slower*)     |
| **`1000`**  | `13.50 ns` (1.00x)        | `13.90 ns` (❌ *1.03x slower*)    | `14.35 ns` (❌ *1.06x slower*)    | `13.91 ns` (❌ *1.03x slower*)    | `14.10 ns` (❌ *1.04x slower*)    | `14.98 ns` (❌ *1.11x slower*)    | `13.44 ns` (✅ **1.00x faster**)  | `22.63 ns` (❌ *1.68x slower*)    | `25.24 ns` (❌ *1.87x slower*)     |
| **`16384`** | `135.18 ns` (1.00x)       | `135.56 ns` (❌ *1.00x slower*)   | `193.03 ns` (❌ *1.43x slower*)   | `139.29 ns` (❌ *1.03x slower*)   | `193.82 ns` (❌ *1.43x slower*)   | `189.21 ns` (❌ *1.40x slower*)   | `135.17 ns` (✅ **1.00x faster**) | `195.44 ns` (❌ *1.45x slower*)   | `199.77 ns` (❌ *1.48x slower*)    |

### Clone - Literal

This again just demonstrates the benefits of having a constant vs. heap allocating the constant as `String` is forced to do.
`AFlexStr` being much slower here is likely not correct in the real world as the code is identical to `FlexStr`

|          | `String`                 | `FlexStr`                      | `AFlexStr`                      |
|:---------|:-------------------------|:-------------------------------|:------------------------------- |
| **`40`** | `11.90 ns` (1.00x)       | `1.48 ns` (🚀 **8.04x faster**) | `2.30 ns` (🚀 **5.17x faster**)  |

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

|             | `String`                  | `Rc<str>`                       | `Arc<str>`                      | `FlexStr`                        | `AFlexStr`                      | `CompactStr`                     | `KString`                        | `SmartString`                    | `SmolStr`                         |
|:------------|:--------------------------|:--------------------------------|:--------------------------------|:---------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:--------------------------------- |
| **`0`**     | `5.86 ns` (1.00x)         | `0.73 ns` (🚀 **8.08x faster**)  | `4.49 ns` (✅ **1.30x faster**)  | `1.28 ns` (🚀 **4.57x faster**)   | `2.47 ns` (🚀 **2.37x faster**)  | `3.47 ns` (✅ **1.69x faster**)   | `14.71 ns` (❌ *2.51x slower*)    | `4.21 ns` (✅ **1.39x faster**)   | `12.08 ns` (❌ *2.06x slower*)     |
| **`10`**    | `12.26 ns` (1.00x)        | `0.73 ns` (🚀 **16.85x faster**) | `4.71 ns` (🚀 **2.60x faster**)  | `1.11 ns` (🚀 **11.02x faster**)  | `2.32 ns` (🚀 **5.28x faster**)  | `3.43 ns` (🚀 **3.57x faster**)   | `14.80 ns` (❌ *1.21x slower*)    | `4.12 ns` (🚀 **2.98x faster**)   | `12.14 ns` (✅ **1.01x faster**)   |
| **`20`**    | `11.51 ns` (1.00x)        | `0.76 ns` (🚀 **15.23x faster**) | `4.73 ns` (🚀 **2.43x faster**)  | `0.87 ns` (🚀 **13.19x faster**)  | `2.29 ns` (🚀 **5.04x faster**)  | `3.44 ns` (🚀 **3.35x faster**)   | `15.49 ns` (❌ *1.35x slower*)    | `4.02 ns` (🚀 **2.86x faster**)   | `12.17 ns` (❌ *1.06x slower*)     |
| **`100`**   | `12.93 ns` (1.00x)        | `1.33 ns` (🚀 **9.74x faster**)  | `5.12 ns` (🚀 **2.52x faster**)  | `1.64 ns` (🚀 **7.86x faster**)   | `5.43 ns` (🚀 **2.38x faster**)  | `15.12 ns` (❌ *1.17x slower*)    | `17.51 ns` (❌ *1.35x slower*)    | `16.70 ns` (❌ *1.29x slower*)    | `16.27 ns` (❌ *1.26x slower*)     |
| **`1000`**  | `55.12 ns` (1.00x)        | `3.05 ns` (🚀 **18.06x faster**) | `7.46 ns` (🚀 **7.38x faster**)  | `3.08 ns` (🚀 **17.87x faster**)  | `5.77 ns` (🚀 **9.55x faster**)  | `40.18 ns` (✅ **1.37x faster**)  | `54.93 ns` (✅ **1.00x faster**)  | `55.64 ns` (❌ *1.01x slower*)    | `16.39 ns` (🚀 **3.36x faster**)   |
| **`16384`** | `484.42 ns` (1.00x)       | `5.73 ns` (🚀 **84.48x faster**) | `7.41 ns` (🚀 **65.36x faster**) | `2.49 ns` (🚀 **194.42x faster**) | `6.29 ns` (🚀 **77.02x faster**) | `452.33 ns` (✅ **1.07x faster**) | `500.87 ns` (❌ *1.03x slower*)   | `497.66 ns` (❌ *1.03x slower*)   | `17.57 ns` (🚀 **27.56x faster**)  |

### Convert

Thanks mostly to `ryu` and `itoa`, our primitive conversions handily outperforms `String`

|            | `String`                  | `AFlexStr`                      | `FlexStr`                        |
|:-----------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`bool`** | `17.02 ns` (1.00x)        | `1.07 ns` (🚀 **15.87x faster**) | `0.86 ns` (🚀 **19.84x faster**)  |
| **`char`** | `10.55 ns` (1.00x)        | `10.04 ns` (✅ **1.05x faster**) | `10.28 ns` (✅ **1.03x faster**)  |
| **`i8`**   | `12.69 ns` (1.00x)        | `10.01 ns` (✅ **1.27x faster**) | `10.20 ns` (✅ **1.24x faster**)  |
| **`i16`**  | `21.17 ns` (1.00x)        | `10.61 ns` (✅ **1.99x faster**) | `10.63 ns` (✅ **1.99x faster**)  |
| **`i32`**  | `38.49 ns` (1.00x)        | `12.68 ns` (🚀 **3.04x faster**) | `12.67 ns` (🚀 **3.04x faster**)  |
| **`i64`**  | `35.91 ns` (1.00x)        | `12.10 ns` (🚀 **2.97x faster**) | `12.12 ns` (🚀 **2.96x faster**)  |
| **`i128`** | `63.06 ns` (1.00x)        | `34.05 ns` (✅ **1.85x faster**) | `33.45 ns` (✅ **1.89x faster**)  |
| **`f32`**  | `118.53 ns` (1.00x)       | `18.47 ns` (🚀 **6.42x faster**) | `17.36 ns` (🚀 **6.83x faster**)  |
| **`f64`**  | `191.93 ns` (1.00x)       | `30.56 ns` (🚀 **6.28x faster**) | `30.14 ns` (🚀 **6.37x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

