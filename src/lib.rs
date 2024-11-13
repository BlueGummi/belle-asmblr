mod config;
mod encode;
mod instructions;
mod lex;
mod tokens;
mod verify;
pub use config::*;
pub use encode::*;
pub use instructions::*;
pub use lex::*;
pub use tokens::*;
pub use verify::*;
use colored::Colorize;
#[test]
pub fn basic_checks() {
    let mut ident = Token::Ident("mov".to_string());
    let op1 = Some(&Token::Register(0));
    let op2 = Some(&Token::Literal(4));
    let result = encode_instruction(&ident, op1, op2);
    assert_eq!(result.abs() as u32, 0b1111011111100);
    println!("{}", "MOV check complete!".blue());

    ident = Token::Ident("hlt".to_string());
    let result = encode_instruction(&ident, None, None);
    assert_eq!(result, 0);
}
pub fn sr_check() {
    let ident = Token::SR("powwow".to_string());
    let result = encode_instruction(&ident, None, None);
    assert_eq!(result.abs() as u32, 0b1111000000000001);
}

