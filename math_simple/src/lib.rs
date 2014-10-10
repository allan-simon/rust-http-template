#![feature(plugin_registrar, quote)]

extern crate syntax;
extern crate rustc;
use rustc::plugin;
use syntax::parse::token;

use self::math_expr::math_expr;

mod math_expr;
mod parse;
mod generate;

#[plugin_registrar]
#[doc(hidden)]
pub fn plugin_registrar(reg: &mut plugin::Registry) {
    reg.register_syntax_extension(
        token::intern("math_expr"),
        syntax::ext::base::NormalTT(box math_expr, None)
    );
}
