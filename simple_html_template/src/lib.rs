#![feature(plugin_registrar, quote)]

extern crate syntax;
extern crate rustc;
use rustc::plugin;
use syntax::parse::token;

use self::html::html_template;

mod html;
mod parse;
mod generate;

#[plugin_registrar]
#[doc(hidden)]
pub fn plugin_registrar(reg: &mut plugin::Registry) {
    reg.register_syntax_extension(
        token::intern("html_template"),
        syntax::ext::base::IdentTT(box html_template, None)
    );
}
