use syntax::{ast, codemap, parse};
use syntax::ext::base;
use parse::Parse;
use syntax::parse::token;
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


    let codemap = cx.codemap();
    println!("{}", codemap.span_to_snippet(sp).unwrap());

    let files = codemap.files.borrow();

    for x in files.iter() {
        let file = x.deref(); 
        //println!("file name:{}", file.name);
        //println!("{}", file.src);
    }
    println!("we have {}", state.full_message.clone().unwrap());
    

    MacExpr::new(
        cx.expr_str(
            sp,
            // transform a &str into an InternedString
            // usable by expr_str
            token::intern_and_get_ident(
                state.full_message.unwrap().as_slice()
            )
        )
    )
}
