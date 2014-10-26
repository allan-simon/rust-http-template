use syntax::{ast, codemap, parse};
use syntax::ext::base;
use syntax::parse::parser::Parser;
use syntax::ext::base::ExtCtxt;
use syntax::parse::lexer;


use parse::parse;
use generate::Generate;

use tags::template::Template;

use reader::HtmlTemplateReader;


/// Defines the state of a `html_template!` macro as it is parsing.
///
///
#[deriving(Clone)]
pub struct HtmlState {
    pub skin: ast::Ident,
    pub name: Option<ast::Ident>,
    pub templates: Vec<Template>
}


///
///
///
impl HtmlState {
    pub fn new(skin: ast::Ident) -> HtmlState {
        HtmlState {
            skin: skin,
            name: None,
            templates: Vec::new()
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


    // TODO: certainly move that to reader
    let trdr = lexer::new_tt_reader(
        &cx.parse_sess().span_diagnostic,
        None,
        tokens
    );
    let mut reader = HtmlTemplateReader::new(trdr);
    let state = parse( 
        reader,
        cx,
        name
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
