#![allow(dead_code)]

use flexstr_support::StringToFromBytes;
use inline_flexstr::InlineFlexStr;
use std::net::ToSocketAddrs;

/// Test ToSocketAddrs implementation for InlineFlexStr
pub fn test_to_socket_addrs<S>(addr: &'static S)
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

