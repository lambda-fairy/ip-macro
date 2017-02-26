#![feature(plugin_registrar, rustc_private)]

extern crate rustc;
extern crate rustc_plugin;
extern crate syntax;

use rustc_plugin::Registry;
use std::net::Ipv6Addr;
use std::str::FromStr;
use syntax::ast::{Ident, LitKind, LitIntType};
use syntax::codemap::Span;
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
use syntax::ext::build::AstBuilder;
use syntax::parse::token;
use syntax::symbol::Symbol;
use syntax::tokenstream::TokenTree;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    // TODO write these ^_^
    //reg.register_macro("ip", expand_ip);
    //reg.register_macro("ipv4", expand_ipv4);
    reg.register_macro("ipv6", expand_ipv6);
}

fn expand_ipv6(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> Box<MacResult + 'static> {
    if let Some(s) = extract_string(cx, sp, args) {
        match Ipv6Addr::from_str(&s.as_str()) {
            Ok(addr) => {
                let fn_path = ["std", "net", "Ipv6Addr", "new"].iter()
                    .map(|s| Ident::from_str(s)).collect();
                let args = addr.segments().iter()
                    .map(|x| cx.expr_lit(
                            sp,
                            LitKind::Int(*x as _, LitIntType::Unsuffixed)))
                    .collect();
                let expr = cx.expr_call_global(sp, fn_path, args);
                return MacEager::expr(expr);
            },
            Err(e) => cx.span_err(sp, &e.to_string()),
        }
    }
    DummyResult::any(sp)
}

fn extract_string(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree]) -> Option<Symbol> {
    if args.len() != 1 {
        cx.span_err(
            sp,
            &format!("argument should be a single string literal, but got {} arguments", args.len()));
        return None;
    }

    match args[0] {
        TokenTree::Token(_, token::Literal(token::Str_(s), _)) |
        TokenTree::Token(_, token::Literal(token::StrRaw(s, _), _)) => Some(s),
        _ => {
            cx.span_err(sp, "expected literal string");
            None
        },
    }
}
