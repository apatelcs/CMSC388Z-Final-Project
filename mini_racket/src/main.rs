mod ast;
mod tokens;
mod lexer;
mod errors;
mod parser;
mod a86;
mod compiler;
mod types;

use std::env;
use std::fs;
use std::path::Path;
use lexer::lexer::tokenize;
use parser::parser::parse;
use compiler::compiler::compile;

// Wrote simple tests to see if lexer/parser are working
fn main() {
    // let code = "(add1 (add1 (sub1 42)))";
    // let tokens = tokenize(String::from(code));
    // match tokens {
    //     Ok(mut toks) => {
    //         println!("Tokens are: {}", toks);
    //         let parsed = parse(&mut toks);
    //         match parsed {
    //             Ok((v, e)) => println!("Expression: {}", e),
    //             Err(err) => println!("{}", err)
    //         }
    //     },
    //     Err(err) => println!("{}", err)
    // }
    let mut args = env::args();
    let file = &args.nth(1).unwrap();
    let p = Path::new(file);
    let file_stem = p.file_stem().unwrap().to_str().unwrap();
    let path = format!("./{}.rkt", file_stem);
    let source = fs::read_to_string(path).expect("Unable to read file");
    let tokens = tokenize(source);
    let parse = match tokens {
        Ok(mut toks) => parse(&mut toks),
        Err(err) => panic!("{}", err)
    };
    let asm = match parse {
        Ok((_,expr)) => compile(expr),
        Err(err) => panic!("{}", err)
    };
    let write_path = format!("./{}.s", file_stem);
    fs::write(write_path, asm.to_string()).expect("Unable to write file");
}
