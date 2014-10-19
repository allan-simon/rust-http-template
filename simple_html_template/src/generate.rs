use syntax::ptr::P;
use syntax::{ast, ast_util, abi, codemap};
use syntax::ext::base;
use syntax::ext::build::AstBuilder;

use syntax::ext::quote::rt::ToSource;
use syntax::ext::quote::rt::ExtParseUtils;

use html::HtmlState;
use html::RawHtml;
use html::RawRust;

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

        let skin = self.skin.clone().unwrap();

        // because of the fact that <% rust %> can contains incomplete
        // (for example  <% rust if true { %> it's true ! <%rust } %>
        // code, we can't parse them directly, instead we need an
        // intermdiate state in which we will recreate a block source code
        // that we will parse as a last step
        // we need a starting "{" and a final "}" to use block parser
        let mut template_block_str = "{\n".to_string();

        // we declare a "out" variable that will be used to contains the
        // html output
        // Note: we could have written directly in template_block_str
        // without using quote_stmt! and "to_source", but it makes the code
        // "cleaner" and will ease the transition if one day we find a better
        // solution to not have to go through this second parsing step
        // (maybe at least test if we have 0 <% rust %> tag, in which case
        // we can directly use quote_stmt output
        let expr = quote_stmt!(
            cx,
            let mut out = String::new();
        );

        template_block_str.push_str(expr.to_source().as_slice());
        template_block_str.push('\n');

        for stuff in self.sub_tags.iter() {
            match *stuff {
                // Raw html are added directly directly to "out"
                RawHtml(ref x) => {

                    let html_str = x.as_slice();
                    let raw_html_expr = quote_stmt!(
                        cx,
                        out.push_str($html_str);
                    );

                    template_block_str.push_str(
                        raw_html_expr.to_source().as_slice()
                    );
                    template_block_str.push('\n');
                }
                // Raw rust is added to the intermdiate source code
                RawRust(ref x) => {
                    template_block_str.push_str(x.as_slice());
                    template_block_str.push('\n');
                }
            }
        }

        // we create the return statement
        let return_expr = quote_stmt!(
            cx,
            return out;
        );
        template_block_str.push_str(return_expr.to_source().as_slice()); 

        // we close the block
        template_block_str.push_str("\n}");

        // we parse the intermediate source code in order to extract out of
        // it a block to use as the function's body
        let tt = cx.parse_tts(template_block_str);        
        let block = cx.new_parser_from_tts(tt.as_slice()).parse_block();


        // we create the function itself
        let render_fn = ast::ItemFn(
            //TODO: we should be able to take it
            //directly from the parsing phase
            cx.fn_decl(
                // Takes no arguments
                vec![],
                // returns a String
                quote_ty!(cx, String)
            ),

            // All the usual types.
            ast::NormalFn,
            abi::Rust,
            ast_util::empty_generics(),

            // Add the body of the function.
            // which is made of only one expr
            // hence the call to block_expr
            block
        );

        // and now we transform the function into an
        // AST item, by giving the function a name, a visibility
        // etc.
        let render_item = P(ast::Item {
            // TODO hardcoded, we should replace render
            // by the xxx coming from <% template xxx() %>
            ident: cx.ident_of(
                self.name.unwrap().as_str()
            ),
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
        cx.item_mod(sp, sp, skin, vec![], vec![], vec![render_item])
    }
}
