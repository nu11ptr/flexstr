use std::{
    alloc::{GlobalAlloc, Layout, System},
    cell::Cell,
    fmt::Debug,
    ops::Deref,
};

use inline_flexstr::INLINE_CAPACITY;
#[cfg(any(feature = "bytes", feature = "cstr"))]
use serde::de::value::SeqDeserializer;
use serde::de::value::{BorrowedBytesDeserializer, BytesDeserializer, Error};
#[cfg(any(feature = "str", feature = "cstr", feature = "path"))]
use serde::de::value::{BorrowedStrDeserializer, StrDeserializer};
use serde::{Deserialize, Deserializer, de::DeserializeOwned};

thread_local! {
    static ALLOCATIONS: Cell<Option<usize>> = const { Cell::new(None) };
}

struct CountingAllocator;

fn record_allocation() {
    let _ = ALLOCATIONS.try_with(|count| {
        if let Some(n) = count.get() {
            count.set(Some(n + 1));
        }
    });
}

// Each integration test binary has its own allocator. Counting only the current
// thread keeps unrelated parallel tests from affecting these assertions.
unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        record_allocation();
        unsafe { System.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        record_allocation();
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        record_allocation();
        unsafe { System.realloc(ptr, layout, size) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static ALLOCATOR: CountingAllocator = CountingAllocator;

fn without_allocations<T>(f: impl FnOnce() -> T) -> T {
    struct Reset;
    impl Drop for Reset {
        fn drop(&mut self) {
            ALLOCATIONS.set(None);
        }
    }
    ALLOCATIONS.set(Some(0));
    let reset = Reset;
    let result = f();
    let count = ALLOCATIONS.get().unwrap();
    drop(reset);
    assert_eq!(count, 0, "deserializing an inline value allocated");
    result
}

// Inline-only callers pass None; FlexStr callers also check the storage variant.
type CheckStorage<T> = Option<fn(&T, bool)>;

fn check<'de, T, D, S>(de: D, expected: &S, fits: bool, storage: CheckStorage<T>)
where
    T: Deserialize<'de> + Deref<Target = S>,
    D: Deserializer<'de>,
    S: ?Sized + PartialEq + Debug,
{
    let result = if fits {
        without_allocations(|| T::deserialize(de))
    } else {
        T::deserialize(de)
    };
    if !fits && storage.is_none() {
        assert!(result.is_err(), "oversized value fit in inline storage");
    } else {
        let value = result.unwrap();
        assert_eq!(&*value, expected);
        if let Some(check_storage) = storage {
            check_storage(&value, fits);
        }
    }
}

struct OwnedBytes(Vec<u8>);

impl<'de> Deserializer<'de> for OwnedBytes {
    type Error = Error;

    fn deserialize_any<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        visitor.visit_byte_buf(self.0)
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string bytes
        byte_buf option unit unit_struct newtype_struct seq tuple tuple_struct
        map struct enum identifier ignored_any
    }
}

// Model a format that only avoids an allocation when given a borrowing hint.
#[cfg(any(feature = "str", feature = "cstr", feature = "path"))]
struct BorrowingDeserializer<'de, const BYTES: bool>(&'de str);

#[cfg(any(feature = "str", feature = "cstr", feature = "path"))]
impl<'de, const BYTES: bool> Deserializer<'de> for BorrowingDeserializer<'de, BYTES> {
    type Error = Error;

    fn deserialize_any<V: serde::de::Visitor<'de>>(self, _: V) -> Result<V::Value, Error> {
        panic!("deserialization must request the borrowing representation")
    }

    fn deserialize_str<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        assert!(!BYTES);
        visitor.visit_borrowed_str(self.0)
    }

    fn deserialize_bytes<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        assert!(BYTES);
        visitor.visit_borrowed_bytes(self.0.as_bytes())
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char string
        byte_buf option unit unit_struct newtype_struct seq tuple tuple_struct
        map struct enum identifier ignored_any
    }
}

#[cfg(any(feature = "str", feature = "path"))]
pub fn text<T, S>(expected: fn(&str) -> &S, storage: CheckStorage<T>)
where
    T: DeserializeOwned + Deref<Target = S>,
    S: ?Sized + PartialEq + Debug,
{
    for len in [0, INLINE_CAPACITY, INLINE_CAPACITY + 1, INLINE_CAPACITY * 3] {
        let input = "x".repeat(len);
        let fits = len <= INLINE_CAPACITY;
        check::<T, _, _>(
            BorrowingDeserializer::<false>(&input),
            expected(&input),
            fits,
            storage,
        );
        check::<T, _, _>(
            BorrowedStrDeserializer::<Error>::new(&input),
            expected(&input),
            fits,
            storage,
        );
        check::<T, _, _>(
            StrDeserializer::<Error>::new(&input),
            expected(&input),
            fits,
            storage,
        );
        check::<T, _, _>(
            serde_json::Value::String(input.clone()),
            expected(&input),
            fits,
            storage,
        );
        check::<T, _, _>(
            BytesDeserializer::<Error>::new(input.as_bytes()),
            expected(&input),
            fits,
            storage,
        );
        check::<T, _, _>(
            BorrowedBytesDeserializer::<Error>::new(input.as_bytes()),
            expected(&input),
            fits,
            storage,
        );
        check::<T, _, _>(
            OwnedBytes(input.as_bytes().to_vec()),
            expected(&input),
            fits,
            storage,
        );
        let json = serde_json::to_string(&input).unwrap();
        check::<T, _, _>(
            &mut serde_json::Deserializer::from_str(&json),
            expected(&input),
            fits,
            storage,
        );
    }
    for bytes in [&[0xff][..], &[0xc3, 0x28][..]] {
        assert!(T::deserialize(BytesDeserializer::<Error>::new(bytes)).is_err());
        assert!(T::deserialize(BorrowedBytesDeserializer::<Error>::new(bytes)).is_err());
        assert!(T::deserialize(OwnedBytes(bytes.to_vec())).is_err());
    }
    // JSON must allocate scratch space for escapes; the result still has to be correct.
    let escaped: T = serde_json::from_str(r#""\u00e9\n""#).unwrap();
    assert_eq!(&*escaped, expected("é\n"));
}

#[cfg(feature = "bytes")]
pub fn bytes<T: DeserializeOwned + Deref<Target = [u8]>>(storage: CheckStorage<T>) {
    for len in [0, INLINE_CAPACITY, INLINE_CAPACITY + 1, INLINE_CAPACITY * 3] {
        let input = vec![0xff; len];
        let fits = len <= INLINE_CAPACITY;
        check::<T, _, _>(
            SeqDeserializer::<_, Error>::new(input.iter().copied()),
            &input[..],
            fits,
            storage,
        );
        let json = serde_json::to_string(&input).unwrap();
        check::<T, _, _>(
            &mut serde_json::Deserializer::from_str(&json),
            &input[..],
            fits,
            storage,
        );
    }
    for invalid in ["[256]", "[-1]", "[1, false]", "[1.5]"] {
        assert!(serde_json::from_str::<T>(invalid).is_err());
    }
    // Like Box<[u8]>, these strings deserialize Serde sequences, not byte buffers.
    assert!(T::deserialize(BytesDeserializer::<Error>::new(b"x")).is_err());
    assert!(T::deserialize(BorrowedBytesDeserializer::<Error>::new(b"x")).is_err());
    assert!(T::deserialize(OwnedBytes(b"x".to_vec())).is_err());
}

#[cfg(feature = "cstr")]
pub fn cstr<T: DeserializeOwned + Deref<Target = std::ffi::CStr>>(storage: CheckStorage<T>) {
    for len in [0, INLINE_CAPACITY - 1, INLINE_CAPACITY, INLINE_CAPACITY * 3] {
        let input = "x".repeat(len);
        let expected = std::ffi::CString::new(input.as_bytes()).unwrap();
        let fits = len < INLINE_CAPACITY; // The terminator also occupies inline storage.
        check::<T, _, _>(
            BorrowingDeserializer::<true>(&input),
            expected.as_c_str(),
            fits,
            storage,
        );
        check::<T, _, _>(
            BorrowedStrDeserializer::<Error>::new(&input),
            expected.as_c_str(),
            fits,
            storage,
        );
        check::<T, _, _>(
            StrDeserializer::<Error>::new(&input),
            expected.as_c_str(),
            fits,
            storage,
        );
        check::<T, _, _>(
            serde_json::Value::String(input.clone()),
            expected.as_c_str(),
            fits,
            storage,
        );
        check::<T, _, _>(
            BytesDeserializer::<Error>::new(input.as_bytes()),
            expected.as_c_str(),
            fits,
            storage,
        );
        check::<T, _, _>(
            BorrowedBytesDeserializer::<Error>::new(input.as_bytes()),
            expected.as_c_str(),
            fits,
            storage,
        );
        check::<T, _, _>(
            OwnedBytes(input.as_bytes().to_vec()),
            expected.as_c_str(),
            fits,
            storage,
        );
        check::<T, _, _>(
            SeqDeserializer::<_, Error>::new(input.bytes()),
            expected.as_c_str(),
            fits,
            storage,
        );
        let json = serde_json::to_string(input.as_bytes()).unwrap();
        check::<T, _, _>(
            &mut serde_json::Deserializer::from_str(&json),
            expected.as_c_str(),
            fits,
            storage,
        );
    }
    // Serde's CString representation excludes the terminator, so even a final NUL is invalid.
    for input in ["\0", "x\0", "x\0y"] {
        assert!(T::deserialize(StrDeserializer::<Error>::new(input)).is_err());
        assert!(T::deserialize(serde_json::Value::String(input.to_owned())).is_err());
        assert!(T::deserialize(BytesDeserializer::<Error>::new(input.as_bytes())).is_err());
        assert!(T::deserialize(BorrowedBytesDeserializer::<Error>::new(input.as_bytes())).is_err());
        assert!(T::deserialize(OwnedBytes(input.as_bytes().to_vec())).is_err());
        let json = serde_json::to_string(input.as_bytes()).unwrap();
        assert!(serde_json::from_str::<T>(&json).is_err());
    }
    check::<T, _, _>(
        BytesDeserializer::<Error>::new(b"\xff"),
        c"\xff",
        true,
        storage,
    );
}
