#![allow(dead_code)]

use flexstr::{FlexStr, RefCounted};
use flexstr_support::StringToFromBytes;
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
