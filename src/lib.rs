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
#[test]
pub fn mov_check() {
    let result = encode_instruction(
        &Token::Ident("mov".to_string()),
        Some(&Token::Register(0)),
        Some(&Token::Literal(4)),
    );
    assert_eq!(result.unsigned_abs() as u16, 0b01111011111100);
} // I don't know why it's signed and unsigned sometimes
  // weird
#[test]
pub fn hlt_check() {
    let result = encode_instruction(&Token::Ident("hlt".to_string()), None, None);
    assert_eq!(result, 0);
}
#[test]
pub fn add_check() {
    let result = encode_instruction(
        &Token::Ident("add".to_string()),
        Some(&Token::Register(6)),
        Some(&Token::Literal(4)),
    );
    assert_eq!(result as u16, 0b0001110100000100);
}
#[test]
pub fn sr_check() {
    let result = encode_instruction(&Token::SR("powwow".to_string()), None, None);
    assert_eq!(result as u16, 0b1111000000000001);
}
#[test]
pub fn and_check() {
    let result = encode_instruction(
        &Token::Ident("and".to_string()),
        Some(&Token::Register(4)),
        Some(&Token::Register(0)),
    );
    assert_eq!(result as u16, 0b0010100000000000);
}
#[test]
pub fn or_check() {
    let result = encode_instruction(
        &Token::Ident("or".to_string()),
        Some(&Token::Register(2)),
        Some(&Token::Literal(7)),
    );
    assert_eq!(result as u16, 0b0011010100000111);
}
