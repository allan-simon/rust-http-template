
use syntax::ptr::P;
use syntax::{ast, codemap};
use syntax::ext::base;
use syntax::ext::build::AstBuilder;


use html::HtmlState;


mod template;

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

///
///
///
impl Generate<()> for HtmlState {

    fn generate(
        self,
        sp: codemap::Span,
        cx: &mut base::ExtCtxt,
        _: ()
    ) -> P<ast::Item> {

        let skin = self.skin;

        let template_items = self.templates.into_iter().map(
            |template| template.generate(sp, cx, ())
        ).collect();

        // we create the module made of the created functions
        cx.item_mod(sp, sp, skin, vec![], vec![], template_items)
    }
}


