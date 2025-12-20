#![allow(dead_code)]

use flexstry::{FlexStr, InlineFlexStr, RefCounted, StringToFromBytes};
use std::net::ToSocketAddrs;

/// Test ToSocketAddrs implementation for FlexStr
pub fn test_to_socket_addrs<S, R>(addr: &'static S)
where
    S: ?Sized + StringToFromBytes + ToSocketAddrs,
    R: RefCounted<S>,
{
    let flex_str: FlexStr<'_, S, R> = FlexStr::from_borrowed(addr);

    // Test ToSocketAddrs - this should work if addr is a valid socket address
    // We'll use a simple test that doesn't require network access
    let mut iter = flex_str.to_socket_addrs().unwrap();
    assert!(iter.next().is_some());
    assert!(iter.next().is_none());
}

/// Test ToSocketAddrs implementation for InlineFlexStr
pub fn test_inline_to_socket_addrs<S>(addr: &'static S)
where
    S: ?Sized + StringToFromBytes + ToSocketAddrs,
{
    // Input should be small enough to inline
    let inline_str =
        InlineFlexStr::try_from_type(addr).expect("test input should be small enough to inline");

    // Test ToSocketAddrs
    let mut iter = inline_str.to_socket_addrs().unwrap();
    assert!(iter.next().is_some());
    assert!(iter.next().is_none());
}
