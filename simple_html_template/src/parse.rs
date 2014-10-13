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
        ) => { println!("nice job")},

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
        parse_end_template(&mut state, parser);
        while parser.token != token::EOF {


            match parser.token {
                token::LT => println!("found <"),
                token::GT => println!("found >"),
                token::LPAREN => println!("found ("),
                token::RPAREN => println!("found )"),
                token::BINOP(token::PERCENT) => println!("found %"),
                token::IDENT(ident, _) => {
                    println!("found ident {}", ident.as_str());
                },
                _ => {
                    parser.fatal("not supported");
                }
            }
            parser.bump();
        }

        state
    }
}
