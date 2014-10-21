
use syntax::ast;
use syntax::codemap;
use syntax::ext::base;
use syntax::parse::token;
use syntax::parse::parser::Parser;

use html::HtmlState;

use self::template::parse_template_tag;

use parse_utils::is_tag_start;

use tags::TEMPLATE;
use tags::END;


pub mod template;
pub mod rust;
pub mod print;
pub mod include;

/// Trait that means something can be parsed with a configuration.
pub trait Parse<Cfg> {
    /// Parse Self from a Parser and a configuration object.
    fn parse(&mut Parser, Cfg) -> Self;
}

///
///
///
impl<'a, 'b> Parse<(
    codemap::Span,
    &'a mut base::ExtCtxt<'b>,
    Option<ast::Ident>
)> for HtmlState {

    ///
    ///
    ///
    fn parse(
        parser: &mut Parser,
        (_, context, name): (codemap::Span, &'a mut base::ExtCtxt, Option<ast::Ident>)
    ) -> HtmlState {

        let mut state = HtmlState::new(name);

        while parser.token != token::EOF {
            
            if !is_tag_start(parser) {
                parser.bump();
                continue;
            }

            //TODO: certainly a better way to do "consume < and %"
            parser.bump();
            parser.bump();

            match parser.parse_ident().as_str() {

                TEMPLATE => {
                    state.templates.push(
                        parse_template_tag(parser, context)
                    );
                },
                END => {
                    parser.fatal("<% end template %> found without opening tag");
                },
                otherwise => {
                    let span = parser.last_span;
                    parser.span_fatal(
                        span,
                        format!(
                            "Expected `template` or `end`, but found `{}`",
                            otherwise
                        ).as_slice()
                    );
                }
            }

        }

        state
    }
}
