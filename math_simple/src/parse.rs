use syntax::{ast, codemap};
use syntax::ext::base;
use syntax::parse::token;
use syntax::ptr::P;
use syntax::parse::parser::Parser;


use math_expr::MathExpState;

/// Trait that means something can be parsed with a configuration.
pub trait Parse<Cfg> {
    /// Parse Self from a Parser and a configuration object.
    fn parse(&mut Parser, Cfg) -> Self;
}

static ONE:  &'static str = "one";
static TWO:  &'static str = "two";
static PLUS: &'static str = "plus";

///
///
fn parse_operand(
    state: &mut MathExpState,
    value: uint,
    parser: &mut Parser
) {

    if state.left.is_none() {
        state.left = Some(value);
        return;
    }
    // left is already filled

    if state.operator.is_none() {
        parser.fatal("needs an operator here");
    }

    if state.right.is_some() {
        parser.fatal("math_expr only accepts simple operation");
    }
    state.right = Some(value);
}

/// 
fn parse_operator(
    state: &mut MathExpState,
    value: uint,
    parser: &mut Parser
) {
    if state.left.is_none() {
        parser.fatal("operator can't be first");
    }

    if state.right.is_some() {
        parser.fatal("math_expr only accepts simple operation");
    }

    if state.operator.is_some() {
        parser.fatal("there's already an operator");
    }

    state.operator = Some(value);
}

///
///
///
impl<'a, 'b> Parse<(
    codemap::Span,
    &'a mut base::ExtCtxt<'b>
)> for MathExpState {

    ///
    ///
    ///
    fn parse(
        parser: &mut Parser,
        (sp, cx): (codemap::Span, &'a mut base::ExtCtxt)
    ) -> MathExpState {

        let mut state = MathExpState::new();

        // Now parse all tests and subsections:
        while
            parser.token != token::RBRACE &&
            parser.token != token::EOF
        {
            let ident_name = parser.parse_ident();

            match ident_name.as_str() {
                ONE => {
                    parse_operand(&mut state, 1, parser);
                },
                TWO => {
                    parse_operand(&mut state, 2, parser);
                },
                PLUS => {
                    parse_operator(&mut state, 1, parser);
                },
                otherwise => {println!("woot?! {}", otherwise);}
            }
        }

        state
    }
}
