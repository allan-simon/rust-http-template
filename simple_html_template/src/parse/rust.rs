use syntax::parse::parser::Parser;
use syntax::parse::token;

use syntax::ext::base;

use tags::template::SubTag;
use tags::template::RawRust;

use parse_utils::block_to_string;

/// Parse the inside of a orphan rust tag
pub fn parse_rust_tag (
    parser: &mut Parser,
    context: &base::ExtCtxt
) -> SubTag {

    let start_rust_block = parser.span.clone();
    //TODO need to handle the following error case
    //  <% rust blabla       <% another tag %>
    //                  ^
    //                  %> is missing


    while parser.token != token::EOF {
        parser.bump();
    }

    // if the EOF is the real enf of file and not our %>
    // we have a syntax error
    if (parser.reader.is_eof()) {
        parser.fatal("`rust` tag open but not closed");
    }

    let inner_string = block_to_string(
        context,
        &start_rust_block,
        &parser.span
    );
    
    //TODO: certainly a better way to do "consume '%>' "
    parser.bump();

    return RawRust(inner_string);

}


