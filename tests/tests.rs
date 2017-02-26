#![feature(plugin)]
#![plugin(ip_macro)]

use std::net::Ipv6Addr;
use std::str::FromStr;

#[test]
fn ipv6_address() {
    let actual = ipv6!("2607:f8b0:4009:80b::200e");
    let expected = Ipv6Addr::from_str("2607:f8b0:4009:80b::200e").unwrap();
    assert_eq!(actual, expected);
}
