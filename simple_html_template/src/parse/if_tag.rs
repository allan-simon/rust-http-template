use syntax::ext::base;
use syntax::parse::parser::Parser;
use syntax::parse::token;

use tags::IF;
use tags::template::SubTag;
use tags::template;
use tags::if_tag::If;

use parse_utils::parse_end_tag;
use parse_utils::is_tag_end;

use parse::template::parse_inner_template;


///
///
///
pub fn parse_if_tag( 
    parser: &mut Parser,
    context: &base::ExtCtxt
) -> SubTag {

    let mut tag = If::new();

    parse_start_if(&mut tag, parser);
    tag.sub_tags = parse_inner_template(parser, context);
    parse_end_tag(parser, IF);

    return template::If(tag);
}

///
///
///
fn parse_start_if(state: &mut If, parser: &mut Parser) {

    match (
        parser.parse_expr(),
        // we can't use bump_and_get, as it, get AND THEN bump
        // which mean that is_tag_end would have tested the token AFTER
        parser.token.clone()
    ) {
        (
            ref condition,
            token::EOF,
        ) if is_tag_end(parser)  => {
            state.condition = Some(condition.clone());
            println!("found if beginning")
        },

        (one, two) => {
            parser.fatal(format!(
                "Expected `<% if %>`, found <% if {} {}",
                one,
                two
            ).as_slice());
        }
    };
    parser.bump();
}




