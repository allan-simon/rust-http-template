use syntax::ptr::P;
use syntax::{ast, ast_util, abi, codemap};
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

        // we create the 'return' expression, it returns for the
        // moment a static string based on the one computed during
        // parsing phase
        let inner_string : &str = self.inner_string.as_slice();
        let expr = quote_expr!(
            cx,
            {
                return $inner_string;
            }
        );


        // we create the function itself
        let render_fn = ast::ItemFn(
            //TODO: we should be able to take it
            //directly from the parsing phase
            cx.fn_decl(
                // Takes no arguments
                vec![],
                // returns a static string
                quote_ty!(cx, &'static str)
            ),

            // All the usual types.
            ast::NormalFn,
            abi::Rust,
            ast_util::empty_generics(),

            // Add the body of the function.
            // which is made of only one expr
            // hence the call to block_expr
            cx.block_expr(expr)
        );

        // and now we transform the function into an
        // AST item, by giving the function a name, a visibility
        // etc.
        let render_item = P(ast::Item {
            // TODO hardcoded, we should replace render
            // by the xxx coming from <% template xxx() %>
            ident: cx.ident_of("render"),
            // note: attrs here are all the #[], not to be confused
            // with the function arguments
            attrs: vec![],
            id: ast::DUMMY_NODE_ID,
            node: render_fn,
            // TODO: maybe permit the template creator to precise
            // if this function is public or not ?
            vis: ast::Public,
            span: sp
        });


        // we create the module made of the created function
        // TODO: allow the creation of several items
        cx.item_mod(sp, sp, name, vec![], vec![], vec![render_item])
    }
}
