mod config;
mod encode;
mod lex;
mod tokens;
mod verify;
use config::*;
use encode::*;
use lex::*;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use tokens::*;
use verify::*;

fn main() -> io::Result<()> {
    let config = declare_config();
    let mut lines: Vec<String> = Vec::new();
    let mut has_err: bool = false;
    if config.file.is_some() {
        let file = File::open(Path::new(&config.file.unwrap()))?;
        for line in io::BufReader::new(file).lines() {
            match line {
                Ok(content) => lines.push(content),
                Err(e) => eprintln!("Error reading line from file: {}", e),
            }
        }
    } else {
        println!("No input file specified, defaulting to default ASM code.");
        lines.push("mov %r0, #63".to_string());
        lines.push("add %r2, %r3 ; blah blah".to_string());
        lines.push("beq #43".to_string());
        lines.push("add %r4, #69".to_string());
    }
    lines.retain(|line| !line.is_empty());
    for line in &mut lines {
        *line = line.trim().to_string();
    }
    lines.retain(|line| !line.starts_with(';'));

    if config.verbose {
        println!("Processing lines:");
        for line in &lines {
            println!("{}", line);
        }
    }

    let mut encoded_instructions = Vec::new();
    let mut line_count: u32 = 1;
    let mut write_to_file: bool = true;
    for line in lines {
        let tokens = lex(&line, line_count);

        let instruction = tokens.first();
        let register1 = tokens.get(1);
        let register2 = tokens.get(3);

        if let Some(ins) = instruction {
            let encoded_instruction = encode_instruction(ins, register1, register2);
            if verify(ins, register1, register2, line_count) {
                write_to_file = false;
                has_err = true;
            }
            encoded_instructions.extend(&encoded_instruction.to_be_bytes());
            if config.verbose {
                println!("Instruction: {:016b}", encoded_instruction);
            }
        } else {
            println!("Not enough tokens to encode instruction for line: {}", line);
        }
        if config.verbose {
            for token in tokens {
                println!("{}", token);
            }
        }
        line_count += 1;
    }
    if has_err {
        eprintln!("Exiting...");
        std::process::exit(1);
    }
    print_subroutine_map();

    if let Some(output_file) = &config.output {
        if write_to_file {
            write_encoded_instructions_to_file(output_file, &encoded_instructions)?;
        }
    }

    Ok(())
}
