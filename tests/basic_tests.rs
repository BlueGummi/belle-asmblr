use belle_asm::*;
#[test]
pub fn basic_checks() {
    let mut ident = Token::Ident("mov".to_string());
    let mut op1 = Some(&Token::Register(0));
    let mut op2 = Some(&Token::Literal(4));
    let result = encode_instruction(&ident, op1, op2);
    assert_eq!(result, -0b1111011111100);

    ident = Token::Ident("nop".to_string());
    let result = encode_instruction(&ident, None, None);
    assert_eq!(result, 0);
    let result = encode_instruction(&ident, op1, op2);

    let result = encode_instruction(&ident, op1, op2);

    let result = encode_instruction(&ident, op1, op2);

    let result = encode_instruction(&ident, op1, op2);
}
