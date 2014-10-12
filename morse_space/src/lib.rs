#![feature(plugin_registrar, quote)]

extern crate syntax;
extern crate rustc;
use rustc::plugin;
use syntax::parse::token;

use self::morse::morse;

mod morse;
mod parse;

#[plugin_registrar]
#[doc(hidden)]
pub fn plugin_registrar(reg: &mut plugin::Registry) {
    reg.register_syntax_extension(
        token::intern("morse"),
        syntax::ext::base::NormalTT(box morse, None)
    );
}
