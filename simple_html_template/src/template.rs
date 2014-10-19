use syntax::parse::token;
use syntax::parse::parser::Parser;
use syntax::ext::base;
use syntax::ast;

use parse_utils::block_to_string;
use parse_utils::is_tag_start;

use rust::parse_rust_tag;

use tags::TEMPLATE;
use tags::RUST;
use tags::END;


//TODO factorize

/// Define a HTML Template, i.e a piece of html
/// with other tags inside it
///
#[deriving(Clone)]
pub struct Template {
    pub name: Option<ast::Ident>,
    pub sub_tags: Vec<SubTag>
}

/// Create a new template
impl Template {
    pub fn new() -> Template {
        Template {
            name: None,
            sub_tags: Vec::new()
        }
    }
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
pub fn parse_template_tag( 
    parser: &mut Parser,
    context: &base::ExtCtxt
) -> Template {

    let mut template = Template::new();

    parse_start_template(&mut template, parser);
    template.sub_tags = parse_inner_template(parser, context);
    parse_end_template(parser);

    return template;
}

///
///
///
fn parse_start_template(state: &mut Template, parser: &mut Parser) {

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
                return sub_tags;

            },
            _ => parser.fatal("unknown tag"),
        }

        // we start a new raw html block
        start_html_block = parser.span.clone();
    }

    parser.fatal("template tag opened but not closed");
}


