use syntax::ast;
use syntax::codemap;
use syntax::ext::base;
use syntax::parse::token;
use syntax::parse::parser::Parser;

use html::HtmlState;
use html::SubTag;
use html::RawHtml;

/// Trait that means something can be parsed with a configuration.
pub trait Parse<Cfg> {
    /// Parse Self from a Parser and a configuration object.
    fn parse(&mut Parser, Cfg) -> Self;
}

const TEMPLATE:      &'static str = "template";
const END:           &'static str = "end";

///
fn parse_start_template(state: &mut HtmlState, parser: &mut Parser) {

    match (
        parser.parse_ident(),
        parser.parse_fn_decl(true),
        parser.bump_and_get(),
        parser.bump_and_get()
    ) {
        (
            functioname,
            ref function,
            token::BINOP(token::PERCENT),
            token::GT
        ) => {
            state.name = Some(functioname);
            println!("found template beginning")
        },

        (one, two, three, four) => {
            parser.fatal(format!(
                "Expected `<% template xxx() %>`, found <% template {} {} {}{}",
                one,
                two,
                three,
                four
            ).as_slice());
        }
    };
}

///
///
///
fn parse_end_template(parser: &mut Parser) {

    match (
        parser.parse_ident().as_str(),
        parser.bump_and_get(),
        parser.bump_and_get()
    ) {
        (
            template,
            token::BINOP(token::PERCENT),
            token::GT
        ) if template == TEMPLATE => { println!("found end template")},

        (one, two, three) => {
            parser.fatal(format!(
                "Expected `<% end template %>`, found <% end {} {}{}",
                one,
                Parser::token_to_string(&two),
                Parser::token_to_string(&three),
            ).as_slice());
        }
    };

}

///
///
///
fn is_template_tag_start (
    parser: &Parser,
    last_token: Option<token::Token>
) -> bool {

    if parser.token != token::BINOP(token::PERCENT) {
        return false;
    }


    match last_token {
        None => return false,
        Some(value) => {
            return value == token::LT;
        }
    }

}

/// Extract as raw text the content between two spans
///
fn block_to_string(
    context: &base::ExtCtxt,
    start_span: &codemap::Span,
    end_span: &codemap::Span
) -> String {


    let tmp_span = codemap::Span {
        lo: start_span.lo,
        hi: end_span.lo,
        expn_id: end_span.expn_id
    };

    context.codemap().span_to_snippet(tmp_span).unwrap_or(String::new())

}

/// Parse the content inside a <% template xxx() %> tag
/// and return when we've finished to parse the <% end template %>
/// if we have parsed all tokens without seeing <% end template %>
/// we quit with error
fn parse_inner_template (
    parser: &mut Parser,
    context: &base::ExtCtxt
) -> Vec<SubTag> {

    let mut sub_tags = Vec::new();

    let mut last_token = None;
    // to know when we have a piece of HTML to display as it
    let mut start_html_block = parser.span.clone();
    let mut end_html_block = parser.span.clone();

    while parser.token != token::EOF {

        if !is_template_tag_start(parser, last_token) {
            // we update endspan everytime as we're not sure
            // when a span will be the last one
            end_html_block = parser.span.clone();
            last_token = Some(parser.token.clone());
            parser.bump();
            continue;
        }
        // we consider that what we have after a <% template %>
        // is certainly html
        //start_html_block = parser.span.clone();
        last_token = Some(parser.token.clone());
        parser.bump();

        //TODO handle token::LE (see how they've done for brain_fuck macro
        match parser.parse_ident().as_str() {
            TEMPLATE => {
                parser.fatal("<% template %> can't be nested");
            }
            END => {
                parse_end_template(parser);
                println!("start {} , end {}", start_html_block, end_html_block);
                let inner_string = block_to_string(
                    context,
                    &start_html_block,
                    &end_html_block
                );
                sub_tags.push(RawHtml(inner_string));
                return sub_tags;
            }
            _ => { println!("ignored stuff");}
        }
    }

    parser.fatal("template tag opened but not closed");
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
        (_, context, name): (codemap::Span, &'a mut base::ExtCtxt, Option<ast::Ident>)
    ) -> HtmlState {

        let mut state = HtmlState::new(name);

        println!("parser");

        let mut last_token = None;

        while parser.token != token::EOF {
            
            if !is_template_tag_start(parser, last_token) {
                // we update endspan everytime as we're not sure
                // when a span will be the last one
                last_token = Some(parser.token.clone());
                parser.bump();
                continue;
            }

            last_token = Some(parser.token.clone());
            parser.bump();
            //TODO handle token::LE (see how they've done for brain_fuck macro
            match parser.parse_ident().as_str() {

                TEMPLATE => {
                    parse_start_template(&mut state, parser);
                    state.sub_tags = parse_inner_template(parser, context);
                },
                END => {
                    parser.fatal("<% end template %> found without opening tag");
                },
                otherwise => {
                    let span = parser.last_span;
                    parser.span_fatal(
                        span,
                        format!(
                            "Expected `template` or `end`, but found `{}`",
                            otherwise
                        ).as_slice()
                    );
                }
            }

            last_token = Some(parser.token.clone());
            parser.bump();
        }


        state
    }
}
