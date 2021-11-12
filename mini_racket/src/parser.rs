use crate::ast::*;

/* FUNCTION HAS ERRORS
// TODO:
// - update parse function parameter type
// - check return type
// - check functionality
*/

// S-Expr -> Expr
pub fn parse(s) -> Result<_, &str> {
    match s {
        TInt(i) => Ok(Int(i)),
        _ => Err("Parse error!")
    }
}
