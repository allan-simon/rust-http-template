use syntax::parse::token;
use syntax::parse::parser::Parser;
use syntax::ext::base;

use parse_utils::block_to_string;
use parse_utils::is_tag_start;
use parse_utils::eat_tag_start;
use parse_utils::is_tag_end;
use parse_utils::parse_end_tag;

use parse::rust::parse_rust_tag;
use parse::include::parse_include_tag;
use parse::print::parse_print_tag;
use parse::if_tag::parse_if_tag;

use tags::TEMPLATE;
use tags::RUST;
use tags::PRINT;
use tags::END;
use tags::INCLUDE;
use tags::IF;

use tags::template::Template;
use tags::template::SubTag;
use tags::template::RawHtml;

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
    parse_end_tag(parser, TEMPLATE);

    return template;
}

///
///
///
fn parse_start_template(state: &mut Template, parser: &mut Parser) {

    match (
        parser.parse_ident(),
        parser.parse_fn_decl(true),
        // we can't use bump_and_get, as it, get AND THEN bump
        // which mean that is_tag_end would have tested the token AFTER
        parser.token.clone()
    ) {
        (
            functioname,
            ref function_decl,
            token::EOF,
        ) if is_tag_end(parser)  => {
            state.name = Some(functioname);
            state.inputs = function_decl.inputs.clone();
            println!("found template beginning")
        },

        (one, two, three) => {
            parser.fatal(format!(
                "Expected `<% template xxx() %>`, found <% template {} {} {}",
                one,
                two,
                three
            ).as_slice());
        }
    };
    parser.bump();
}


/// Parse the content inside a <% template xxx() %> tag
/// and return when we've finished to parse the <% end template %>
/// if we have parsed all tokens without seeing <% end template %>
/// we quit with error
pub fn parse_inner_template (
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

        eat_tag_start(parser);
        //TODO: certainly a better way to do "consume '<%' "


        let tag_name = parser.bump_and_get();
        println!("{}", Parser::token_to_string(&tag_name));
        match Parser::token_to_string(&tag_name).as_slice() {
            TEMPLATE => parser.fatal("<% template %> can't be nested"),
            RUST => sub_tags.push(parse_rust_tag(parser, context)),
            INCLUDE => sub_tags.push(parse_include_tag(parser)),
            PRINT => sub_tags.push(parse_print_tag(parser)),
            IF => sub_tags.push(parse_if_tag(parser, context)),
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


