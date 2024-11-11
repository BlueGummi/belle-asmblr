use crate::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

pub fn register_to_binary(reg: Option<&Token>) -> i16 {
    match reg {
        Some(Token::Register(num)) => {
            if *num > 8 {
                eprintln!("{}", "Register value cannot be greater than 7".bold().red());
            }
            *num
        }
        Some(Token::Literal(literal)) => (1 << 7) | *literal,
        Some(Token::SR(sr)) | Some(Token::SRCall(sr)) => {
            let map = SUBROUTINE_MAP.lock().unwrap();
            let subroutine_value = map.get(sr);
            if let Some(value) = subroutine_value {
                *value as i16
            } else {
                let mut subroutine_counter = 1;
                let mut subroutine_map = HashMap::new();

                let file_result = File::open(Path::new(CONFIG.file.as_ref().unwrap()));
                if file_result.is_err() {
                    println!("File not found");
                    std::process::exit(1);
                }

                for line in io::BufReader::new(file_result.unwrap())
                    .lines()
                    .map_while(Result::ok)
                {
                    let line = line.split(';').next().unwrap_or(&line);
                    if line.ends_with(':') {
                        let subroutine_name = line.trim_end_matches(':').trim().to_string();
                        subroutine_map.insert(subroutine_name, subroutine_counter);
                        subroutine_counter += 1;
                    }
                }
                if !subroutine_map.contains_key(sr) {
                    eprintln!("Subroutine \"{}\" does not exist.", sr);
                }

                return *subroutine_map.get(sr).unwrap_or(&0);
            }
        }
        Some(Token::MemAddr(n)) => *n,
        _ => 0,
    }
}

pub fn write_encoded_instructions_to_file(
    filename: &str,
    encoded_instructions: &[u8],
) -> io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(encoded_instructions)?;
    println!("Wrote to file.");
    Ok(())
}
pub fn encode_instruction(ins: &Token, reg1: Option<&Token>, reg2: Option<&Token>) -> i16 {
    let mut subr: bool = false;
    let mut is_call: bool = false;
    let instruction_bin = match ins {
        Token::Ident(ref instruction) => match instruction.to_uppercase().as_str() {
            "HLT" => 0b0000, // 0
            "ADD" => 0b0001, // 1
            "AND" => 0b0010, // 2
            "OR" => 0b0011,  // 3
            "CALL" => {
                if CONFIG.debug {
                    println!("this is a call!");
                }
                is_call = true;
                0b0100 // 4
            }
            "RET" => 0b0101, // 5
            "LD" => 0b0110,  // 6
            "ST" => 0b0111,  // 7
            "JMP" => 0b1000, // 8
            "JZ" => 0b1001,  // 9
            "CMP" => 0b1010, // 10
            "SHL" => 0b1011, // 11
            "SHR" => 0b1100, // 12
            "INT" => 0b1101, // 13
            "MOV" => 0b1110, // 14

            _ => {
                eprintln!("Instruction not recognized: {}", instruction);
                std::process::exit(1);
            }
        },
        _ => {
            if let Token::SR(_) = ins {
                subr = true;
                if CONFIG.debug {
                    println!("Subroutine detected");
                }
                0b1111
            } else {
                0b0000
            }
        }
    };
    if is_call {
        return (instruction_bin << 12) | register_to_binary(reg1);
    }
    if subr {
        return (instruction_bin << 12) | register_to_binary(Some(ins));
    }
    let register_bin1 = register_to_binary(reg1);
    let register_bin2 = register_to_binary(reg2);
    (instruction_bin << 12) | (register_bin1 << 8) | register_bin2
}
