use belle_asm::*;
#[test]
pub fn basic_checks() {
    let result = encode_instruction(
        &Token::Ident("mov".to_string()),
        Some(&Token::Register(0)),
        Some(&Token::Literal(4)),
    );
    assert_eq!(result, -7932);
}
