pub mod ast;
pub mod interpreter;

use std::{fs, path::PathBuf};

use lalrpop_util::lalrpop_mod;
use clap::{Parser};

lalrpop_mod!(
    #[allow(clippy::ptr_arg)]
    #[rustfmt::skip]
    tinylang
);

#[derive(Parser, Debug)]
struct Args{
    #[arg(short, long)]
    file: PathBuf
}

use interpreter::Interpreter;

fn main() {
    let args = Args::parse();

    let is_tl = args
    .file
    .extension()
    .and_then(|e| e.to_str())
    .is_some_and(|e| e.eq_ignore_ascii_case("tl"));

    if !is_tl {
        eprintln!("Expected a .tl file, got: {}", args.file.display());
        std::process::exit(2);
    }

    let source = fs::read_to_string(&args.file).unwrap_or_else(|e| {
       eprintln!("Failed to read {}: {e}", args.file.display());
       std::process::exit(1); 
    } );

    let parser = tinylang::ProgramParser::new();
    let program = parser.parse(&source).unwrap_or_else(|e| {
        eprintln!("Parse error: {e}");
        std::process::exit(1);
    });

    let mut interp = Interpreter::new();
    if let Err(e) = interp.run(program) {
        eprintln!("Runtime error: {e}");
        std::process::exit(1);
    }
}