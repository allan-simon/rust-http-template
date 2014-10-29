use syntax::ast;
use syntax::ptr::P;

use tags;

/// Define a HTML Template, i.e a piece of html
/// with other tags inside it
///
#[deriving(Clone)]
pub struct Template {
    pub name: Option<ast::Ident>,
    pub inputs: Vec<ast::Arg>,
    pub sub_tags: Vec<SubTag>
}

/// Create a new template
impl Template {
    pub fn new() -> Template {
        Template {
            name: None,
            inputs: Vec::new(),
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
    RawRust(String),
    Include(P<ast::Expr>),
    Print(ast::Ident),
    If(tags::if_tag::If)
}


