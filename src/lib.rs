#![feature(proc_macro)]

/// Macros for writing literal IP addresses.
///
/// This crate provides three macros:
///
/// * `ip!()`
/// * `ipv4!()`
/// * `ipv6!()`
///
/// All of these take a single argument: a string literal representing an IP
/// address. `ipv4!()` and `ipv6!()` return an `Ipv4Addr` and `Ipv6Addr`,
/// respectively. `ip!()` parses a generic `IpAddr`, which can be either an IPv4
/// or an IPv6 address.
///
/// # Example
///
/// ```rust
/// #![feature(proc_macro)]  // <- Don't forget this!
///
/// extern crate ip_macro;
/// use ip_macro::ip;
///
/// fn main() {
///     println!("There's no place like {}", ip!("127.0.0.1"));
/// }
/// ```

extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::Tokens;
use std::fmt::Debug;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

#[proc_macro]
pub fn ip(input: TokenStream) -> TokenStream {
    parse_ts(match parse_string_literal(input, "IP address") {
        IpAddr::V4(addr) => {
            let q = quote_ipv4(addr);
            quote!(::std::net::IpAddr::V4(#q))
        }
        IpAddr::V6(addr) => {
            let q = quote_ipv6(addr);
            quote!(::std::net::IpAddr::V6(#q))
        }
    })
}

#[proc_macro]
pub fn ipv4(input: TokenStream) -> TokenStream {
    parse_ts(quote_ipv4(parse_string_literal(input, "IPv4 address")))
}

#[proc_macro]
pub fn ipv6(input: TokenStream) -> TokenStream {
    parse_ts(quote_ipv6(parse_string_literal(input, "IPv6 address")))
}

fn parse_string_literal<T>(input: TokenStream, expected: &'static str) -> T
    where T: FromStr,
          T::Err: Debug
{
    match syn::parse::string(&input.to_string()).expect("string literal").value.parse() {
        Ok(r) => r,
        Err(_) => panic!("invalid {}", expected),
    }
}

fn quote_ipv4(addr: Ipv4Addr) -> Tokens {
    let octets = addr.octets();
    quote!(::std::net::Ipv4Addr::new(#(#octets),*))
}

fn quote_ipv6(addr: Ipv6Addr) -> Tokens {
    let segments = addr.segments();
    quote!(::std::net::Ipv6Addr::new(#(#segments),*))
}

fn parse_ts(tokens: Tokens) -> TokenStream {
    tokens.as_str().parse().unwrap()
}
