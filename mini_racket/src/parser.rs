pub mod parser {
    use std::result::Result;
    use crate::ast::ast::*;
    use crate::errors::errors::ParseError;
    use crate::tokens::tokens::*;
    use Expr::*;
    use Token::*;   
   
    // Returns the head token of the token list
    fn lookahead(toks: &mut TokenVec) -> Result<Token, ParseError> {
        match toks.lst.as_slice() {
            [] => Err(ParseError::new("No more tokens to lookahead", None, None)),
            [first, ..] => Ok(first.clone())
        } 
    }
    
    // Returns the tail of the token list if the head token matches the input token
    fn match_token(toks: &mut TokenVec, tok: Token) -> Result<TokenVec, ParseError> {
        match toks.lst.as_slice() {
            [] => Err(ParseError::new("No more tokens to match", Some(tok), Some(toks.to_owned()))),
            [first, ..] if *first == tok => Ok(TokenVec { lst: toks.lst[1..].to_vec() }),
            _ => Err(ParseError::new("Could not match", Some(tok), Some(toks.to_owned())))
        }
    }

    // Converts list of tokens into an expression in our defined AST
    pub fn parse(mut toks: &mut TokenVec) -> Result<(TokenVec, Expr), ParseError> {
        match lookahead(&mut toks) {
            Ok(TInt(i)) => parse_int(toks, i),
            Ok(TBool(b)) => parse_bool(toks, b),
            Ok(TID(id)) => parse_id(toks, id),
            Ok(LParen) => parse_lparen(toks),
            Ok(e) => Err(ParseError::new("Unexpected base token", Some(e), Some(toks.to_owned()))),
            Err(err) => Err(err)
        }
    }

    fn parse_non_literal(mut toks: &mut TokenVec) -> Result<(TokenVec, Expr), ParseError> {
        match lookahead(&mut toks) {
            Ok(TIf) => parse_if(toks),
            Ok(TLet) => parse_let(toks),
            // Currently a double paran is equivalent to the variables defined by the let
            Ok(LParen) => {
                let mut tvec = match_token(&mut toks, LParen).unwrap();
                let (mut vec, e) = match parse_non_literal(&mut tvec) {
                    Ok(o) => o,
                    Err(e) => return Err(e)
                };
                let rest2 = match lookahead(&mut vec) {
                    Ok(RParen) => match_token(&mut vec, RParen),
                    Ok(_) => return Err(ParseError::new("Unbalanced parenthesis", Some(RParen), Some(tvec))),
                    Err(err) => return Err(err)
                };
                match rest2 {
                    Ok(vec2) => Ok((vec2, e)),
                    Err(err) => Err(err)
                }
            },
            Ok(TOp1(op)) => parse_op1(toks, op),
            Ok(TOp2(op)) => parse_op2(toks, op),
            Ok(e) => Err(ParseError::new("Unexpected non-literal token", Some(e), Some(toks.to_owned()))),
            Err(err) => Err(err)
        }
    }

    fn parse_if(mut toks: &mut TokenVec) -> Result<(TokenVec, Expr), ParseError> {
        let mut tvec1 = match_token(&mut toks, TIf).unwrap();
        let (mut tvec2, e1) = match parse(&mut tvec1) {
            Ok(o) => o,
            Err(e) => return Err(e)
        };
        let (mut tvec3, e2) = match parse(&mut tvec2) {
            Ok(o) => o,
            Err(e) => return Err(e)
        };
        match parse(&mut tvec3) {
            Ok((vec, e3)) => Ok((vec, If(Box::new(e1), Box::new(e2), Box::new(e3)))),
            Err(err) => Err(err)
        }
    }

    fn parse_let(mut toks: &mut TokenVec) -> Result<(TokenVec, Expr), ParseError> {
        let mut tvec1 = match_token(&mut toks, TLet).unwrap();
        let mut tvec2 = match lookahead(&mut tvec1) {
            Ok(LParen) => match_token(&mut tvec1, LParen).unwrap(),
            Ok(e) => return Err(ParseError::new("Bad let expression", Some(e), Some(toks.to_owned()))),
            Err(err) => return Err(err)
        };
        let mut tvec3 = match lookahead(&mut tvec2) {
            Ok(LParen) => match_token(&mut tvec2, LParen).unwrap(),
            Ok(e) => return Err(ParseError::new("Bad let expression", Some(e), Some(toks.to_owned()))),
            Err(err) => return Err(err)
        };
        let (mut tvec4, e_id) = match lookahead(&mut tvec3) {
            Ok(TID(id)) => (match_token(&mut tvec3, TID(id)).unwrap(), id),
            Ok(e) => return Err(ParseError::new("Bad let expression", Some(e), Some(toks.to_owned()))),
            Err(err) => return Err(err)
        };
        let (mut tvec5, e_id_val) = match parse( &mut tvec4) {
            Ok(vec) => vec,
            Err(e) => return Err(e),
        };
        let mut tvec6 = match lookahead(&mut tvec5) {
            Ok(RParen) => match_token(&mut tvec5, RParen).unwrap(),
            Ok(e) => return Err(ParseError::new("Bad let expression", Some(e), Some(toks.to_owned()))),
            Err(err) => return Err(err)
        };
        let mut tvec7 = match lookahead(&mut tvec6) {
            Ok(RParen) => match_token(&mut tvec6, RParen).unwrap(),
            Ok(e) => return Err(ParseError::new("Bad let expression", Some(e), Some(toks.to_owned()))),
            Err(err) => return Err(err)
        };
        let (tvec8, expr) = match parse( &mut tvec7) {
            Ok(vec) => vec,
            Err(e) => return Err(e),
        };
        Ok((tvec8, Let(e_id, Box::new(e_id_val), Box::new(expr))))
    }

    fn parse_op1(mut toks: &mut TokenVec, op: &'static str) -> Result<(TokenVec, Expr), ParseError> {
        let mut tvec = match_token(&mut toks, TOp1(op.clone())).unwrap();
        match parse(&mut tvec) {
            Ok((vec, e)) => Ok((vec, Prim1(op.clone(), Box::new(e)))),
            Err(err) => Err(err)
        }
    }


    fn parse_op2(mut toks: &mut TokenVec, op: &'static str) -> Result<(TokenVec, Expr), ParseError> {
        let mut tvec1 = match_token(&mut toks, TOp2(op.clone())).unwrap();
        let (mut tvec2, e1) = match parse(&mut tvec1) {
            Ok(o) => o,
            Err(e) => return Err(e)
        };
        match parse(&mut tvec2) {
            Ok((vec, e2)) => Ok((vec, Prim2(op.clone(), Box::new(e1), Box::new(e2)))),
            Err(err) => Err(err)
        }
    }

    fn parse_bool(mut toks: &mut TokenVec, b: bool) -> Result<(TokenVec, Expr), ParseError> {
        let rest = match_token(&mut toks, TBool(b));
        match rest {
            Ok(tvec) => Ok((tvec, Bool(b))),
            Err(err) => Err(err)
        }
    }

    // Converts list of tokens into an expression in our defined AST
    fn parse_int(mut toks: &mut TokenVec, i: i32) -> Result<(TokenVec, Expr), ParseError> {
        let rest = match_token(&mut toks, TInt(i));
        match rest {
            Ok(tvec) => Ok((tvec, Int(i))),
            Err(err) => Err(err)
        }
    }

    fn parse_id(mut toks: &mut TokenVec, id: &'static str) -> Result<(TokenVec, Expr), ParseError> {
        let rest = match_token(&mut toks, TID(id));
        match rest {
            Ok(tvec) => Ok((tvec, Var(id))),
            Err(err) => Err(err)
        }
    }

    fn parse_lparen(mut toks: &mut TokenVec) -> Result<(TokenVec, Expr), ParseError> {
        let mut tvec = match_token(&mut toks, LParen).unwrap();
        let (mut vec, e) = match parse_non_literal(&mut tvec) {
            Ok(o) => o,
            Err(e) => return Err(e)
        };
        let rest2 = match lookahead(&mut vec) {
            Ok(RParen) => match_token(&mut vec, RParen),
            Ok(_) => return Err(ParseError::new("Unbalanced parenthesis", Some(RParen), Some(tvec))),
            Err(err) => return Err(err)
        };
        match rest2 {
            Ok(vec2) => Ok((vec2, e)),
            Err(err) => Err(err)
        }
    }
}
