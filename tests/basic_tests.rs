use encode::*;
use tokens::*;
#[test]
pub fn basic_checks() {
    let result = encode_instruction(
        Token::Ident("mov".to_string()),
        Token::Register(0),
        Token::Literal(4),
    );
    assert_eq!(result, 0b1110000100000100);
}
