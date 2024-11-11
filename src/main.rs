mod config;
use config::*;
mod encode;
use encode::*;
mod lex;
use lex::*;
mod tokens;
use tokens::*;
mod verify;
use colored::*;
use once_cell::sync::Lazy;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use verify::*;

static CONFIG: Lazy<Args> = Lazy::new(declare_config);
fn main() -> io::Result<()> {
    let mut lines: Vec<String> = Vec::new();
    let mut has_err: bool = false;
    if CONFIG.file.is_some() {
        let file = File::open(Path::new(CONFIG.file.as_ref().unwrap()))?;
        for line in io::BufReader::new(file).lines() {
            match line {
                Ok(content) => lines.push(content),
                Err(e) => eprintln!(
                    "{} reading line from file: {}",
                    "Error".red().bold(),
                    e.to_string().green()
                ),
            }
        }
    } else {
        println!(
            "{}",
            "No input file specified, defaulting to default ASM code.".yellow()
        );
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

    if CONFIG.verbose {
        println!("{}", "Processing lines:".blue());
        for line in &lines {
            println!("{}", line.green());
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
            if CONFIG.verbose {
                println!("Instruction: {:016b}", encoded_instruction);
                let ins_str: String = format!("{:016b}", encoded_instruction);
                if let Some(ins) = ins_str.get(0..4) {
                    println!("INS: {}", ins);
                }
                if let Some(dst) = ins_str.get(4..7) {
                    println!("DST: {}", dst);
                }
            }
        } else {
            println!(
                "{} to encode instruction for line: {}",
                "Not enough tokens".red().bold(),
                line.to_string().green()
            );
        }
        if CONFIG.verbose {
            for token in tokens {
                println!("{}", token.to_string().blue().bold());
            }
        }
        line_count += 1;
    }
    if has_err {
        eprintln!("{}", "Exiting...".red());
        std::process::exit(1);
    }
    print_subroutine_map();

    if let Some(output_file) = &CONFIG.output {
        if write_to_file {
            write_encoded_instructions_to_file(output_file, &encoded_instructions)?;
        }
    }

    Ok(())
}
