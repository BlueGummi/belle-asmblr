use crate::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

pub fn register_to_binary(reg: Option<&Token>) -> i16 {
    // this is some really stupid stuff
    match reg {
        // register
        Some(Token::Register(num)) => {
            if *num > 8 {
                eprintln!("{}", "Register value cannot be greater than 7".bold().red());
            }
            if CONFIG.debug {
                println!("REG {:b}", num);
            }
            *num
        }
        // all looks good
        Some(Token::Literal(literal)) => (1 << 8) | *literal,

        // uh oh... what on earth...
        Some(Token::SR(sr)) | Some(Token::SRCall(sr)) => {
            // we're gonna lock the hashmap to get stuff from it
            let map = SUBROUTINE_MAP.lock().unwrap();
            let subroutine_value = map.get(sr);
            // it's gonna see if it exists and return the key if it does
            if let Some(value) = subroutine_value {
                *value as i16
            } else {
                /*
                 * now this is the real buffoonery
                 * we will read from the input file, and load every subroutine we see into a
                 * hashmap (which should be identical to the big public one) to allow for
                 * subroutine hoisting, essentially.
                 * */
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
                // this is utterly ludicrous
                {
                    let line = line.split(';').next().unwrap_or(&line); // delete comments
                    if line.ends_with(':') {
                        // add this to the hashmap for subroutines
                        let subroutine_name = line.trim_end_matches(':').trim().to_string();
                        subroutine_map.insert(subroutine_name, subroutine_counter); // what on
                                                                                    // earth am i
                                                                                    // doing
                        subroutine_counter += 1;
                    }
                }
                if !subroutine_map.contains_key(sr) {
                    eprintln!("Subroutine \"{}\" does not exist.", sr.bold().red());
                }

                return *subroutine_map.get(sr).unwrap_or(&0); // all this for a single number
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
    // pretty obvious
    let mut file = File::create(filename)?;
    file.write_all(encoded_instructions)?;
    println!("Wrote to file.");
    Ok(())
}
pub fn encode_instruction(ins: &Token, reg1: Option<&Token>, reg2: Option<&Token>) -> i16 {
    // these booleans will define instruction encoding once set to true
    // different instructions are encoded differently
    /*  Standard instruction encoding, e.g. ADD, MUL
     *  Let's take ADD as our example
     *  0001 111 1 00101001 <- these last 8 bits are a num, if the first bit is on it is neg.
     *  ^    ^   ^This standalone '1' is the determinant bit, if it is on, the next value is
     *  |    |    a literal, if off, it is a register
     *  |    |
     *  |    These next 3 bits are the destination register, max value of 7
     *  These first four bits denote the opcode. opcodes are always 4 bits long.
     */

    /* now let's look at the encoding for an instruction such as ST, storing a value from register
     * to memory
     * 0111 1101 10101 001
     * ^    ^          ^ Last 3 bits denote SOURCE register.
     * |    |These 9 bits denote a memory address, max 512
     * | Opcode
     */

    // for other instructions, such as RET and HLT, I should implement variants with arguments to
    // denote things such as the .start keyword, like 00001000 VALUE000

    let mut subr: bool = false;
    let mut is_st: bool = false;
    let mut is_one_arg: bool = false;
    let instruction_bin = match ins {
        Token::Ident(ref instruction) => match instruction.to_uppercase().as_str() {
            // self
            // explanatory
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
        return (instruction_bin << 12) | register_to_binary(Some(ins)); // subroutines push ins to
                                                                        // the left and keep the
                                                                        // subr on the right
    }
    if is_one_arg {
        return (instruction_bin << 12) | register_to_binary(reg1); // one arg opcodes, same
                                                                   // encoding as subr but reg1
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
