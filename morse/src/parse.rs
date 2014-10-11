use syntax::codemap;
use syntax::ext::base;
use syntax::parse::token;
use syntax::parse::parser::Parser;


use morse::MorseState;

/// Trait that means something can be parsed with a configuration.
pub trait Parse<Cfg> {
    /// Parse Self from a Parser and a configuration object.
    fn parse(&mut Parser, Cfg) -> Self;
}

static A:  &'static str = ".-";
static B:  &'static str = "-...";
static C: &'static str = "-.-.";

/// 
fn parse_morse_part(
    state: &mut MorseState,
    value: &str
) {
    match state.current_letter {
        Some(ref mut x) => {}, // x.push_str(value),
        None => state.current_letter = Some(value.to_string())
    }

    state.current_letter.map(|c| c.push_str(value));
}

///
fn validate_morse_character(
    state: &mut MorseState,
) {
    let letter = match state.current_letter {
        Some(ref a) if a.as_slice() == A => "A",
        Some(ref a) if a.as_slice() == B => "B",
        Some(ref a) if a.as_slice() == C => "C",
        _ => {println!("!?"); ""}
    };

    match state.full_message {
        Some(ref mut x) => x.push_str(letter),
        None => state.full_message = Some(letter.to_string())
    }
    state.current_letter = None;
}


///
///
///
impl<'a, 'b> Parse<(
    codemap::Span,
    &'a mut base::ExtCtxt<'b>
)> for MorseState {

    ///
    ///
    ///
    fn parse(
        parser: &mut Parser,
        (_, _): (codemap::Span, &'a mut base::ExtCtxt)
    ) -> MorseState {

        let mut state = MorseState::new();

        while parser.token != token::EOF {
            match parser.token {
                token::DOT => parse_morse_part(&mut state, "."),
                token::DOTDOT => parse_morse_part(&mut state, ".."),
                token::DOTDOTDOT => parse_morse_part(&mut state, "..."),
                token::BINOP(token::MINUS) => parse_morse_part(&mut state, "-"),
                token::UNDERSCORE => validate_morse_character(&mut state),
                _ => {
                    parser.fatal("only `_` `.` and  `-` are accepted");
                }
            }
            parser.bump();
        }
        validate_morse_character(&mut state);

        state
    }
}
