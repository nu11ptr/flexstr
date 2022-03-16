# flexstr

[![Crate](https://img.shields.io/crates/v/flexstr?style=for-the-badge)](https://crates.io/crates/flexstr)
[![Docs](https://img.shields.io/docsrs/flexstr?style=for-the-badge)](https://docs.rs/flexstr)

A flexible, simple to use, immutable, clone-efficient `String` replacement for 
Rust. It unifies literals, inlined, and heap allocated strings into a single 
type.

## Overview

Rust is great, but it's `String` type is optimized as a mutable string 
buffer, not for typical string use cases. Most string use cases don't 
modify their contents, often need to copy strings around as if 
they were cheap like integers, typically concatenate instead of modify, and 
often end up being cloned with identical contents. Additionally, `String` 
isn't able to wrap a string literal without additional allocation and copying.

Rust needs a new string type to unify usage of both literals and 
allocated strings in these typical use cases. This crate creates a new string 
type 
that is optimized for those use cases, while retaining the usage simplicity of
`String`.

This type is not inherently "better" than `String`, but different. It 
works best in 'typical' string use cases (immutability, concatenation, cheap 
multi ownership) whereas `String` works better in "string buffer" use cases
(mutability, string building, single ownership).

## Installation

Optional features:
* `fast_format` = enables `flex_ufmt!` and `a_flex_ufmt!` `format!`-like 
  macros for very fast formatting (with some limitations)
* `fp_convert` = Convert floating point types directly into a `FlexStr`
* `int_convert` = Convert integer types directly into a `FlexStr`
* `serde` = Serialization support for `FlexStr`

```toml
[dependencies.flexstr]
version = "0.8"
features = ["fast_format, fp_convert", "int_convert", "serde"]
```

## Examples

```rust
use flexstr::{flex_fmt, flex_str, FlexStr, IntoFlexStr, ToCase, ToFlexStr};

fn main() {
  // Use `flex_str` macro to wrap literals as compile-time constants
  const STATIC_STR: FlexStr = flex_str!("This will not allocate or copy");
  assert!(STATIC_STR.is_static());

  // Strings up to 22 bytes (on 64-bit) will be inlined automatically 
  // (demo only, use macro or `from_static` for literals as above)
  let inline_str = "inlined".to_flex_str();
  assert!(inline_str.is_inline());

  // When a string is too long to be wrapped/inlined, it will heap allocate
  // (demo only, use macro or `from_static` for literals as above)
  let rc_str = "This is too long to be inlined".to_flex_str();
  assert!(rc_str.is_heap());

  // You can efficiently create a new `FlexStr` (without creating a `String`)
  // This is equivalent to the stdlib `format!` macro
  let inline_str2 = flex_fmt!("in{}", "lined");
  assert!(inline_str2.is_inline());
  assert_eq!(inline_str, inline_str2);

  // We can upper/lowercase strings without converting to a `String`
  // This doesn't heap allocate
  let inline_str3: FlexStr = "INLINED".to_ascii_lower();
  assert!(inline_str3.is_inline());
  assert_eq!(inline_str, inline_str3);

  // Concatenation doesn't even copy if we can fit it in the inline string
  let inline_str4 = inline_str3 + "!!!";
  assert!(inline_str4.is_inline());
  assert_eq!(inline_str4, "inlined!!!");
  
  // Clone is cheap, and never allocates
  // (at most it is a ref count increment for heap allocated strings)
  let rc_str2 = rc_str.clone();
  assert!(rc_str2.is_heap());

  // Regardless of storage type, these all operate seamlessly together 
  // and choose storage as required
  let heap_str2 = STATIC_STR + &inline_str;
  assert!(heap_str2.is_heap());
  assert_eq!(heap_str2, "This will not allocate or copyinlined");
}
```

## How Does It Work?

Internally, `FlexStr` uses an enum with these variants:

* `Static` - A simple wrapper around a static string literal (`&'static str`)
* `Inlined` - An inlined string (no heap allocation for small strings)
* `Heap` - A heap allocated (reference counted) string

The type automatically chooses the best storage and allows you to use them 
interchangeably as a single string type.

## Features

* Optimized for immutability and cheap cloning
* Allows for multiple ownership of the same string memory contents
* Serves as a universal string type (unifying literals and allocated strings)
* Doesn't allocate for literals and short strings (64-bit: up to 22 bytes)
* The same size as a `String` (64-bit: 24 bytes)
* Optional `serde` serialization support (feature = "serde")
* Compatible with embedded systems (doesn't use `std`)
* Efficient conditional ownership (borrows can take ownership without 
  allocation/copying)
* Both single threaded compatible (`FlexStr`) and multi-thread safe 
  (`AFlexStr`) options
* All dependencies are optional and based on feature usage
* It is simple to use!

## Types

* `FlexStr` - regular usage 
    * `Heap` storage based on `Rc`
* `AFlexStr`- provides `Send` / `Sync` for multi-threaded use
    * `Heap` storage based on `Arc` 

## Usage

### Hello World

```rust
use flexstr::flex_str;

fn main() {
  // From literal - no copying or allocation
  let world = flex_str!("world!");

  println!("Hello {world}");
}
```

### Creation Scenarios

```rust
use flexstr::{flex_str, FlexStr, IntoAFlexStr, IntoFlexStr, ToFlexStr};

fn main() {
  // From literal - no runtime, all compile-time
  const literal: FlexStr = flex_str!("literal");

  // From borrowed string - Copied into inline string
  let owned = "inlined".to_string();
  let str_to_inlined = owned.to_flex_str();

  // From borrowed String - copied into `str` wrapped in `Rc`
  let owned = "A bit too long to be inlined!!!".to_string();
  let str_to_wrapped = owned.to_flex_str();

  // From String - copied into inline string (`String` storage released)
  let inlined = "inlined".to_string().into_flex_str();

  // From String - `str` wrapped in `Rc` (`String` storage released)
  let counted = "A bit too long to be inlined!!!".to_string().into_flex_str();

  // *** If you want a Send/Sync type you need `AFlexStr` instead ***

  // From FlexStr wrapped literal - no copying or allocation
  let literal2 = literal.into_a_flex_str();

  // From FlexStr inlined string - no allocation
  let inlined = inlined.into_a_flex_str();

  // From FlexStr `Rc` wrapped `str` - copies into `str` wrapped in `Arc`
  let counted = counted.into_a_flex_str();
}
```

### Passing FlexStr to Conditional Ownership Functions

This has always been a confusing situation in Rust, but it is easy with 
`FlexStr` since multi ownership is cheap. By passing as `&FlexStr` instead 
of `&str`, you retain the option for very fast multi ownership.

```rust
use flexstr::{flex_str, IntoFlexStr, FlexStr};

struct MyStruct {
  s: FlexStr
}

impl MyStruct {
  fn to_own_or_not_to_own(s: &FlexStr) -> Self {
    let s = if s == "own me" {
      // Since a wrapped literal, no copy or allocation
      s.clone()
    } else {
      // Wrapped literal - no copy or allocation
      flex_str!("own me")
    };

    Self { s }
  }
}

fn main() {
  // Wrapped literals - compile time constant
  const S: FlexStr = flex_str!("borrow me");
  const S2: FlexStr = flex_str!("own me");

  let struct1 = MyStruct::to_own_or_not_to_own(&S);
  let struct2 = MyStruct::to_own_or_not_to_own(&S2);

  assert_eq!(S2, struct1.s);
  assert_eq!(S2, struct2.s);
}
```

### Make Your Own String Type

All you need to do is pick an inline size (the default `STRING_SIZED_INLINE` 
will result in a type the same size as a `String`) and a storage type. The 
storage type must implement `Deref<Target = str>`, `From<String>`, `From<&str>`, 
and `Clone`. Pretty much all smart pointers do this already.

```rust
use flexstr::{Flex, PTR_SIZED_PAD, Repeat, STRING_SIZED_INLINE, ToFlex};

type BoxFlexStr = Flex<STRING_SIZED_INLINE, PTR_SIZED_PAD, PTR_SIZED_PAD, Box<str>>;

fn main() {
  // This will be allocated in a box instead of the default `Rc`
  // Demo only: normally use `into_flex` with literals just to wrap them
  let my_str: BoxFlexStr = "cool!".to_flex().repeat_n(3);
  assert_eq!(my_str, "cool!cool!cool!");
}
```

## Performance Characteristics

* Clones are cheap and never allocate
    * At minimum, they are just a copy of the enum and at max an additional 
      reference count increment
* Literals are just wrapped when used with `into()` and never copied
* Calling `into()` on a `String` will result in an inline string (if 
  short) otherwise copied into a `str` wrapped in `Rc`/`Arc` 
  (which will allocate, copy, and then release original `String` storage)
* `into_flex_str()` and `into_a_flex_str()` are equivalent to calling `into()` 
  on both literals and `String` (they are present primarily for `let` 
  bindings so there is no need to declare a type)
* `to_flex_str()` and `to_a_flex_str()` are meant for taking ownership of 
  borrowed strings and always copy into either an inline string (for short strings) or 
  an `Rc`/`Arc` wrapped `str` (which will allocate)
* `to_string` always copies into a new `String`
* Conversions back and forth between `AFlexStr` and `FlexStr` using `into()` 
  are cheap when using wrapped literals or inlined strings
    * Inlined strings and wrapped literals just create a new enum wrapper
    * Reference counted wrapped strings will always require an allocation 
      and copy for the  new `Rc` or `Arc`

## Benchmarks

In general, inline/static creates are fast but heap creates are somewhat slower 
than `String`. Clones and conversions from primitive types are much faster. 
Other operations (repeat, additions, etc.) tend to be about the same 
performance, but with some nuance.

[Full benchmarks](benchmarks/README.md)

## Negatives

There is no free lunch:

* Due to usage of `Rc` (or `Arc`), when on-boarding `String` it will need to 
  reallocate and copy
* Due to the enum wrapper, every string operation has the overhead of an extra
  branching operation
* Since `FlexStr` is not `Send` or `Sync`, there is a need to consider 
  single-threaded   (`FlexStr`) and multi-threaded (`AFlexStr`) use cases and 
  convert accordingly

## Status

This is currently beta quality and still needs testing. The API may very 
possibly change but semantic versioning will be followed.

## License

This project is licensed optionally under either:

* Apache License, Version 2.0, (LICENSE-APACHE
  or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license (LICENSE-MIT or https://opensource.org/licenses/MIT)
