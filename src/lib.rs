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
        Some(&Token::MemPointer(7)),
    );
    assert_eq!(result as u16, 0b0011010010000111);
}
#[test]
pub fn mul_check() {
    let result = encode_instruction(
        &Token::Ident("mul".to_string()),
        Some(&Token::Register(0)),
        Some(&Token::RegPointer(7)),
    );
    assert_eq!(result as u16, 0b1011000001000111);
}
#[test]
pub fn pop_check() {
    let result = encode_instruction(
        &Token::Ident("pop".to_string()),
        Some(&Token::Register(7)),
        None,
    );
    assert_eq!(result as u16, 0b1100000000000111);
}
#[test]
pub fn push_check() {
    let result = encode_instruction(
        &Token::Ident("push".to_string()),
        Some(&Token::Register(2)),
        None,
    );
    assert_eq!(result as u16, 0b1000000000000010);
}
#[test]
pub fn ret_check() {
    let result = encode_instruction(
        &Token::Ident("ret".to_string()),
        None,
        None,
    );
    assert_eq!(result as u16, 0b0101000000000000);
}
#[test]
pub fn call_check() {
    let result = encode_instruction(
        &Token::Ident("call".to_string()),
        Some(&Token::SRCall("func".to_string())),
        None,
    );
    assert_eq!(result as u16, 0b0100000000000000);
}
#[test]
pub fn ld_check() {
    let result = encode_instruction(
        &Token::Ident("ld".to_string()),
        Some(&Token::Register(5)),
        Some(&Token::Literal(1)),
    );
    assert_eq!(result as u16, 0b0110101100000001);
}
#[test]
pub fn st_check() {
    let result = encode_instruction(
        &Token::Ident("st".to_string()),
        Some(&Token::MemAddr(1)),
        Some(&Token::Register(4)),
    );
    assert_eq!(result as u16, 0b0111000000001100);
                               //           ^ 1 over here
}
#[test]
pub fn jz_check() {
    let result = encode_instruction(
        &Token::Ident("jz".to_string()),
        Some(&Token::MemAddr(8)),
        None,
    );
    assert_eq!(result as u16, 0b1001000000001000);
}
#[test]
pub fn int_check() {
    let result = encode_instruction(
        &Token::Ident("int".to_string()),
        Some(&Token::Literal(1)),
        None,
    );
    assert_eq!(result as u16, 0b1101000100000001);
}
#[test]
pub fn cmp_check() {
    let result = encode_instruction(
        &Token::Ident("cmp".to_string()),
        Some(&Token::Register(4)),
        Some(&Token::MemPointer(32)),
    );
    assert_eq!(result as u16, 0b1010100010100000);
}
