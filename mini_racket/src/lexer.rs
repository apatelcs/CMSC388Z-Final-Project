mod ast;
use ast::Expr::*;

pub mod lexer {
    fn tokenize(text: String) {
        let mut toks = Vec::new(); 
        let mut pos = 0;

        let mut chars = "gravy train".chars().fuse();

        while let Some(c) = chars.next() {
            if c == 'x' {
                chars.next(); // Skip the next one
            }
        }
    }
}
