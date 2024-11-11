use crate::*;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

pub static SUBROUTINE_MAP: Lazy<Mutex<HashMap<String, u32>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
static SUBROUTINE_COUNTER: Lazy<Mutex<u32>> = Lazy::new(|| Mutex::new(1));

pub fn print_subroutine_map() {
    let map = SUBROUTINE_MAP.lock().unwrap();
    for (name, counter) in map.iter() {
        if CONFIG.verbose {
            println!("Subroutine: {}, Counter: {}", name, counter);
        }
    }
}

pub fn lex(line: &str, line_number: u32) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            ' ' | '\t' => continue,
            '\n' => tokens.push(Token::NewLine),
            ',' => tokens.push(Token::Comma),
            ';' => {
                tokens.push(Token::Semicolon);
                break;
            }
            '&' => tokens.push(Token::AndSign),
            '%' => {
                let mut reg = String::new();
                reg.push(c);
                if let Some(&next) = chars.peek() {
                    if next == 'r' || next == 'R' {
                        reg.push(chars.next().unwrap());
                    } else {
                        eprintln!("Expected 'r' or 'R' after '%': line {}", line_number);
                        break;
                    }
                } else {
                    eprintln!(
                        "Expected register identifier after '%': line {}",
                        line_number
                    );
                    break;
                }
                while let Some(&next) = chars.peek() {
                    if next.is_ascii_digit() {
                        reg.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                if reg.len() > 2 {
                    if let Ok(reg_num) = reg.trim()[2..].parse::<i16>() {
                        tokens.push(Token::Register(reg_num));
                    } else {
                        eprintln!("Invalid register number: line {}", line_number);
                        break;
                    }
                } else {
                    eprintln!("Register must have a number: line {}", line_number);
                    break;
                }
            }
            '@' => {
                let mut subroutine_call = String::new();
                while let Some(&next) = chars.peek() {
                    if next.is_alphanumeric() || next == '_' {
                        subroutine_call.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                tokens.push(Token::SRCall(subroutine_call));
            }
            'a'..='z' | 'A'..='Z' => {
                let mut ident = String::new();
                ident.push(c);
                while let Some(&next) = chars.peek() {
                    if next.is_alphanumeric() || next == '_' {
                        ident.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                if let Some(&next) = chars.peek() {
                    if next == ':' {
                        chars.next();
                        let mut map = SUBROUTINE_MAP.lock().unwrap();
                        if !map.contains_key(&ident) {
                            let mut counter = SUBROUTINE_COUNTER.lock().unwrap();
                            map.insert((*ident).to_string(), *counter);
                            *counter += 1;
                            tokens.push(Token::SR(ident));
                        } else {
                            eprintln!(
                                "Duplicate subroutine declaration: '{}': line {}",
                                ident, line_number
                            );
                        }
                    } else {
                        tokens.push(Token::Ident(ident));
                    }
                } else {
                    tokens.push(Token::Ident(ident));
                }
            }
            '#' => {
                let mut number = c.to_string();
                if let Some(&next) = chars.peek() {
                    if next == '-' {
                        number.push(chars.next().unwrap());
                    }
                }

                while let Some(&next) = chars.peek() {
                    if next.is_ascii_digit() {
                        number.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }

                let num_value = match number[1..].parse::<i16>() {
                    Ok(value) => value,
                    Err(_) => {
                        eprintln!(
                            "Value after # must be numeric literal: line {}",
                            line_number
                        );
                        eprintln!("{}", number);
                        std::process::exit(1);
                    }
                };

                if num_value > 128 || num_value < -128 {
                    eprintln!(
                        "Numeric literal cannot be over +/- 128: line {}",
                        line_number
                    );
                    std::process::exit(1);
                }

                // if it is less than 0, bitflip first bit
                let stored_value = if num_value < 0 {
                    // first positive value as sign bit
                    let positive_value = num_value.abs() as u8; // convert to positive
                    (positive_value & 0x7F) | 0x80 // set the sign bit (flip first bit)
                } else {
                    num_value as u8
                };

                tokens.push(Token::Literal(stored_value as i16));
            }
            '$' => {
                let mut addr = c.to_string();
                while let Some(&next) = chars.peek() {
                    if next.is_ascii_digit() {
                        addr.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                if addr[1..].parse::<i16>().is_err() {
                    eprintln!(
                        "Value after $ must be numeric, 0-4,096: line {}",
                        line_number
                    );
                    std::process::exit(1);
                }
                let addr_val = addr[1..].parse::<i16>().unwrap();
                if addr_val >= 4096 || addr_val <= 0 {
                    eprintln!("Address must be between 0-4,096: line {}", line_number);
                    std::process::exit(1);
                }
                tokens.push(Token::MemAddr(addr_val));
            }
            _ => {
                eprintln!("Unknown character: {}: line {}", c, line_number);
            }
        }
    }

    tokens.push(Token::Eol);
    tokens
}
