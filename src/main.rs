use belle_asm::*;
use colored::*;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    if CONFIG.debug {
        println!("Main func started.");
    }
    let mut lines: Vec<String> = Vec::new();
    let mut has_err: bool = false;
    if CONFIG.file.is_some() && !CONFIG.file.as_ref().unwrap().is_empty() {
        if CONFIG.debug {
            println!("File is Some");
        }
        if !File::open(Path::new(CONFIG.file.as_ref().unwrap())).is_ok() {
            println!(
                "{}{}{}",
                "error: ".red().bold(),
                CONFIG.file.as_ref().unwrap().green(),
                " no such file or directory".red()
            );
            std::process::exit(1);
        }
        let file = File::open(Path::new(CONFIG.file.as_ref().unwrap()))?;
        let reader = io::BufReader::new(file);
        let include_regex = Regex::new(r#"^\s*#include\s+"([^"]+)""#).unwrap();
        for line in reader.lines() {
            match line {
                Ok(content) => {
                    if content.trim().starts_with("#include") {
                        if let Some(captures) = include_regex.captures(content.trim()) {
                            let include_file = captures[1].to_string();
                            if let Ok(included_lines) = read_include_file(&include_file) {
                                lines.extend(included_lines);
                            } else {
                                eprintln!(
                                    "{} could not read include file: {}",
                                    "Error".red().bold(),
                                    include_file.green()
                                );
                                has_err = true;
                            }
                        }
                    } else {
                        lines.push(content);
                        break;
                    }
                }
                Err(e) => {
                    eprintln!(
                        "{} reading line from file: {}",
                        "Error".red().bold(),
                        e.to_string().green()
                    );
                    has_err = true;
                }
            }
        }
        let file = File::open(Path::new(CONFIG.file.as_ref().unwrap()))?;
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            match line {
                Ok(content) => lines.push(content),
                Err(e) => {
                    eprintln!(
                        "{} reading line from file: {}",
                        "error".red().bold(),
                        e.to_string().green()
                    );
                    has_err = true;
                }
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
    lines.retain(|line| !line.starts_with(';'));
    lines.retain(|line| !line.starts_with('#'));
    lines.remove(0); // removes the first line
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
        if CONFIG.debug {
            for token in &tokens {
                println!(
                    "{} {}",
                    "Token:".green().bold(),
                    token.to_string().blue().bold()
                );
            }
        }
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
fn read_include_file(file_name: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut included_lines = Vec::new();
    let file = File::open(file_name)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(content) => included_lines.push(content),
            Err(e) => eprintln!(
                "{} reading line from include file: {}",
                "Error".red().bold(),
                e.to_string().green()
            ),
        }
    }
    Ok(included_lines)
}
#[cfg(test)]
mod tests {
    // no tests
}
