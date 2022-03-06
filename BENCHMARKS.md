# Benchmarks

## Create and Destroy

`Rc`-based heap creates are about 50% more expensive compared to `String` 
and `Arc`-based approx. twice as slow. Inline/static creation is very fast as 
expected, and are several fold faster than creating a heap-based `String`.

### FlexStr

```
create_static_normal    time:   [3.7062 ns 3.7213 ns 3.7422 ns]
create_inline_small     time:   [3.8932 ns 3.9004 ns 3.9084 ns]
create_heap_rc_normal   time:   [13.533 ns 13.557 ns 13.587 ns]
create_heap_rc_large    time:   [18.605 ns 18.635 ns 18.664 ns]
create_heap_arc_normal  time:   [18.535 ns 18.551 ns 18.568 ns]
create_heap_arc_large   time:   [26.794 ns 26.861 ns 26.937 ns]
```

### Comparables

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

## Clone

Clones are MUCH cheaper than `String` (except when using `Arc`, at which 
point they are only slightly faster, but still save memory).

### FlexStr

```
clone_static_normal     time:   [3.9540 ns 3.9572 ns 3.9610 ns]
clone_inline_small      time:   [4.4717 ns 4.4763 ns 4.4819 ns]
clone_heap_rc_normal    time:   [4.4738 ns 4.4839 ns 4.4965 ns]
clone_heap_arc_normal   time:   [10.596 ns 10.607 ns 10.618 ns]
```

### Comparables

```
clone_string_small      time:   [11.774 ns 11.789 ns 11.807 ns]
clone_string_normal     time:   [12.289 ns 12.422 ns 12.540 ns]
clone_string_large      time:   [14.931 ns 15.013 ns 15.116 ns]
clone_rc_normal         time:   [652.97 ps 653.58 ps 654.30 ps]
clone_arc_normal        time:   [3.2948 ns 3.2986 ns 3.3021 ns]
```

## Conversions

Thanks (mostly) to `itoa` and `ryu` our conversions are much faster than
`String` and it isn't really even close.

### FlexStr

```
convert_bool            time:   [3.7080 ns 3.7094 ns 3.7109 ns]
convert_char            time:   [3.8104 ns 3.8159 ns 3.8222 ns]
convert_i8              time:   [3.2817 ns 3.2827 ns 3.2838 ns]
convert_i16             time:   [3.5285 ns 3.5379 ns 3.5511 ns]
convert_i32             time:   [10.568 ns 10.575 ns 10.582 ns]
convert_i64             time:   [7.6351 ns 7.6390 ns 7.6430 ns]
convert_i128            time:   [38.756 ns 38.787 ns 38.821 ns]
convert_f32             time:   [24.669 ns 24.692 ns 24.721 ns]
convert_f64             time:   [33.105 ns 33.145 ns 33.184 ns]
```

### Comparables

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

## Operations

### FlexStr

Formatting is a little faster with inline and a little slower with heap 
based, but roughly the same. I suspect `format_args!` dominates the time
and is known to be slow, and they both use it.

Addition is surprisingly slow on both inline and static strings. 
That code path will need to be looked at for optimizations. Heap additions 
are somewhat slower as well.

Repetition of strings is more or less the same.

```
format_inline_short     time:    [47.147 ns 47.566 ns 48.096 ns]
format_heap_rc_long     time:    [83.948 ns 84.067 ns 84.192 ns]
format_heap_arc_long    time:    [87.600 ns 87.900 ns 88.477 ns]
add_static_small        time:    [32.212 ns 32.262 ns 32.304 ns]
add_inline_small        time:    [17.247 ns 17.271 ns 17.295 ns]
add_heap_rc_normal      time:    [56.522 ns 56.796 ns 57.142 ns]
add_heap_arc_normal     time:    [56.504 ns 56.539 ns 56.574 ns]
repeat_inline_tiny10    time:    [27.500 ns 27.576 ns 27.680 ns]
repeat_heap_rc_normal10 time:    [47.161 ns 47.228 ns 47.294 ns]
repeat_heap_arc_normal10 time:   [46.882 ns 46.935 ns 46.993 ns]
```

### Comparables

```
format_string_short     time:   [53.822 ns 54.099 ns 54.412 ns]
format_string_long      time:   [72.984 ns 73.308 ns 73.802 ns]
add_string_small        time:   [16.496 ns 16.541 ns 16.586 ns]
add_string_normal       time:   [32.962 ns 33.044 ns 33.132 ns]
repeat_string_tiny10    time:   [27.783 ns 27.800 ns 27.816 ns]
repeat_string_normal10  time:   [44.734 ns 44.837 ns 44.953 ns]
```
