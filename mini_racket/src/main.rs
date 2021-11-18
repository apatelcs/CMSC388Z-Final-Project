mod ast;
mod tokens;
mod lexer;
mod errors;
mod parser;

use lexer::lexer::tokenize;
use parser::parser::parse;

// Wrote simple tests to see if lexer/parser are working
fn main() {
    println!("TOKENIZE TEST");

    let v_good = tokenize(String::from("(((123)  123))"));
    let v_bad = tokenize(String::from("(((123)a123))"));

    match v_good {
        Err(e) => println!("{}", e),
        Ok(ok) => println!("{}", ok)
    }

    match v_bad {
        Err(e) => println!("{}", e),
        Ok(ok) => println!("{}", ok)
    }
    
    println!("---------------------------------------");
    println!("PARSE TEST");
    let good = tokenize(String::from("42"));
    let bad = tokenize(String::from("(42)"));

    println!("Parsing the program: 42");

    match good {
        Err(e) => println!("{}", e),
        Ok(mut ok) => {
            println!("{}", ok);
            let parsed = parse(&mut ok);
            match parsed {
                Err(e) => println!("{}", e),
                Ok(_) => println!("Parsed :)")
            }
        }
    }

    println!("Parsing the program: (42)");

    match bad {
        Err(e) => println!("{}", e),
        Ok(mut ok) => {
            println!("{}", ok);
            let parsed = parse(&mut ok);
            match parsed {
                Err(e) => println!("{}", e),
                Ok(_) => println!("Parsed :)")
            }
        }
    }
}
