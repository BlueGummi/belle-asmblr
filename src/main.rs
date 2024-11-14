use belle_asm::*;
use colored::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
fn main() -> io::Result<()> {
    if CONFIG.debug {
        println!("Main func started.");
    }
    let mut lines: Vec<String> = Vec::new(); // vector to push lines onto
    let mut has_err: bool = false;
    if CONFIG.file.is_some() && !CONFIG.file.as_ref().unwrap().is_empty() {
        if CONFIG.debug {
            println!("File is Some");
        }
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
        println!("{}", "No input file specified".yellow());
        std::process::exit(1);
    }
    lines.retain(|line| !line.is_empty());
    for line in &mut lines {
        *line = line.trim().to_string();
    }
    lines.retain(|line| !line.starts_with(';')); // retain non-empty lines that don't start with ;
    if CONFIG.verbose || CONFIG.debug {
        println!("{}", "Processing lines:".blue());
        for line in &lines {
            println!("{}", line.green());
        }
    }

    let mut encoded_instructions = Vec::new();
    let mut line_count: u32 = 1; // bigger numbers with 32
    let mut write_to_file: bool = true; // defines if we should write to file (duh)
    for line in lines {
        let tokens = lex(&line, line_count);

        let instruction = tokens.first();
        let operand1 = tokens.get(1);
        let operand2 = {
            if let Some(Token::Comma) = tokens.get(2) {
                tokens.get(3)
            } else {
                tokens.get(2)
            }
        };
        if CONFIG.debug {
            println!("Raw line: {}", line.green());
        }
        if let Some(ins) = instruction {
            let encoded_instruction = encode_instruction(ins, operand1, operand2);
            if verify(ins, operand1, operand2, line_count) {
                write_to_file = false;
                has_err = true;
            }
            encoded_instructions.extend(&encoded_instruction.to_be_bytes());
            if CONFIG.verbose || CONFIG.debug {
                println!("Instruction: {:016b}", encoded_instruction);
            }
            let ins_str: String = format!("{:016b}", encoded_instruction);
            if CONFIG.debug {
                if let Some(ins) = ins_str.get(0..4) {
                    // fixed length instructions my beloved
                    println!("INS: {}", ins.blue().bold());
                }
                if let Some(dst) = ins_str.get(4..7) {
                    println!("DST: {}", dst.blue().bold());
                }
                if let Some(dtb) = ins_str.get(7..8) {
                    println!("DTB: {}", dtb.blue().bold());
                }
                if let Some(src) = ins_str.get(8..15) {
                    println!("SRC: {}", src.blue().bold());
                }
            }
        } else {
            println!(
                "{} to encode instruction for line: {}",
                "Not enough tokens".red().bold(),
                line.to_string().green()
            );
        }
        if CONFIG.debug {
            for token in tokens {
                println!(
                    "{} {}",
                    "Token:".green().bold(),
                    token.to_string().blue().bold()
                );
            }
        }
        line_count += 1; // line count exists so we can have line number errors
    }
    if has_err {
        eprintln!("{}", "Exiting...".red());
        std::process::exit(1); // wowzers, amazing
    }
    if CONFIG.debug {
        print_subroutine_map();
    }

    match &CONFIG.output {
        Some(output_file) if write_to_file => {
            write_encoded_instructions_to_file(output_file, &encoded_instructions)?;
        }
        _ => println!("some funny business happen on output writing :C"),
    }

    Ok(())
}
#[cfg(test)]
mod tests {
    // no tests
}
