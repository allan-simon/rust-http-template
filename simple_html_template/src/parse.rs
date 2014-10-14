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
static END:           &'static str = "end";

///
fn parse_start_template(state: &mut HtmlState, parser: &mut Parser) {

    match (
        parser.bump_and_get(),
        parser.parse_ident().as_str(),
        parser.parse_ident(),
        parser.parse_fn_decl(true),
        parser.bump_and_get(),
        parser.bump_and_get()
    ) {
        (
            token::BINOP(token::PERCENT),
            block_name,
            functioname,
            ref function,
            token::BINOP(token::PERCENT),
            token::GT
        ) if block_name == TEMPLATE => { println!("found template beginning")},

        (one, two, three, four, five, six) => {
            parser.fatal(format!(
                "Expected `<% template xxx() %>`, found {}{}{}{}{}{}",
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

fn parse_end_template(state: &mut HtmlState, parser: &mut Parser) {
    match (
        parser.bump_and_get(),
        parser.parse_ident().as_str(),
        parser.parse_ident().as_str(),
        parser.bump_and_get(),
        parser.bump_and_get()
    ) {
        (
            token::BINOP(token::PERCENT),
            end,
            template,
            token::BINOP(token::PERCENT),
            token::GT
        ) if end == END && template == TEMPLATE => { println!("found end template")},

        (one, two, three, four, five) => {
            parser.fatal(format!(
                "Expected `<% end template %>`, found <{} {} {} {}{}",
                Parser::token_to_string(&one),
                two,
                three,
                Parser::token_to_string(&four),
                Parser::token_to_string(&five),
            ).as_slice());
        }
    };

}

fn is_template_tag_start (parser: &Parser) -> bool {

    if (parser.token != token::BINOP(token::PERCENT)) {
        return true;
    }

    match parser.last_token {
        None => return false,
        Some(ref last_token) => {
            return **last_token == token::LT
        }
    }

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

        // try to find a <%
        while
            is_template_tag_start(parser) &&
            parser.token != token::EOF
        {
            parser.bump();
        }
        parse_start_template(&mut state, parser);
        //TODO handle token::LE (see how they've done for brain_fuck macro
        // try to find a <%
        while
            is_template_tag_start(parser) &&
            parser.token != token::EOF
        {
            parser.bump();
        }
        parse_end_template(&mut state, parser);
        state
    }
}
