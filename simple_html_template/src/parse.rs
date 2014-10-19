use syntax::ast;
use syntax::codemap;
use syntax::ext::base;
use syntax::parse::token;
use syntax::parse::parser::Parser;

use html::HtmlState;
use html::SubTag;
use html::RawHtml;

use rust::parse_rust_tag;

use parse_utils::block_to_string;
use parse_utils::is_tag_start;

/// Trait that means something can be parsed with a configuration.
pub trait Parse<Cfg> {
    /// Parse Self from a Parser and a configuration object.
    fn parse(&mut Parser, Cfg) -> Self;
}

const TEMPLATE:      &'static str = "template";
const RUST:          &'static str = "rust";
const END:           &'static str = "end";

///
///
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

/// Parse the content inside a <% template xxx() %> tag
/// and return when we've finished to parse the <% end template %>
/// if we have parsed all tokens without seeing <% end template %>
/// we quit with error
fn parse_inner_template (
    parser: &mut Parser,
    context: &base::ExtCtxt
) -> Vec<SubTag> {

    let mut sub_tags = Vec::new();

    // to know when we have a piece of HTML to display as it
    let mut start_html_block = parser.span.clone();

    while parser.token != token::EOF {
        if !is_tag_start(parser) {
            parser.bump();
            continue;
        }

        // the beginning of a tag implies that the current raw html block
        // is finished
        let inner_string = block_to_string(
            context,
            &start_html_block,
            &parser.span
        );
        sub_tags.push(RawHtml(inner_string));

        //TODO: certainly a better way to do "consume < and %"
        parser.bump();
        parser.bump();

        match parser.parse_ident().as_str() {
            TEMPLATE => parser.fatal("<% template %> can't be nested"),
            RUST => sub_tags.push(parse_rust_tag(parser, context)),
            END => {
                parse_end_template(parser);
                return sub_tags;

            },
            _ => parser.fatal("unknown tag"),
        }

        // we start a new raw html block
        start_html_block = parser.span.clone();
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

        while parser.token != token::EOF {
            
            if !is_tag_start(parser) {
                parser.bump();
                continue;
            }

            //TODO: certainly a better way to do "consume < and %"
            parser.bump();
            parser.bump();

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

            parser.bump();
        }

        state
    }
}
