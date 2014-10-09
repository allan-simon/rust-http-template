#![feature(plugin_registrar, quote)]

extern crate syntax;
extern crate rustc;
use rustc::plugin;
use syntax::parse::token;
use syntax::{ast, codemap};
use syntax::ext::base;
use syntax::ext::base::MacExpr;
use syntax::ext::base::ExtCtxt;
use syntax::ext::build::AstBuilder;  // trait for expr_uint



pub fn math_expr<'a>(
    cx: &'a mut ExtCtxt,
    sp: codemap::Span,
    _: &[ast::TokenTree]
) -> Box<base::MacResult + 'a> {
    MacExpr::new(cx.expr_uint(sp, 42))
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut plugin::Registry) {
    reg.register_syntax_extension(
        token::intern("fourty_two"),
        syntax::ext::base::NormalTT(box math_expr, None)
    );
}
