use syntax::parse::parser::Parser;
use syntax::codemap;
use syntax::parse::token;
use syntax::ext::base;

/// check if the current token read by the parser is a end of tag
/// i.e "%>", we encapsulate that to hide the dirty trick we have
/// to represent our custom "%>" token by the EOF token
pub fn is_tag_end (
    parser: &Parser
) -> bool {
    parser.token == token::EOF && !parser.reader.is_eof()
}

///
///
///
pub fn is_tag_start (
    parser: &mut Parser
) -> bool {

    if parser.token == token::LT {
        let next_is_percent = parser.look_ahead(
            1,
            |token| *token == token::BINOP(token::PERCENT)
        );

        if (next_is_percent) {
            return true;
        }

        // we consider <%= as being a starting tag
        // we will take care of splitting "%=" into "%" and "="
        // in the 'eat_tag_start' function
        let next_is_percent_equal = parser.look_ahead(
            1,
            |token| *token == token::BINOPEQ(token::PERCENT)
        );


        return next_is_percent_equal;

    }
    return false;
}

/// consume the <% of a tag, and handle corner case
/// for example <%= which is normally < and %=, it will leave
/// the '='
pub fn eat_tag_start (
    parser: &mut Parser
) {

    // we 'eat' the <
    parser.bump();

    // if we have %= , we replace by = (which is like 'eating
    // the %)
    if parser.token == token::BINOPEQ(token::PERCENT) {
        let span = parser.span;
        let lo = span.lo + codemap::BytePos(1);
        parser.replace_token(token::EQ, lo, span.hi);
    } else {
        parser.bump();
    }
}

/// Extract as raw text the content between two spans
///
pub fn block_to_string(
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
