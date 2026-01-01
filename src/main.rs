// Action! Compiler for Z80
// A cross-compiler that generates Z80 machine code from Action! source

mod lexer;
mod token;
mod ast;
mod parser;
mod codegen;
mod runtime;
mod error;

use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "kz80_action")]
#[command(about = "Action! language compiler for Z80", long_about = None)]
struct Args {
    /// Input Action! source file
    #[arg(short, long)]
    input: PathBuf,

    /// Output binary file
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Origin address for code (default: 0x4200)
    #[arg(long, default_value = "0x4200")]
    org: String,

    /// Generate listing file
    #[arg(short, long)]
    listing: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    // Parse origin address
    let org = if args.org.starts_with("0x") || args.org.starts_with("0X") {
        u16::from_str_radix(&args.org[2..], 16).unwrap_or(0x4200)
    } else {
        args.org.parse().unwrap_or(0x4200)
    };

    // Read source file
    let source = match fs::read_to_string(&args.input) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading file {:?}: {}", args.input, e);
            std::process::exit(1);
        }
    };

    if args.verbose {
        println!("Compiling {:?}...", args.input);
        println!("Origin address: 0x{:04X}", org);
    }

    // Tokenize
    let mut lexer = lexer::Lexer::new(&source);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Lexer error: {}", e);
            std::process::exit(1);
        }
    };

    if args.verbose {
        println!("Tokens: {}", tokens.len());
        for tok in &tokens {
            println!("  {:?}", tok);
        }
    }

    // Parse
    let mut parser = parser::Parser::new(tokens);
    let program = match parser.parse() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Parser error: {}", e);
            std::process::exit(1);
        }
    };

    if args.verbose {
        println!("AST: {:?}", program);
    }

    // Generate runtime library first, leaving space for initial JP instruction
    let runtime_start = org + 3;  // JP instruction takes 3 bytes
    let (runtime_code, runtime_symbols) = runtime::generate_runtime(runtime_start);
    let code_start = runtime_symbols.end_address;

    if args.verbose {
        println!("Runtime: {} bytes (0x{:04X}-0x{:04X})",
                 runtime_code.len(), runtime_start, code_start);
        println!("  PrintB: 0x{:04X}", runtime_symbols.print_b);
        println!("  PrintC: 0x{:04X}", runtime_symbols.print_c);
        println!("  PrintE: 0x{:04X}", runtime_symbols.print_e);
        println!("  Print:  0x{:04X}", runtime_symbols.print);
    }

    // Generate code
    let mut codegen = codegen::CodeGenerator::new(code_start);
    codegen.set_runtime_symbols(&runtime_symbols);
    let program_code = match codegen.generate(&program) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Code generation error: {}", e);
            std::process::exit(1);
        }
    };

    // Build final binary:
    // 1. JP to code_start (entry point with CALL main, HALT)
    // 2. Runtime library
    // 3. Program code
    let mut binary = Vec::new();
    binary.push(0xC3);  // JP
    binary.push((code_start & 0xFF) as u8);
    binary.push((code_start >> 8) as u8);
    binary.extend(runtime_code);
    binary.extend(program_code);

    // Determine output filename
    let output_path = args.output.unwrap_or_else(|| {
        let mut p = args.input.clone();
        p.set_extension("bin");
        p
    });

    // Write output
    if let Err(e) = fs::write(&output_path, &binary) {
        eprintln!("Error writing output file {:?}: {}", output_path, e);
        std::process::exit(1);
    }

    println!("Compiled {} bytes to {:?}", binary.len(), output_path);

    // Generate listing if requested
    if args.listing {
        let listing_path = {
            let mut p = output_path.clone();
            p.set_extension("lst");
            p
        };
        let listing = codegen.generate_listing();
        if let Err(e) = fs::write(&listing_path, listing) {
            eprintln!("Error writing listing file {:?}: {}", listing_path, e);
        } else {
            println!("Listing written to {:?}", listing_path);
        }
    }
}
