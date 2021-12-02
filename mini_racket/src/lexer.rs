pub mod lexer {
    use std::result::Result;
    use regex::Regex;
    use lazy_static::lazy_static;
    use crate::tokens::tokens::*;
    use Token::*;
    use crate::errors::errors::TokError;

    lazy_static! {
        static ref INT_RE: Regex = Regex::new(r"^[0-9]+").unwrap();
        static ref OP1_RE: Regex = Regex::new(r"^add1 |^sub1 ").unwrap();
    }
    
    // Takes raw string and converts to a list of tokens
    pub fn tokenize(text: String) -> Result<TokenVec, TokError> {
        let mut toks = Vec::<Token>::new();
        let mut pos:usize = 0;        

        while pos < text.len(){
            if "\n\t ".contains(&text[pos..(pos + 1)]) {
                pos += 1
            }
            else if &text[pos..(pos + 1)] == "(" {
                toks.push(LParen);
                pos += 1;
            }
            else if &text[pos..(pos + 1)] == ")" {
                toks.push(RParen);
                pos += 1;
            }
            else if let Some(i_match) = INT_RE.find(&text[pos..]) {
                let v = i_match.as_str().parse::<i32>().unwrap();
                toks.push(TInt(v));
                pos += i_match.end()
            }
            else if let Some(op1_match) = OP1_RE.find(&text[pos..]) { // Matches to op1
                let op = op1_match.as_str().clone().trim_end();
                toks.push(TOp1(String::from(op)));
                pos += op1_match.end()
            }
            else {
                return Err(TokError::new("Could not parse", pos))
            }
        }

        Ok(TokenVec { lst: toks })
    }
}
