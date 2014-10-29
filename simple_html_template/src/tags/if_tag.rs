use syntax::ast;
use syntax::ptr::P;

use tags::template::SubTag;

/// Define a Template "if", it can contains exactly
/// the same things as a template
///
#[deriving(Clone)]
pub struct If {
    pub condition: Option<P<ast::Expr>>,
    pub sub_tags: Vec<SubTag>
}

/// Create a new if
impl If {
    pub fn new() -> If {
        If {
            condition: None,
            sub_tags: Vec::new()
        }
    }
}


