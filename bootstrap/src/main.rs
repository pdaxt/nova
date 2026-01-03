//! Nova Bootstrap Compiler
//!
//! This is a minimal compiler written in Rust. Its only purpose is to compile
//! the first version of the Nova compiler written in Nova itself.
//!
//! Once Stage 1 is complete, this code will be archived and no longer maintained.

mod span;
mod lexer;
mod token;
mod ast;
mod parser;
mod types;
mod ir;
mod codegen;
mod error;

use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Nova Bootstrap Compiler v0.0.1");
        eprintln!();
        eprintln!("Usage: nova <command> [options]");
        eprintln!();
        eprintln!("Commands:");
        eprintln!("  compile <file.nova>    Compile a Nova source file");
        eprintln!("  lex <file.nova>        Show tokens (debug)");
        eprintln!("  parse <file.nova>      Show AST (debug)");
        eprintln!("  help                   Show this message");
        process::exit(1);
    }

    match args[1].as_str() {
        "compile" => cmd_compile(&args[2..]),
        "lex" => cmd_lex(&args[2..]),
        "parse" => cmd_parse(&args[2..]),
        "help" | "--help" | "-h" => {
            eprintln!("Nova Bootstrap Compiler v0.0.1");
            eprintln!("https://github.com/nova-lang/nova");
        }
        other => {
            eprintln!("Unknown command: {}", other);
            eprintln!("Run 'nova help' for usage.");
            process::exit(1);
        }
    }
}

fn cmd_compile(args: &[String]) {
    if args.is_empty() {
        eprintln!("Error: No input file specified");
        eprintln!("Usage: nova compile <file.nova>");
        process::exit(1);
    }

    let path = Path::new(&args[0]);
    let source = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading {}: {}", path.display(), e);
            process::exit(1);
        }
    };

    // Lex
    let tokens = match lexer::lex(&source) {
        Ok(t) => t,
        Err(e) => {
            error::report(&source, path.to_str().unwrap_or("input"), e);
            process::exit(1);
        }
    };

    // Parse
    let ast = match parser::parse(tokens) {
        Ok(a) => a,
        Err(e) => {
            error::report(&source, path.to_str().unwrap_or("input"), e);
            process::exit(1);
        }
    };

    // Type check
    let typed_ast = match types::check(&ast) {
        Ok(t) => t,
        Err(e) => {
            error::report(&source, path.to_str().unwrap_or("input"), e);
            process::exit(1);
        }
    };

    // Generate IR
    let ir = ir::lower(&typed_ast);

    // Generate WASM
    let wasm = codegen::generate(&ir);

    // Write output
    let output_path = path.with_extension("wasm");
    match fs::write(&output_path, wasm) {
        Ok(()) => println!("Wrote {}", output_path.display()),
        Err(e) => {
            eprintln!("Error writing {}: {}", output_path.display(), e);
            process::exit(1);
        }
    }
}

fn cmd_lex(args: &[String]) {
    if args.is_empty() {
        eprintln!("Error: No input file specified");
        process::exit(1);
    }

    let path = Path::new(&args[0]);
    let source = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading {}: {}", path.display(), e);
            process::exit(1);
        }
    };

    match lexer::lex(&source) {
        Ok(tokens) => {
            for token in tokens {
                println!("{:?}", token);
            }
        }
        Err(e) => {
            error::report(&source, path.to_str().unwrap_or("input"), e);
            process::exit(1);
        }
    }
}

fn cmd_parse(args: &[String]) {
    if args.is_empty() {
        eprintln!("Error: No input file specified");
        process::exit(1);
    }

    let path = Path::new(&args[0]);
    let source = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading {}: {}", path.display(), e);
            process::exit(1);
        }
    };

    let tokens = match lexer::lex(&source) {
        Ok(t) => t,
        Err(e) => {
            error::report(&source, path.to_str().unwrap_or("input"), e);
            process::exit(1);
        }
    };

    match parser::parse(tokens) {
        Ok(ast) => {
            println!("{:#?}", ast);
        }
        Err(e) => {
            error::report(&source, path.to_str().unwrap_or("input"), e);
            process::exit(1);
        }
    }
}
