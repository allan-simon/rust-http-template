use syntax::{ast, codemap, parse};
use syntax::ext::base;
use parse::Parse;
use syntax::ext::base::ExtCtxt;

use generate::Generate;

/// Defines the state of a `html_template!` macro as it is parsing.
///
///
#[deriving(Clone)]
pub struct HtmlState {
    pub skin: Option<ast::Ident>,
    pub name: Option<ast::Ident>,
    pub sub_tags: Vec<SubTag>
}

/// Defines the things we can put inside a <% template %>
///
///
#[deriving(Clone)]
pub enum SubTag {
    RawHtml(String),
    RawRust(String)
}

///
///
///
impl HtmlState {
    pub fn new(skin: Option<ast::Ident>) -> HtmlState {
        HtmlState {
            skin: skin,
            name: None,
            sub_tags: Vec::new()
        }
    }
}

///
///
///
pub fn html_template<'a>(
    cx: &'a mut ExtCtxt,
    sp: codemap::Span,
    name: ast::Ident,
    tokens: Vec<ast::TokenTree>
) -> Box<base::MacResult + 'a> {

    let state: HtmlState = Parse::parse(
        &mut parse::tts_to_parser(
            cx.parse_sess(),
            tokens.into_vec(),
            cx.cfg()
        ),
        (sp, &mut*cx, Some(name))
    );


    base::MacItems::new(
        Some(
            state.generate(
                sp,
                cx,
                ()
            )
        ).into_iter()
    )

}
