use syntax::parse::parser::Parser;
use syntax::codemap;
use syntax::parse::token;
use syntax::ext::base;

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

        return next_is_percent;
    }
    return false;
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
