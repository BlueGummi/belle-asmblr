use crate::*;
use std::fmt;
#[derive(Debug)]
// self explanatory, you got this
pub enum Token {
    Ident(String),
    Register(i16),
    Comma,
    Literal(i16),
    NewLine,
    Semicolon,
    Eol,
    SRCall(String),
    SR(String),
    MemAddr(i16),
    Label(String),
    RegPointer(i16),
    MemPointer(i16),
    Value(String),
}
impl Token {
    pub fn get_raw(&self) -> String {
        match self {
            Token::Ident(s) => s.to_string(),
            Token::Register(n) => n.to_string(),
            Token::Comma => "comma".to_string(),
            Token::Literal(n) => n.to_string(),
            Token::NewLine => "newline".to_string(),
            Token::Eol => "eol".to_string(),
            Token::Semicolon => "semicolon".to_string(),
            Token::SRCall(s) => s.to_string(),
            Token::SR(s) => s.to_string(),
            Token::MemAddr(n) => n.to_string(),
            Token::Label(s) => s.to_string(),
            Token::RegPointer(n) => n.to_string(),
            Token::MemPointer(n) => n.to_string(),
            Token::Value(s) => s.to_string(),
        }
    }
}
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if CONFIG.verbose || CONFIG.debug {
            match self {
                Token::Ident(s) => write!(f, "Ident(\"{}\") Length: [{}]", s, s.len()),
                Token::Register(n) => write!(f, "Register({})", n),
                Token::Comma => write!(f, "Comma"),
                Token::Literal(n) => write!(f, "Number Literal({})", n),
                Token::NewLine => write!(f, "Newline"),
                Token::Eol => writeln!(f, "Eol"),
                Token::Semicolon => write!(f, "Semicolon"),
                Token::SRCall(s) => write!(f, "SRCall({})", s),
                Token::SR(s) => write!(f, "Subroutine({})", s),
                Token::MemAddr(n) => write!(f, "MemAddr({})", n),
                Token::Label(s) => write!(f, "Label({})", s),
                Token::RegPointer(n) => write!(f, "Reg Pointer({})", n),
                Token::MemPointer(n) => write!(f, "Mem Pointer({})", n),
                Token::Value(s) => write!(f, "Value({})", s),
            }
        } else {
            Ok(())
        }
    }
}
