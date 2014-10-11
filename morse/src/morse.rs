use syntax::{ast, codemap, parse};
use syntax::ext::base;
use parse::Parse;
use syntax::ext::base::MacExpr;
use syntax::ext::base::ExtCtxt;
use syntax::ext::build::AstBuilder;  // trait for expr_uint

/// Defines the state of a `morse!` macro as it is parsing.
///
///
#[deriving(Clone)]
pub struct MorseState {
    pub current_letter: Option<String>,
    pub full_message: Option<String>
}

///
///
///
impl MorseState {
    pub fn new() -> MorseState {
        MorseState {
            current_letter: None,
            full_message: None
        }
    }
}

///
///
///
pub fn morse<'a>(
    cx: &'a mut ExtCtxt,
    sp: codemap::Span,
    tokens: &[ast::TokenTree]
) -> Box<base::MacResult + 'a> {

    let state: MorseState = Parse::parse(
        &mut parse::tts_to_parser(
            cx.parse_sess(),
            tokens.into_vec(),
            cx.cfg()
        ),
        (sp, &mut*cx)
    );

    println!("we found {}", state.full_message.unwrap());
    let total = 42;
    MacExpr::new(cx.expr_uint(sp, total))
}
