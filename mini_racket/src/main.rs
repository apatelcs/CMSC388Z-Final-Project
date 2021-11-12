mod ast;
mod tokens;
mod lexer;
mod errors;

use lexer::lexer::tokenize;


fn main() {

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
}
