pub mod parser {
    use std::result::Result;
    use crate::ast::ast::*;
    use crate::tokens::tokens::*;
    use Expr::*;
    use Token::*;
   
    // Returns the head token of the token list
    fn lookahead(toks: &mut TokenVec) -> Result<Token, &'static str> {
        match toks.lst.as_slice() {
            [] => Err("No more tokens!"),
            [first, ..] => Ok(*first)
        } 
    }
    
    // Returns the tail of the token list if the head token matches the input token
    fn match_token(toks: &mut TokenVec, tok: Token) -> Result<TokenVec, &'static str> {
        match toks.lst.as_slice() {
            [] => Err("No more tokens!"),
            [first, ..] if *first == tok => Ok(TokenVec { lst: toks.lst[1..].to_vec() }),
            _ => Err("Tokens did not match!")
        }
    }

    // Converts list of tokens into an expression in our defined AST
    pub fn parse(mut toks: &mut TokenVec) -> Result<(TokenVec, Expr), &'static str> {
        match lookahead(&mut toks) {
            Ok(TInt(i)) => {
                let rest = match_token(&mut toks, TInt(i));
                match rest {
                    Ok(tvec) => Ok((tvec, Int(i))),
                    Err(err) => Err(err)
                }
            } 
            Ok(_) => Err("Unexpected token!"),
            Err(err) => Err(err)
        }
    }

}
