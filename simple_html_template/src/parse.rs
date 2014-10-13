use syntax::ast;
use syntax::codemap;
use syntax::ext::base;
use syntax::parse::token;
use syntax::parse::parser::Parser;

use syntax::codemap::Pos;

use html::HtmlState;

/// Trait that means something can be parsed with a configuration.
pub trait Parse<Cfg> {
    /// Parse Self from a Parser and a configuration object.
    fn parse(&mut Parser, Cfg) -> Self;
}

static TEMPLATE:      &'static str = "template";

///
fn parse_start_template(state: &mut HtmlState, parser: &mut Parser) {

    match (
        parser.bump_and_get(),
        parser.bump_and_get(),
        parser.parse_ident(),
        parser.parse_ident(),
        parser.parse_fn_decl(true),
        parser.bump_and_get(),
        parser.bump_and_get()
    ) {
        (
            token::LT,
            token::BINOP(token::PERCENT),
            ident,
            functioname,
            function,
            token::BINOP(token::PERCENT),
            token::GT
        ) => { println!("found template beginning")},

        (one, two, three, four, five, six, seven) => {
            parser.fatal(format!(
                "Expected `<% template xxx() %>`, found {}{}{}{}{}{}{}",
                one,
                two,
                three,
                four,
                five,
                six,
                seven,
            ).as_slice());
        }
    };
}

fn parse_end_template(state: &mut HtmlState, parser: &mut Parser) {
    match (
        parser.bump_and_get(),
        parser.bump_and_get(),
        parser.parse_ident(),
        parser.parse_ident(),
        parser.bump_and_get(),
        parser.bump_and_get()
    ) {
        (
            token::LT,
            token::BINOP(token::PERCENT),
            end,
            template,
            token::BINOP(token::PERCENT),
            token::GT
        ) => { println!("found end template")},

        (one, two, three, four, five, six) => {
            parser.fatal(format!(
                "Expected `<% end template %>`, found {}{}{}{}{}{}",
                one,
                two,
                three,
                four,
                five,
                six
            ).as_slice());
        }
    };

}


///
///
///
impl<'a, 'b> Parse<(
    codemap::Span,
    &'a mut base::ExtCtxt<'b>,
    Option<ast::Ident>
)> for HtmlState {

    ///
    ///
    ///
    fn parse(
        parser: &mut Parser,
        (_, _, name): (codemap::Span, &'a mut base::ExtCtxt, Option<ast::Ident>)
    ) -> HtmlState {

        let mut state = HtmlState::new(name);

        println!("parser");

        parse_start_template(&mut state, parser);
        //TODO handle token::LE (see how they've done for brain_fuck macro
        while parser.token != token::LT && parser.token != token::EOF {
            parser.bump();
        }
        parse_end_template(&mut state, parser);
        state
    }
}
