pub mod parser {
    use std::result::Result;
    use crate::tokens::tokens::*;
    use Tokens::*;
    use crate::ast::ast::*;
    use Expr::*;
    use crate::lexer::lexer::*;

    // S-Expr -> Expr
    pub fn parse(s: &mut TokenVec) -> Result<Expr, &'static str> {
        match lookahead(s) {
            Some(TInt(i)) => {
                let rest = pop_tok_list(s);
                match rest {
                    None => Ok(Int(i)),
                    _ => Err("Literal reached and still has tokens")
                }
            },
            None => Err("No tokens to parse!"),
            _ => Err("Parse error!")
        }
    }

    pub fn lookahead(s: &mut TokenVec) -> Option<Tokens> {
        if s.0.len() != 0 {
            return Some(s.0[0]);
        }
        return None;
    }

    pub fn pop_tok_list(s: &mut TokenVec) -> Option<TokenVec> {
        if s.0[1..].len() == 0 {
            return None;
        }

        Some(TokenVec(s.0[1..].to_vec()))
    }
}
