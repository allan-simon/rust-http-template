use syntax::{ast, codemap, parse};
use syntax::ptr::P;
use syntax::ext::base;
use parse::Parse;
use syntax::ext::base::MacExpr;
use syntax::ext::base::ExtCtxt;
use syntax::ext::build::AstBuilder;  // trait for expr_uint

use std::vec::Vec;


//use generate::Generate;

/// Defines the state of a `math_expr!` macro as it is parsing.
///
///
#[deriving(Clone)]
pub struct MathExpState {
    pub left: Option<uint>,
    pub right: Option<uint>,
    pub operator: Option<uint> //TODO replace uint by a enum
}

///
///
///
impl MathExpState {
    pub fn new() -> MathExpState {
        MathExpState {
            left: None,
            right: None,
            operator: None
        }
    }
}

///
///
///
pub fn math_expr<'a>(
    cx: &'a mut ExtCtxt,
    sp: codemap::Span,
    tokens: &[ast::TokenTree]
) -> Box<base::MacResult + 'a> {
    // Parse a full DescribeState from the input,
    // emitting errors if used incorrectly.
    let state: MathExpState = Parse::parse(
        &mut parse::tts_to_parser(
            cx.parse_sess(),
            tokens.into_vec(),
            cx.cfg()
        ),
        (sp, &mut*cx)
    );

    let total = state.left.unwrap() + state.right.unwrap();
    MacExpr::new(cx.expr_uint(sp, total))
}
