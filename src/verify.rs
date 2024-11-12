use crate::*;
pub fn verify(ins: &Token, arg1: Option<&Token>, arg2: Option<&Token>, line_num: u32) -> bool {
    let instructions = [
        "HLT", "ADD", "AND", "OR", "CALL", "RET", "LD", "ST", "JMP", "JZ", "MUL", "CMP", "NOP",
        "INT", "MOV",
    ]; // I can't think of a better way to do this
    let raw_token = ins.get_raw().to_uppercase(); // has to be uppercase
    let mut has_error: bool = false; // this is a weird way to handle errors
    let mut err_msg: String = String::from("");
    if let Token::Ident(_) = ins {
        if instructions.contains(&raw_token.as_str()) {
            match raw_token.as_str() {
                // boolean bonanza
                "HLT" | "RET" | "NOP" => {
                    if is_arg(arg1) | is_arg(arg2) {
                        err_msg = format!("{} does not take any arguments", raw_token);
                        has_error = true;
                    }
                }
                "ADD" | "AND" | "OR" | "LD" | "ST" | "MOV" | "MUL" | "CMP" => {
                    if !is_arg(arg1) | !is_arg(arg2) {
                        err_msg = format!("{} requires two arguments", raw_token);
                        has_error = true;
                    }
                }
                "INT" => {
                    let args_satisfied =
                        (is_arg(arg1) && is_arg(arg2)) | (is_arg(arg1) && !is_arg(arg2));
                    if !args_satisfied {
                        err_msg = format!("{} requires one or two arguments", raw_token);
                        has_error = true;
                    }
                }
                "CALL" | "JMP" | "JZ" => {
                    if CONFIG.debug {
                        println!("arg1 {:?}, arg2 {:?}", arg1, arg2);
                    }
                    if !is_arg(arg1) | is_arg(arg2) {
                        err_msg = format!("{} requires one argument", raw_token);
                        has_error = true;
                    }
                }

                _ => {
                    err_msg = "instruction not covered".to_string();
                    has_error = true;
                }
            }
        }
    }
    if has_error {
        eprintln!("Err: \"{}\" on line {}", err_msg, line_num);
    }
    has_error
}
// this is all self-explanatory, wait till you see lex.rs
fn is_arg(tok_to_check: Option<&Token>) -> bool {
    if tok_to_check.is_none() {
        return false;
    }
    if tok_to_check.is_some() {
        return matches!(
            tok_to_check.unwrap(),
            Token::Register(_) | Token::Literal(_) | Token::SRCall(_) | Token::MemAddr(_) | Token::MemPointer(_) | Token::RegPointer(_)
        );
    }
    false
}
