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
            [first, ..] => Ok(first.clone())
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
            },
            Ok(TBool(b)) => {
                let rest = match_token(&mut toks, TBool(b));
                match rest {
                    Ok(tvec) => Ok((tvec, Bool(b))),
                    Err(err) => Err(err)
                }
            },
            Ok(LParen) => {
                let rest = match_token(&mut toks, LParen);
                match rest {
                    Ok(mut tvec) => {
                        match parse_non_literal(&mut tvec) {
                            Ok((mut vec, e)) => {
                                match lookahead(&mut vec) {
                                    Ok(RParen) => {
                                        let rest2 = match_token(&mut vec, RParen);
                                        match rest2 {
                                            Ok(vec2) => Ok((vec2, e)),
                                            Err(err) => Err(err)
                                        }
                                    },
                                    Ok(_) => Err("Unbalanced parenthesis!"),
                                    Err(err) => Err(err)
                                }
                            },
                            Err(err) => Err(err)
                        }
                    },
                    Err(err) => Err(err)
                }
            },
            Ok(_) => Err("Unexpected token!"),
            Err(err) => Err(err)
        }
    }

    fn parse_non_literal(mut toks: &mut TokenVec) -> Result<(TokenVec, Expr), &'static str> {
        match lookahead(&mut toks) {
            Ok(TIf) => {
                let rest = match_token(&mut toks, TIf);
                match rest {
                    Ok(mut tvec1) => {
                        match parse(&mut tvec1) {
                            Ok((mut tvec2, e1)) => {
                                match parse(&mut tvec2) {
                                    Ok((mut tvec3, e2)) => {
                                        match parse(&mut tvec3) {
                                            Ok((vec, e3)) => Ok((vec, If(Box::new(e1), Box::new(e2), Box::new(e3)))),
                                            Err(err) => Err(err)
                                        }
                                    },
                                    Err(err) => Err(err)
                                }
                            },
                            Err(err) => Err(err)
                        }
                    },
                    Err(err) => Err(err)
                }
            },
            Ok(TOp1(op)) => {
                let rest = match_token(&mut toks, TOp1(op.clone()));
                match rest {
                    Ok(mut tvec) => {
                        match parse(&mut tvec) {
                            Ok((vec, e)) => Ok((vec, Prim1(op.clone(), Box::new(e)))),
                            Err(err) => Err(err)
                        }
                    },
                    Err(err) => Err(err)
                }
            },
            // Ok(TOp2(op)) => {
            //     let rest = match_token(&mut toks, TOp2(op.clone()));
            //     match rest {
            //         Ok(mut tvec1) => {
            //             match parse(&mut tvec1) {
            //                 Ok((mut tvec2, e1)) => {
            //                     match parse(&mut tvec2) {
            //                         Ok((vec, e2)) => Ok((vec, Prim2(op.clone(), Box::new(e1), Box::new(e2)))),
            //                         Err(err) => Err(err)
            //                     }
            //                 },
            //                 Err(err) => Err(err)
            //             }
            //         },
            //         Err(err) => Err(err)
            //     }
            // },
            Ok(_) => Err("Unexpected token!"),
            Err(err) => Err(err)
        }
    }
}
