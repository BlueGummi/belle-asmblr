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
            if CONFIG.debug {
                println!("{:b}", num);
            }
            *num
        }
        Some(Token::Literal(literal)) => (1 << 8) | *literal,
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
                    eprintln!("Subroutine \"{}\" does not exist.", sr.bold().red());
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
    let mut is_st: bool = false;
    let mut is_one_arg: bool = false;
    let instruction_bin = match ins {
        Token::Ident(ref instruction) => match instruction.to_uppercase().as_str() {
            "HLT" => HLT_OP, // 0
            "ADD" => ADD_OP, // 1
            "AND" => AND_OP, // 2
            "OR" => OR_OP,   // 3
            "CALL" => {
                if CONFIG.debug {
                    println!("this is a call!");
                }
                is_one_arg = true;
                CALL_OP // 4
            }
            "RET" => RET_OP, // 5
            "LD" => LD_OP,   // 6
            "ST" => {
                is_st = true;
                ST_OP // 7
            }
            "JMP" => {
                is_one_arg = true;
                JMP_OP // 8
            }
            "JZ" => {
                is_one_arg = true;
                JZ_OP // 9
            }
            "CMP" => CMP_OP, // 10
            "MUL" => MUL_OP, // 11
            "NOP" => NOP_OP, // 12
            "INT" => {
                is_one_arg = true;
                INT_OP // 13
            }
            "MOV" => MOV_OP, // 14

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
                SR_OP
            } else {
                HLT_OP
            }
        }
    };
    if subr {
        return (instruction_bin << 12) | register_to_binary(Some(ins));
    }
    if is_one_arg {
        return (instruction_bin << 12) | register_to_binary(reg1);
    }
    if is_st {
        return (instruction_bin << 12)
            | (register_to_binary(reg1) << 3)
            | register_to_binary(reg2);
    }

    let register_bin1 = register_to_binary(reg1);
    let register_bin2 = register_to_binary(reg2);
    (instruction_bin << 12) | (register_bin1 << 9) | register_bin2
}
