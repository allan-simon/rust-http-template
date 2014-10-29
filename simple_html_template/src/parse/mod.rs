
use syntax::ast;
use syntax::codemap;
use syntax::ext::base;
use syntax::parse::token;
use syntax::parse::parser::Parser;

use html::HtmlState;

use self::template::parse_template_tag;

use reader::HtmlTemplateReader;
use parse_utils::is_tag_start;

use tags::TEMPLATE;
use tags::END;


pub mod template;
pub mod rust;
pub mod print;
pub mod include;
pub mod if_tag;

///
///
///
pub fn parse<'a>(
    reader: HtmlTemplateReader,
    context: &'a mut base::ExtCtxt,
    name: ast::Ident
) -> HtmlState {

    let mut parser = Parser::new(
        context.parse_sess(),
        context.cfg(),
        box reader
    );

    let mut state = HtmlState::new(name);

    while !parser.reader.is_eof() {

        if !is_tag_start(&mut parser) {
            parser.bump();
            continue;
        }

        //TODO: certainly a better way to do "consume < and %"
        parser.bump();
        parser.bump();

        match parser.parse_ident().as_str() {

            TEMPLATE => {
                state.templates.push(
                    parse_template_tag(&mut parser, context)
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
