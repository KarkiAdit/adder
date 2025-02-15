use std::env;
use std::fs::File;
use std::io::prelude::*;
use sexp::parse;

mod compiler;
mod interpreter;
mod datatypes;

use compiler::compile_expr;
use interpreter::{eval, parse_expr};

fn main() -> std::io::Result<()> {
    // Check if correct arguments are provided
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <input-file> <output-file>", args[0]);
        std::process::exit(1);
    }

    let in_name = &args[1];
    let out_name = &args[2];

    // Open and read the input file safely
    let mut in_file = File::open(in_name)?;
    let mut in_contents = String::new();
    in_file.read_to_string(&mut in_contents)?;

    // Handle potential parsing errors
    let expr = match parse(&in_contents) {
        Ok(parsed_sexp) => parse_expr(&parsed_sexp),
        Err(e) => {
            eprintln!("Error parsing input S-expression: {}", e);
            std::process::exit(1);
        }
    };

    // Evaluate the expression before compiling (for debugging/testing)
    let interpreted_result = eval(&expr);
    println!("Interpreted Result: {}", interpreted_result);

    let result = compile_expr(&expr); // Use compile_expr from compiler.rs

    let asm_program = format!("
section .text
global our_code_starts_here
our_code_starts_here:
  {}
  ret
", result);

    // Safely create and write to the output file
    let mut out_file = File::create(out_name)?;
    out_file.write_all(asm_program.as_bytes())?;

    Ok(())
}
