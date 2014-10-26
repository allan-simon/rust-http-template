use syntax::ext::tt::transcribe::TtReader;
use syntax::parse::lexer;
use syntax::parse::token;
use syntax::codemap;

/// This reader is no more than a wrapper around TtReader to handle
/// "%>" as one token
pub struct HtmlTemplateReader<'a> {
    pub reader : TtReader<'a>,
    pub buffered: Option<lexer::TokenAndSpan>
}

impl<'a> HtmlTemplateReader<'a> {
    pub fn new<'a>(reader: TtReader<'a>) -> HtmlTemplateReader<'a> {
        HtmlTemplateReader {
            reader: reader,
            buffered: None
        }
    }
}

/// except for the special case of handling "%>" as EOF, otherwise
/// we delegate everything to the TtReader
impl<'a> lexer::Reader for HtmlTemplateReader<'a> {

    fn is_eof(&self) -> bool { self.reader.is_eof() }
    fn fatal(&self, string: &str) -> ! { self.reader.fatal(string) }
    fn err(&self, string: &str) { self.reader.err(string) }
    fn peek(&self) -> lexer::TokenAndSpan { self.reader.peek()}

    fn next_token(&mut self) -> lexer::TokenAndSpan {

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
        
        token
    }
}
