use syntax::codemap;
use syntax::ext::base;
use syntax::parse::token;
use syntax::parse::parser::Parser;

use syntax::codemap::Pos;

use morse::MorseState;

/// Trait that means something can be parsed with a configuration.
pub trait Parse<Cfg> {
    /// Parse Self from a Parser and a configuration object.
    fn parse(&mut Parser, Cfg) -> Self;
}

static A: &'static str = ".-";
static B: &'static str = "-...";
static C: &'static str = "-.-.";
static D: &'static str = "-..";
static E: &'static str = ".";
static F: &'static str = "..-.";
static G: &'static str = "--.";
static H: &'static str = "....";
static I: &'static str = "..";
static J: &'static str = ".---";
static K: &'static str = "-.-";
static L: &'static str = ".-..";
static M: &'static str = "--";
static N: &'static str = "-.";
static O: &'static str = "---";
static P: &'static str = ".--.";
static Q: &'static str = "--.-";
static R: &'static str = ".-.";
static S: &'static str = "...";
static T: &'static str = "-";
static U: &'static str = "..-";
static V: &'static str = "...-";
static W: &'static str = ".--";
static X: &'static str = "-..-";
static Y: &'static str = "-.--";
static Z: &'static str = "--..";

/// 
fn parse_morse_part(
    state: &mut MorseState,
    value: &str
) {
    match state.current_letter {
        Some(ref mut x) => x.push_str(value),
        None => state.current_letter = Some(value.to_string())
    }
}

///
fn validate_morse_character(
    state: &mut MorseState,
) {
    let letter = match state.current_letter {
        Some(ref a) if a.as_slice() == A => "A",
        Some(ref a) if a.as_slice() == B => "B",
        Some(ref a) if a.as_slice() == C => "C",
        Some(ref a) if a.as_slice() == D => "D",
        Some(ref a) if a.as_slice() == E => "E",
        Some(ref a) if a.as_slice() == F => "F",
        Some(ref a) if a.as_slice() == G => "G",
        Some(ref a) if a.as_slice() == H => "H",
        Some(ref a) if a.as_slice() == I => "I",
        Some(ref a) if a.as_slice() == J => "J",
        Some(ref a) if a.as_slice() == K => "K",
        Some(ref a) if a.as_slice() == L => "L",
        Some(ref a) if a.as_slice() == M => "M",
        Some(ref a) if a.as_slice() == N => "N",
        Some(ref a) if a.as_slice() == O => "O",
        Some(ref a) if a.as_slice() == P => "P",
        Some(ref a) if a.as_slice() == Q => "Q",
        Some(ref a) if a.as_slice() == R => "R",
        Some(ref a) if a.as_slice() == S => "S",
        Some(ref a) if a.as_slice() == T => "T",
        Some(ref a) if a.as_slice() == U => "U",
        Some(ref a) if a.as_slice() == V => "V",
        Some(ref a) if a.as_slice() == W => "W",
        Some(ref a) if a.as_slice() == X => "X",
        Some(ref a) if a.as_slice() == Y => "Y",
        Some(ref a) if a.as_slice() == Z => "Z",
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
                _ => {
                    parser.fatal("only `_` `.` and  `-` are accepted");
                }
            }
            parser.bump();
            if (parser.span.lo != parser.last_span.hi) {
                validate_morse_character(&mut state);
            }
            println!("span {} {}", parser.span.lo.to_uint(), parser.span.hi.to_uint());
            println!("last span {} {}", parser.last_span.lo.to_uint(), parser.last_span.hi.to_uint());
        }
        validate_morse_character(&mut state);

        state
    }
}
