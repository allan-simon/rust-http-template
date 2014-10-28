use syntax::ext::tt::transcribe::TtReader;
use syntax::parse::lexer;
use syntax::parse::lexer::Reader;
use syntax::parse::token;
use syntax::codemap;

/// This reader is no more than a wrapper around TtReader to handle
/// "%>" as one token
pub struct HtmlTemplateReader<'a> {
    reader : TtReader<'a>,
    buffered: Option<lexer::TokenAndSpan>,
    current: lexer::TokenAndSpan,
    // to differiante current containing EOF because
    // we have %> and EOF because it's actually the end of file
    is_fake_eof: bool
}

impl<'a> HtmlTemplateReader<'a> {
    pub fn new<'a>(reader: TtReader<'a>) -> HtmlTemplateReader<'a> {
        let current = reader.peek();
        HtmlTemplateReader {
            reader: reader,
            buffered: None,
            current: current,
            is_fake_eof: false
        }
    }
}

/// except for the special case of handling "%>" as EOF, otherwise
/// we delegate everything to the TtReader
impl<'a> lexer::Reader for HtmlTemplateReader<'a> {

    fn is_eof(&self) -> bool {
        self.current.tok == token::EOF && self.is_fake_eof == false
    }
    fn fatal(&self, string: &str) -> ! { self.reader.fatal(string) }
    fn err(&self, string: &str) { self.reader.err(string) }
    fn peek(&self) -> lexer::TokenAndSpan { self.current.clone() }

    fn next_token(&mut self) -> lexer::TokenAndSpan {

        self.is_fake_eof = false;

        // if we have something in the buffer, we consume it
        // without requesting a token to underlying reader
        let buffered = self.buffered.take();
        match buffered {
            Some(x) => {
                return x;
            }
            None => ()
        };

        let token = self.reader.next_token(); 

        match token.tok {
            token::BINOP(token::PERCENT) => {
                let next_token  = self.reader.next_token(); 
                match next_token.tok {
                    token::GT => {
                        self.is_fake_eof = true;
                        return lexer::TokenAndSpan {
                            tok: token::EOF,
                            sp: codemap::mk_sp(
                                token.sp.lo,
                                next_token.sp.hi
                            )
                        };
                    }
                    _ => self.buffered = Some(next_token)
                }  

            }
            _ => ()
        };

        self.current = token.clone();
        
        token
    }
}
