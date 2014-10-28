use syntax::parse::parser::Parser;
use syntax::parse::token;

use syntax::ext::base;

use tags::template::SubTag;
use tags::template::Print;

use parse_utils::block_to_string;
use parse_utils::is_tag_end;

/// Parse the inside of a orphan print tag
pub fn parse_print_tag (
    parser: &mut Parser,
) -> SubTag {

    let ident = parser.parse_ident();
    if !is_tag_end(parser) {
        parser.fatal("`<%= %>` tag open but not closed");
    }
    //TODO: certainly a better way to do "consume '%>'"
    parser.bump();

    Print(ident)
}


