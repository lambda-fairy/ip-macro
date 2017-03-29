#![feature(proc_macro)]

extern crate ip_macro;

use ip_macro::{ip, ipv4, ipv6};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

#[test]
fn ipv4_address() {
    let actual = ipv4!("37.247.50.150");
    let expected = Ipv4Addr::from_str("37.247.50.150").unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn ipv6_address() {
    let actual = ipv6!("2607:f8b0:4009:80b::200e");
    let expected = Ipv6Addr::from_str("2607:f8b0:4009:80b::200e").unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn ip_address_v4() {
    let actual = ip!("37.247.50.150");
    let expected = IpAddr::from_str("37.247.50.150").unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn ip_address_v6() {
    let actual = ip!("2607:f8b0:4009:80b::200e");
    let expected = IpAddr::from_str("2607:f8b0:4009:80b::200e").unwrap();
    assert_eq!(actual, expected);
}
