use crate::*;
pub fn verify(ins: &Token, arg1: Option<&Token>, arg2: Option<&Token>, line_num: u32) -> bool {
    let instructions = [
        "HLT", "ADD", "AND", "OR", "CALL", "RET", "LD", "ST", "JMP", "JZ", "MUL", "SHL", "SHR",
        "INT", "MOV",
    ];
    let raw_token = ins.get_raw().to_uppercase();
    let mut has_error: bool = false;
    let mut err_msg: String = String::from("");
    if let Token::Ident(_) = ins {
        if instructions.contains(&raw_token.as_str()) {
            match raw_token.as_str() {
                "HLT" | "RET" => {
                    if is_arg(arg1) | is_arg(arg2) {
                        err_msg = format!("{} does not take any arguments", raw_token);
                        has_error = true;
                    }
                }
                "ADD" | "AND" | "OR" | "LD" | "ST" | "MOV" | "MUL" => {
                    if !is_arg(arg1) | !is_arg(arg2) {
                        err_msg = format!("{} requires two arguments", raw_token);
                        has_error = true;
                    }
                }
                "SHL" | "SHR" | "INT" => {
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

fn is_arg(tok_to_check: Option<&Token>) -> bool {
    if tok_to_check.is_none() {
        return false;
    }
    if tok_to_check.is_some() {
        return matches!(
            tok_to_check.unwrap(),
            Token::Register(_) | Token::Literal(_) | Token::SRCall(_) | Token::MemAddr(_)
        );
    }
    false
}
