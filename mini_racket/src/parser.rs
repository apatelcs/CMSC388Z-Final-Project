use crate::ast::*;

/* FUNCTION HAS ERRORS
// TODO:
// - update parse function parameter type
// - check functionality
*/

// S-Expr -> Expr
pub fn parse(s) -> Result<ast::Expr, &str> {
    match s {
        TInt(i) => Ok(Expr::Int(i)),
        _ => Err("Parse error!")
    }
}
