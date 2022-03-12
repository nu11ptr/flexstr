# Benchmarks

The more testing I do, the more I'm convinced that microbenchmarks are nearly impossible to do accurately. I find
that simply moving the order of the benchmarks can make LARGE differences. I also see sudden large jumps and drop offs
even without doing anythign at all. Using `black_box` seems just as sketchy, and without writing a book, find it creates
the same problems just skewed slightly differently. Due to all this, please take these with a grain of salt (maybe 5).
Many of these are inaccurate or just plaing wrong.

That said, they are not totally worthless. We can look for trends and patterns, and benchmarking has revealed some
performance surprises that resulted in benefitial code changes. The key things to watch for (these will also give clues
when benchmarks are impossibly wrong):

* At size 0, we would expect "empty string" detection to kick in - should be as fast as a constant more or less
* For literal tests, we are just testing how fast we can return the item - all work done at compile time
* For sizes 22 and under (on 64-bit), these are all inlined - we would expect these to be faster than heap allocations
* For static and inlining, there zero differences between `FlexStr` and `AFlexStr` and we shouldn't expect any performance
difference at all (even those large differences are shown at times)

- [Create and Destroy (Literal)](#create-and-destroy-(literal))
- [Create and Destroy (Computed)](#create-and-destroy-(computed))
- [Clone (Literal)](#clone-(literal))
- [Clone](#clone)
- [Convert](#convert)

## Create and Destroy (Literal)

|          | `String`                | `AFlexStr`                     | `FlexStr`                       |
|:---------|:------------------------|:-------------------------------|:------------------------------- |
| **`40`** | `8.02 ns` (1.00x)       | `1.07 ns` (✅ **7.53x faster**) | `1.07 ns` (✅ **7.53x faster**)  |

## Create and Destroy (Computed)

|             | `String`                  | `Rc<str>`                        | `Arc<str>`                       | `AFlexStr`                       | `FlexStr`                         |
|:------------|:--------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:--------------------------------- |
| **`0`**     | `0.63 ns` (1.00x)         | `9.89 ns` (❌ *15.59x slower*)    | `10.28 ns` (❌ *16.21x slower*)   | `2.78 ns` (❌ *4.38x slower*)     | `2.79 ns` (❌ *4.39x slower*)      |
| **`10`**    | `9.93 ns` (1.00x)         | `10.07 ns` (❌ *1.01x slower*)    | `10.35 ns` (❌ *1.04x slower*)    | `4.88 ns` (✅ **2.04x faster**)   | `6.33 ns` (✅ **1.57x faster**)    |
| **`20`**    | `9.76 ns` (1.00x)         | `9.76 ns` (❌ *1.00x slower*)     | `10.18 ns` (❌ *1.04x slower*)    | `5.59 ns` (✅ **1.74x faster**)   | `6.22 ns` (✅ **1.57x faster**)    |
| **`100`**   | `10.60 ns` (1.00x)        | `10.39 ns` (✅ **1.02x faster**)  | `10.80 ns` (❌ *1.02x slower*)    | `10.84 ns` (❌ *1.02x slower*)    | `10.49 ns` (✅ **1.01x faster**)   |
| **`1000`**  | `13.50 ns` (1.00x)        | `13.96 ns` (❌ *1.03x slower*)    | `15.04 ns` (❌ *1.11x slower*)    | `15.03 ns` (❌ *1.11x slower*)    | `14.24 ns` (❌ *1.06x slower*)     |
| **`16384`** | `137.04 ns` (1.00x)       | `138.89 ns` (❌ *1.01x slower*)   | `187.91 ns` (❌ *1.37x slower*)   | `141.49 ns` (❌ *1.03x slower*)   | `184.11 ns` (❌ *1.34x slower*)    |

## Clone (Literal)

|          | `String`                 | `AFlexStr`                      | `FlexStr`                       |
|:---------|:-------------------------|:--------------------------------|:------------------------------- |
| **`40`** | `11.60 ns` (1.00x)       | `15.38 ns` (❌ *1.33x slower*)   | `4.31 ns` (✅ **2.69x faster**)  |

## Clone

|             | `String`                  | `Rc<str>`                       | `Arc<str>`                      | `AFlexStr`                       | `FlexStr`                         |
|:------------|:--------------------------|:--------------------------------|:--------------------------------|:---------------------------------|:--------------------------------- |
| **`0`**     | `6.02 ns` (1.00x)         | `0.71 ns` (✅ **8.52x faster**)  | `4.54 ns` (✅ **1.33x faster**)  | `14.06 ns` (❌ *2.33x slower*)    | `7.99 ns` (❌ *1.33x slower*)      |
| **`10`**    | `11.87 ns` (1.00x)        | `0.71 ns` (✅ **16.65x faster**) | `4.78 ns` (✅ **2.48x faster**)  | `14.12 ns` (❌ *1.19x slower*)    | `8.12 ns` (✅ **1.46x faster**)    |
| **`20`**    | `11.74 ns` (1.00x)        | `0.73 ns` (✅ **16.03x faster**) | `4.73 ns` (✅ **2.48x faster**)  | `14.04 ns` (❌ *1.20x slower*)    | `8.10 ns` (✅ **1.45x faster**)    |
| **`100`**   | `12.49 ns` (1.00x)        | `1.36 ns` (✅ **9.15x faster**)  | `5.12 ns` (✅ **2.44x faster**)  | `16.00 ns` (❌ *1.28x slower*)    | `1.96 ns` (✅ **6.37x faster**)    |
| **`1000`**  | `51.74 ns` (1.00x)        | `2.84 ns` (✅ **18.22x faster**) | `7.60 ns` (✅ **6.81x faster**)  | `16.01 ns` (✅ **3.23x faster**)  | `2.63 ns` (✅ **19.70x faster**)   |
| **`16384`** | `440.94 ns` (1.00x)       | `5.27 ns` (✅ **83.66x faster**) | `7.27 ns` (✅ **60.65x faster**) | `16.16 ns` (✅ **27.28x faster**) | `2.65 ns` (✅ **166.14x faster**)  |

## Convert

|            | `String`                  | `AFlexStr`                      | `FlexStr`                        |
|:-----------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`bool`** | `17.15 ns` (1.00x)        | `1.07 ns` (✅ **16.08x faster**) | `0.67 ns` (✅ **25.63x faster**)  |
| **`char`** | `10.69 ns` (1.00x)        | `11.50 ns` (❌ *1.08x slower*)   | `13.45 ns` (❌ *1.26x slower*)    |
| **`i8`**   | `12.86 ns` (1.00x)        | `8.94 ns` (✅ **1.44x faster**)  | `10.15 ns` (✅ **1.27x faster**)  |
| **`i16`**  | `21.40 ns` (1.00x)        | `14.27 ns` (✅ **1.50x faster**) | `16.87 ns` (✅ **1.27x faster**)  |
| **`i32`**  | `31.68 ns` (1.00x)        | `11.12 ns` (✅ **2.85x faster**) | `13.07 ns` (✅ **2.42x faster**)  |
| **`i64`**  | `35.91 ns` (1.00x)        | `15.49 ns` (✅ **2.32x faster**) | `17.25 ns` (✅ **2.08x faster**)  |
| **`i128`** | `63.72 ns` (1.00x)        | `33.81 ns` (✅ **1.88x faster**) | `33.74 ns` (✅ **1.89x faster**)  |
| **`f32`**  | `112.45 ns` (1.00x)       | `21.79 ns` (✅ **5.16x faster**) | `21.70 ns` (✅ **5.18x faster**)  |
| **`f64`**  | `172.28 ns` (1.00x)       | `30.79 ns` (✅ **5.60x faster**) | `29.92 ns` (✅ **5.76x faster**)  |

Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

