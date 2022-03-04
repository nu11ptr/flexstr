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

NOTE: The serde feature is optional and only included when specified.

```toml
[dependencies]
flexstr = { version = "0.4", features = ["serde"] }
```

## Examples

```rust
use flexstr::{flex_fmt, FlexStr, IntoFlexStr, ToCase, ToFlexStr};

fn main() {
  // Use an `into` function to wrap a literal, no allocation or copying
  let static_str = "This will not allocate or copy".into_flex_str();
  assert!(static_str.is_static());

  // Strings up to 22 bytes (on 64-bit) will be inlined automatically 
  // (demo only, use `into` for literals as above)
  let inline_str = "inlined".to_flex_str();
  assert!(inline_str.is_inlined());

  // When a string is too long to be wrapped/inlined, it will heap allocate
  // (demo only, use `into` for literals as above)
  let rc_str = "This is too long to be inlined".to_flex_str();
  assert!(rc_str.is_heap());

  // You can efficiently create a new `FlexStr` (without creating a `String`)
  // This is equivalent to the stdlib `format!` macro
  let inline_str2 = flex_fmt!("in{}", "lined");
  assert!(inline_str2.is_inlined());
  assert_eq!(inline_str, inline_str2);

  // We can upper/lowercase strings without converting to a `String`
  // This doesn't heap allocate
  let inline_str3: FlexStr = "INLINED".to_ascii_lower();
  assert!(inline_str3.is_inlined());
  assert_eq!(inline_str, inline_str3);

  // Concatenation doesn't even copy if we can fit it in the inline string
  let inline_str4 = inline_str3 + "!!!";
  assert!(inline_str4.is_inlined());
  assert_eq!(inline_str4, "inlined!!!");
  
  // Clone is almost free, and never allocates
  // (at most it is a ref count increment for heap allocated strings)
  let static_str2 = static_str.clone();
  assert!(static_str2.is_static());

  // Regardless of storage type, these all operate seamlessly together 
  // and choose storage as required
  let heap_str2 = static_str2 + &inline_str;
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
* It is simple to use!

## Types

* `FlexStr` - regular usage 
    * `Heap` storage based on `Rc`
* `AFlexStr`- provides `Send` / `Sync` for multi-threaded use
    * `Heap` storage based on `Arc` 

## Usage

### Hello World

```rust
use flexstr::IntoFlexStr;

fn main() {
  // From literal - no copying or allocation
  let world = "world!".into_flex_str();

  println!("Hello {world}");
}
```

### Conversions

```rust
use flexstr::{IntoAFlexStr, IntoFlexStr, ToFlexStr};

fn main() {
  // From literal - no copying or allocation
  // NOTE: `to_flex_str` will copy, so use `into_flex_str` for literals
  let literal = "literal".into_flex_str();

  // From borrowed string - Copied into inline string
  let owned = "inlined".to_string();
  let str_to_inlined = (&owned).to_flex_str();

  // From borrowed String - copied into `str` wrapped in `Rc`
  let owned = "A bit too long to be inlined!!!".to_string();
  let str_to_wrapped = (&owned).to_flex_str();

  // From String - copied into inline string (`String` storage released)
  let inlined = "inlined".to_string().into_flex_str();

  // From String - `str` wrapped in `Rc` (`String` storage released)
  let counted = "A bit too long to be inlined!!!".to_string().into_flex_str();

  // *** If you want a Send/Sync type you need `AFlexStr` instead ***

  // From FlexStr wrapped literal - no copying or allocation
  let literal = literal.into_a_flex_str();

  // From FlexStr inlined string - no allocation
  let inlined = inlined.into_a_flex_str();

  // From FlexStr `Rc` wrapped `str` - copies into `str` wrapped in `Arc`
  let counted = counted.into_a_flex_str();
}
```

### Passing FlexStr to Conditional Ownership Functions

This has always been a confusing situation in Rust, but it is easy with 
`FlexStr` since multi ownership is cheap.

```rust
use flexstr::{IntoFlexStr, FlexStr};

struct MyStruct {
  s: FlexStr
}

impl MyStruct {
  fn to_own_or_not_to_own(s: &FlexStr) -> Self {
    let s = if s == "own_me" {
      // Since a wrapped literal, no copy or allocation
      s.clone()
    } else {
      // Wrapped literal - no copy or allocation
      "own_me".into()
    };

    Self { s }
  }
}

fn main() {
  // Wrapped literals - no copy or allocation
  let s = "borrow me".into_flex_str();
  let s2 = "own me".into_flex_str();

  let struct1 = MyStruct::to_own_or_not_to_own(&s);
  let struct2 = MyStruct::to_own_or_not_to_own(&s2);

  assert_eq!(s2, struct1.s);
  assert_eq!(s2, struct2.s);
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

Summary: Creates are fairly expensive (yet) compared to `String`, but clones 
are MUCH cheaper (except when using `Arc`).

Keep in mind even though creates are more expensive that 
depending on your workload you may earn that back via clones and it will save
memory as well.

### Create

#### FlexStr

```
create_static_normal    time:   [3.7180 ns 3.7246 ns 3.7311 ns]
create_inline_small     time:   [9.4513 ns 9.4574 ns 9.4643 ns]
create_heap_normal      time:   [13.558 ns 13.577 ns 13.596 ns]
create_heap_large       time:   [19.433 ns 19.451 ns 19.470 ns]
create_heap_arc_normal  time:   [18.400 ns 18.438 ns 18.490 ns]
create_heap_arc_large   time:   [25.057 ns 25.155 ns 25.253 ns]
```

#### Comparables

```
create_string_small     time:   [6.8949 ns 6.9307 ns 6.9868 ns]
create_string_normal    time:   [7.8346 ns 7.8390 ns 7.8441 ns]
create_string_large     time:   [12.852 ns 12.868 ns 12.886 ns]
create_rc_small         time:   [8.0822 ns 8.1364 ns 8.1810 ns]
create_rc_normal        time:   [8.3205 ns 8.3502 ns 8.3816 ns]
create_rc_large         time:   [13.356 ns 13.369 ns 13.384 ns]
create_arc_small        time:   [8.3220 ns 8.3675 ns 8.4364 ns]
create_arc_normal       time:   [8.7265 ns 8.7343 ns 8.7434 ns]
create_arc_large        time:   [13.768 ns 13.816 ns 13.865 ns]
```

### Clone

#### FlexStr

```
clone_static_normal     time:   [3.9540 ns 3.9572 ns 3.9610 ns]
clone_inline_small      time:   [4.4717 ns 4.4763 ns 4.4819 ns]
clone_heap_normal       time:   [4.4738 ns 4.4839 ns 4.4965 ns]
clone_heap_arc_normal   time:   [10.596 ns 10.607 ns 10.618 ns]
```

#### Comparables

```
clone_string_small      time:   [11.774 ns 11.789 ns 11.807 ns]
clone_string_normal     time:   [12.289 ns 12.422 ns 12.540 ns]
clone_string_large      time:   [14.931 ns 15.013 ns 15.116 ns]
clone_rc_normal         time:   [652.97 ps 653.58 ps 654.30 ps]
clone_arc_normal        time:   [3.2948 ns 3.2986 ns 3.3021 ns]
```

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
