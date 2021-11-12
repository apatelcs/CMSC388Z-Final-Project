mod ast;
mod tokens;

use ast::Expr::*;
use tokens::Tokens::*;

/* FUNCTION HAS ERRORS
// TODO:
// - update parse function parameter type
// - check functionality
*/
pub mod parser {
    // S-Expr -> Expr
    pub fn parse(s) -> Result<Expr, &str> {
        match s {
            TInt(i) => Ok(Int(i)),
            _ => Err("Parse error!")
        }
    }
}

