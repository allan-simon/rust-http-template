use syntax::parse::parser::Parser;
use syntax::parse::token;
use syntax::ast;

use tags::template::SubTag;
use tags::template::Include;

use parse_utils::is_tag_end;

/// Parse the inside of a orphan rust tag
/// TODO: implement it, for the moment we simply "consume" the inside
pub fn parse_include_tag (
    parser: &mut Parser
) -> SubTag {

    let call_expr = parser.parse_dot_or_call_expr();
    match call_expr.deref() {
        &ast::Expr { 
            id:  _,
            node: ast::ExprCall(_,_),
            span: _
        } => {},
        _ => parser.fatal("<% include %> should contain a function call")
    }

    // common with <% rust %> tag
    // if we got an EOF which the actual end of file
    if !is_tag_end(parser) {
        parser.fatal("`include` tag open but not closed");
    }
    // find a better way to consume %>
    parser.bump();

    Include(call_expr)
 
}
