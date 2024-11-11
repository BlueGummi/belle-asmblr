use crate::*;
use std::fmt;
#[derive(Debug)]
pub enum Token {
    Ident(String),
    Register(i16),
    Comma,
    AndSign,
    Literal(i16),
    NewLine,
    Semicolon,
    Eol,
    SRCall(String),
    SR(String),
    MemAddr(i16),
}
impl Token {
    pub fn get_raw(&self) -> String {
        match self {
            Token::Ident(s) => s.to_string(),
            Token::Register(n) => n.to_string(),
            Token::Comma => "comma".to_string(),
            Token::AndSign => "and_sign".to_string(),
            Token::Literal(n) => n.to_string(),
            Token::NewLine => "newline".to_string(),
            Token::Eol => "eol".to_string(),
            Token::Semicolon => "semicolon".to_string(),
            Token::SRCall(s) => s.to_string(),
            Token::SR(s) => s.to_string(),
            Token::MemAddr(n) => n.to_string(),
        }
    }
}
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if CONFIG.verbose {
            match self {
                Token::Ident(s) => write!(f, "ident(\"{}\") length: [{}]", s, s.len()),
                Token::Register(n) => write!(f, "register({})", n),
                Token::Comma => write!(f, "comma"),
                Token::AndSign => write!(f, "and_sign"),
                Token::Literal(n) => write!(f, "number_literal({})", n),
                Token::NewLine => write!(f, "new_line"),
                Token::Eol => writeln!(f, "eol"),
                Token::Semicolon => write!(f, "semicolon"),
                Token::SRCall(s) => write!(f, "SRCall({})", s),
                Token::SR(s) => write!(f, "Subroutine({})", s),
                Token::MemAddr(n) => write!(f, "MemAddr({})", n),
            }
        } else {
            Ok(())
        }
    }
}
