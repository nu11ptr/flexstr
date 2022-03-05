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

### Create

Heap creates are fairly expensive still compared to `String` (apparently due 
to the overhead of creating the enum?), `Rc<str>` and`Arc<str>`, but 
inline/static creation is very fast as expected.

#### FlexStr

```
create_static_normal    time:   [3.7062 ns 3.7213 ns 3.7422 ns]
create_inline_small     time:   [3.8932 ns 3.9004 ns 3.9084 ns]
create_heap_normal      time:   [13.533 ns 13.557 ns 13.587 ns]
create_heap_large       time:   [18.605 ns 18.635 ns 18.664 ns]
create_heap_arc_normal  time:   [18.535 ns 18.551 ns 18.568 ns]
create_heap_arc_large   time:   [26.794 ns 26.861 ns 26.937 ns]
```

#### Comparables

```
create_string_small     time:   [7.4377 ns 7.4572 ns 7.4794 ns]
create_string_normal    time:   [8.0550 ns 8.0605 ns 8.0667 ns]
create_string_large     time:   [12.940 ns 12.955 ns 12.973 ns]
create_rc_small         time:   [8.0525 ns 8.0577 ns 8.0639 ns]
create_rc_normal        time:   [8.2438 ns 8.2512 ns 8.2604 ns]
create_rc_large         time:   [13.139 ns 13.153 ns 13.168 ns]
create_arc_small        time:   [8.7128 ns 8.7231 ns 8.7341 ns]
create_arc_normal       time:   [8.7454 ns 8.7851 ns 8.8446 ns]
create_arc_large        time:   [13.827 ns 13.855 ns 13.886 ns]
```

### Clone

Clones are MUCH cheaper than `String` (except when using `Arc`). Interested 
to find out why the enum wrapper and single branch op causes such a large 
differential between the wrapped `Rc<str>`/`Arc<str>` and the raw version.

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

### Conversions

Thanks (mostly) to `itoa` and `ryu` our conversions are much faster than 
`String`.

#### FlexStr

```
convert_bool            time:   [3.6423 ns 3.6436 ns 3.6449 ns]
convert_char            time:   [3.7979 ns 3.7995 ns 3.8012 ns]
convert_i8              time:   [3.1668 ns 3.1744 ns 3.1884 ns]
convert_i16             time:   [17.009 ns 17.039 ns 17.084 ns]
convert_i32             time:   [15.509 ns 15.530 ns 15.553 ns]
convert_i64             time:   [17.836 ns 17.845 ns 17.856 ns]
convert_i128            time:   [38.833 ns 38.872 ns 38.912 ns]
convert_f32             time:   [22.940 ns 22.970 ns 22.999 ns]
convert_f64             time:   [35.064 ns 35.130 ns 35.201 ns]
```

#### Comparables

```
convert_string_bool     time:   [18.466 ns 18.505 ns 18.538 ns]
convert_string_char     time:   [7.2933 ns 7.2966 ns 7.3003 ns]
convert_string_i8       time:   [7.3838 ns 7.4546 ns 7.5457 ns]
convert_string_i16      time:   [23.087 ns 23.477 ns 24.025 ns]
convert_string_i32      time:   [38.577 ns 38.624 ns 38.683 ns]
convert_string_i64      time:   [43.348 ns 43.396 ns 43.446 ns]
convert_string_i128     time:   [71.120 ns 71.174 ns 71.225 ns]
convert_string_f32      time:   [100.24 ns 100.50 ns 100.78 ns]
convert_string_f64      time:   [179.86 ns 180.00 ns 180.14 ns]
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
