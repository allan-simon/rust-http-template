use syntax::ptr::P;
use syntax::{ast, ast_util, abi, codemap};
use syntax::ext::base;
use syntax::ext::build::AstBuilder;

use syntax::ext::quote::rt::ToSource;
use syntax::ext::quote::rt::ExtParseUtils;

use generate::Generate;

use tags::template::Template;
use tags::template::RawHtml;
use tags::template::RawRust;
use tags::template::Include;

///
///
///
impl Generate<()> for Template {

    fn generate(
        self,
        sp: codemap::Span,
        cx: &mut base::ExtCtxt,
        _: ()
    ) -> P<ast::Item> {


        let fn_decl = generate_template_head(&self, cx);
        let block = generate_template_body(&self, cx);

        // we create the function itself
        let render_fn = ast::ItemFn(
            fn_decl,

            // All the usual types.
            ast::NormalFn,
            abi::Rust,
            ast_util::empty_generics(),

            // Add the body of the function.
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

        render_item
    }
}


///
///
///
fn generate_template_head (
    template: &Template,
    cx: &base::ExtCtxt
) -> P<ast::FnDecl> {

    // Note: we can't use the commented code below because self.inputs
    // contains identifier coming from the macro itself, as the block is
    // not generated from the macro itself but an intermediate code,
    // the identifier in the function block though having the same name
    //  as the parameters, will be considered by the compiler
    // as being different (because of Macro hygiene stuff
    //let mut fn_decl = cx.fn_decl(
    //    // Takes arguments found in template declaration
    //    self.inputs.clone(),
    //    // we force returns value to String
    //    quote_ty!(cx, String)
    //);

    let mut first = true;
    let mut fun_decl_str = "(".to_string();

    for arg in template.inputs.iter() {
        if first { first = false; } else { fun_decl_str.push(','); }
        fun_decl_str.push_str(arg.to_source().as_slice());
    }

    fun_decl_str.push(')');
    fun_decl_str.push_str(" -> String");

    let tt = cx.parse_tts(fun_decl_str);

    cx.new_parser_from_tts(tt.as_slice()).parse_fn_decl(false)
}

///
///
///
fn generate_template_body (
    template: &Template,
    cx: &base::ExtCtxt
) -> P<ast::Block> {

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

    for stuff in template.sub_tags.iter() {
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
            },
            // we put call to other template or function
            // the return value of which is added to the output
            // buffer
            Include(ref call_expr) => {

                let include_call = quote_stmt!(
                    cx,
                    out.push_str($call_expr.as_slice());
                );

                template_block_str.push_str(
                    include_call.to_source().as_slice()
                );
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
    template_block_str.push('\n');

    // we close the block
    template_block_str.push('}');

    // we parse the intermediate source code in order to extract out of
    // it a block to use as the function's body
    let tt = cx.parse_tts(template_block_str);

    cx.new_parser_from_tts(tt.as_slice()).parse_block()

}
