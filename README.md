# flexstr

[![Crate](https://img.shields.io/crates/v/flexstr)](https://crates.io/crates/flexstr)
[![Docs](https://docs.rs/flexstr/badge.svg)](https://docs.rs/flexstr)
[![Build](https://github.com/nu11ptr/flexstr/workflows/CI/badge.svg)](https://github.com/nu11ptr/flexstr/actions)
[![codecov](https://codecov.io/gh/nu11ptr/flexstr/branch/master/graph/badge.svg?token=yUZ8v2tKPd)](https://codecov.io/gh/nu11ptr/flexstr)
[![MSRV](https://img.shields.io/badge/msrv-1.59-blue.svg)](https://crates.io/crates/flexstr)


A flexible, simple to use, immutable, clone-efficient `String` replacement for 
Rust. It unifies literals, inlined, and heap allocated strings into a single 
type.

## Table of Contents

- [Overview](#overview)
- [Example](#example)
- [Installation](#installation)
- [How Does It Work?](#how-does-it-work)
- [Features](#features)
- [Types](#types)
- [Usage](#usage)
    - [Hello World!](#hello-world)
    - [Creation Scenarios](#creation-scenarios)
    - [Passing FlexStr to Conditional Ownership Functions](#passing-flexstr-to-conditional-ownership-functions)
    - [Make Your Own String Type](#make-your-own-string-type)
- [Performance Characteristics](#performance-characteristics)
- [Benchmarks](#benchmarks)
- [Downsides](#downsides)
- [Status](#status)
- [License](#license)

## Overview

Rust is great, but it's `String` type is optimized as a mutable string 
buffer, not for typical string use cases. Most string use cases don't 
modify their contents, often need to copy strings around as if 
they were cheap like integers, typically concatenate instead of modify, and 
often end up being cloned with identical contents. Additionally, `String` 
isn't able to wrap a string literal without additional allocation and 
copying forcing a choice between efficiency and storing two different types.

I believe Rust needs a new string type to unify usage of both literals and 
allocated strings for typical string use cases. This crate includes a new 
string type that is optimized for those use cases, while retaining the usage simplicity of
`String`.

## Example

String constants are easily wrapped into the unified string type. String
data is automatically inlined when possible otherwise allocated on the heap.

See [documentation](https://docs.rs/flexstr) or [Usage](#usage) section for 
more examples.

```rust
use flexstr::{local_str, LocalStr, ToLocalStr};

fn main() {
  // Use `local_str` macro to wrap literals as compile-time constants
  const STATIC_STR: LocalStr = local_str!("This will not allocate or copy");
  assert!(STATIC_STR.is_static());

  // Strings up to 22 bytes (on 64-bit) will be inlined automatically
  // (demo only, use macro or `from_static` for literals as above)
  let inline_str = "inlined".to_local_str();
  assert!(inline_str.is_inline());

  // When a string is too long to be wrapped/inlined, it will heap allocate
  // (demo only, use macro or `from_static` for literals as above)
  let rc_str = "This is too long to be inlined".to_local_str();
  assert!(rc_str.is_heap());
}
```

## Installation

Optional features:
* `fast_format` = enables `local_ufmt!` and `shared_ufmt!` `format!`-like 
  macros for very fast formatting (with some limitations)
* `fp_convert` = Convert floating point types directly into a `FlexStr`
* `int_convert` = Convert integer types directly into a `FlexStr`
* `serde` = Serialization support for `FlexStr`
* `std` = enabled by default (use `default-features=false` to enable `#[no_std]`)

```toml
[dependencies.flexstr]
version = "0.9"
features = ["fast_format", "fp_convert", "int_convert", "serde"]
```

## How Does It Work?

Internally, `FlexStr` uses a union with these variants:

* `Static` - A simple wrapper around a static string literal (`&'static str`)
* `Inline` - An inlined string (no heap allocation for small strings)
* `Heap` - A heap allocated (reference counted) string

The type automatically chooses the best storage and allows you to use them 
interchangeably as a single string type.

## Features

* Optimized for immutability and cheap cloning
* Allows for multiple ownership of the same string memory contents
* Serves as a universal string type (unifying literals and allocated strings)
* Doesn't allocate for literals and short strings (64-bit: up to 22 bytes)
* The same inline size as a `String` (64-bit: 24 bytes)
* Optional `serde` serialization support (feature = "serde")
* Compatible with embedded systems (supports `#[no_std]`)
* Efficient conditional ownership (borrows can take ownership without 
  allocation/copying)
* Both single threaded compatible (`LocalStr`) and multi-thread safe 
  (`SharedStr`) options
* All dependencies are optional and based on feature usage
* It is simple to use!

## Types

NOTE: Both types are identical in handling both literals and inline strings.
The only difference occurs when a heap allocation is required.

* `LocalStr` - ultra-fast usage in the local thread
    * `Heap` storage based on `Rc`
* `SharedStr`- provides `Send` / `Sync` for multithreaded use
    * `Heap` storage based on `Arc` 

## Usage

### Hello World

```rust
use flexstr::local_str;

fn main() {
  // From literal - no copying or allocation
  let world = local_str!("world!");

  println!("Hello {world}");
}
```

### Creation Scenarios

```rust
use flexstr::{local_str, LocalStr, IntoSharedStr, IntoLocalStr, ToLocalStr};

fn main() {
  // From literal - no runtime, all compile-time
  const literal: LocalStr = local_str!("literal");

  // From borrowed string - Copied into inline string
  let owned = "inlined".to_string();
  let str_to_inlined = owned.to_local_str();

  // From borrowed String - copied into `str` wrapped in `Rc`
  let owned = "A bit too long to be inlined!!!".to_string();
  let str_to_wrapped = owned.to_local_str();

  // From String - copied into inline string (`String` storage released)
  let inlined = "inlined".to_string().into_local_str();

  // From String - `str` wrapped in `Rc` (`String` storage released)
  let counted = "A bit too long to be inlined!!!".to_string().into_local_str();

  // *** If you want a Send/Sync type you need `SharedStr` instead ***

  // From LocalStr wrapped literal - no copying or allocation
  let literal2 = literal.into_shared_str();

  // From LocalStr inlined string - no allocation
  let inlined = inlined.into_shared_str();

  // From LocalStr `Rc` wrapped `str` - copies into `str` wrapped in `Arc`
  let counted = counted.into_shared_str();
}
```

### Passing FlexStr to Conditional Ownership Functions

This has always been a confusing situation in Rust, but it is easy with 
`FlexStr` since multi ownership is cheap. By passing as `&LocalStr` instead 
of `&str`, you retain the option for very fast multi ownership.

```rust
use flexstr::{local_str, IntoLocalStr, LocalStr};

struct MyStruct {
  s: LocalStr
}

impl MyStruct {
  fn to_own_or_not_to_own(s: &LocalStr) -> Self {
    let s = if s == "own me" {
      // Since a wrapped literal, no copy or allocation
      s.clone()
    } else {
      // Wrapped literal - no copy or allocation
      local_str!("own me")
    };

    Self { s }
  }
}

fn main() {
  // Wrapped literals - compile time constant
  const S: LocalStr = local_str!("borrow me");
  const S2: LocalStr = local_str!("own me");

  let struct1 = MyStruct::to_own_or_not_to_own(&S);
  let struct2 = MyStruct::to_own_or_not_to_own(&S2);

  assert_eq!(S2, struct1.s);
  assert_eq!(S2, struct2.s);
}
```

### Make Your Own String Type

All you need to do is pick a storage type. The storage type must implement 
`Deref<Target = str>`, `From<&str>`, and `Clone`. Pretty much all smart 
pointers do this already.

#### NOTE:

> Custom concrete types need to specify a heap type with an exact size of two 
> machine words (16 bytes on 64-bit, and 8 bytes on 32-bit). Any other size 
> parameter will result in a runtime panic error message on string creation.

```rust
use flexstr::{FlexStrBase, Repeat, ToFlex};

type BoxStr = FlexStrBase<Box<str>>;

fn main() {
  // Any need for a heap string will now be allocated in a `Box` instead of `Rc`
  // However, the below uses static and inline storage...because we can!
  let my_str = BoxStr::from_static("cool!").repeat_n(3);
  assert_eq!(my_str, "cool!cool!cool!");
}
```

## Performance Characteristics

* Clones are cheap and never allocate
    * At minimum, they are just a copy of the union and at max an additional 
      reference count increment
* Literals are just wrapped when used with `into()` and never copied
* Calling `into()` on a `String` will result in an inline string (if 
  short) otherwise copied into a `str` wrapped in `Rc`/`Arc` 
  (which will allocate, copy, and then release original `String` storage)
* `into_local_str()` and `into_shared_str()` are equivalent to calling `into()` 
  on both literals and `String` (they are present primarily for `let` 
  bindings so there is no need to declare a type)
* `to_local_str()` and `to_shared_str()` are meant for taking ownership of 
  borrowed strings and always copy into either an inline string (for short strings) or 
  an `Rc`/`Arc` wrapped `str` (which will allocate)
* `to_string` always copies into a new `String`
* Conversions back and forth between `SharedStr` and `LocalStr` using `into()` 
  are cheap when using wrapped literals or inlined strings
    * Inlined strings and wrapped literals just create a new union wrapper
    * Reference counted wrapped strings will always require an allocation 
      and copy for the  new `Rc` or `Arc`

## Benchmarks

In general, inline/static creates are fast but heap creates are a tiny bit 
slower than `String`. Clones are MUCH faster and don't allocate/copy. Other 
operations (repeat, additions, etc.) tend to be about the same performance, 
but with some nuance depending on string size.

[Full benchmarks](benchmarks/README.md)

## Downsides

There is no free lunch:

* Due to usage of `Rc` (or `Arc`), when on-boarding `String` it will need to 
  reallocate and copy
* Due to the union wrapper, every string operation has the overhead of an extra
  branching operation
* Since `LocalStr` is not `Send` or `Sync`, there is a need to consider 
  single-threaded   (`LocalStr`) and multi-threaded (`SharedStr`) use cases and 
  convert accordingly

## Status

This is currently beta quality and still needs testing. The API may very 
possibly change but semantic versioning will be followed.

## License

This project is licensed optionally under either:

* Apache License, Version 2.0, (LICENSE-APACHE
  or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license (LICENSE-MIT or https://opensource.org/licenses/MIT)
