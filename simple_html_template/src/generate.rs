use syntax::ptr::P;
use syntax::{ast, codemap};
use syntax::parse::token;
use syntax::ext::base;
use syntax::ext::build::AstBuilder;

use html::HtmlState;

/// Trait meaning something can be turned into an ast::Item with configuration.
pub trait Generate<Cfg> {
    /// Turn Self into an ast::Item with a configuration object.
    fn generate(
        self,
        codemap::Span,
        &mut base::ExtCtxt,
        Cfg
    ) -> P<ast::Item>;
}


impl Generate<()> for HtmlState {

    fn generate(
        self,
        sp: codemap::Span,
        cx: &mut base::ExtCtxt,
        _: ()
    ) -> P<ast::Item> {

        let name = self.name.clone().unwrap();

        cx.item_mod(sp, sp, name, vec![], vec![], vec![])
    }
}
