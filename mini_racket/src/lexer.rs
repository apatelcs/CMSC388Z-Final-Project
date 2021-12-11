pub mod lexer {
    use std::result::Result;
    use regex::Regex;
    use lazy_static::lazy_static;
    use crate::tokens::tokens::*;
    use Token::*;
    use crate::errors::errors::TokError;

    lazy_static! {
        static ref INT_RE: Regex = Regex::new(r"^[0-9]+").unwrap();
        static ref BOOL_RE: Regex = Regex::new(r"^#t|^#f").unwrap();
        static ref OP1_RE: Regex = Regex::new(r"^add1 |^sub1 ").unwrap();
        static ref OP2_RE: Regex = Regex::new(r"^\+ |^- ").unwrap();
        static ref ID_RE: Regex = Regex::new(r"^[^\s\(\)]+").unwrap();
    }
    
    // Takes raw string and converts to a list of tokens
    pub fn tokenize(text: &'static str) -> Result<TokenVec, TokError> {
        let mut toks = Vec::<Token>::new();
        let mut pos:usize = 0;
        
        if &text[0..12] == "#lang racket" {
            pos += 12;
        }
        else {
            return Err(TokError::new("Must specify '#lang racket'", pos as i32))
        }

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
            else if &text[pos..(pos + 2)] == "if" {
                toks.push(TIf);
                pos += 2;
            }
            else if &text[pos..(pos + 3)] == "let" {
                toks.push(TLet);
                pos += 3;
            }
            else if let Some(i_match) = INT_RE.find(&text[pos..]) {
                let v = i_match.as_str().parse::<i32>().unwrap();
                toks.push(TInt(v));
                pos += i_match.end()
            }
            else if let Some(b_match) = BOOL_RE.find(&text[pos..]) {
                let s = b_match.as_str();
                match s {
                    "#t" => toks.push(TBool(true)),
                    _ => toks.push(TBool(false))
                };
                pos += b_match.end()
            }
            else if let Some(op1_match) = OP1_RE.find(&text[pos..]) { // Matches to op1
                let op = op1_match.as_str().clone().trim_end();
                toks.push(TOp1(op));
                pos += op1_match.end()
            }
            else if let Some(op2_match) = OP2_RE.find(&text[pos..]) {
                let op = op2_match.as_str().clone().trim_end();
                toks.push(TOp2(op));
                pos += op2_match.end()
            }
            else if let Some(id_match) = ID_RE.find(&text[pos..]) {
                let id = id_match.as_str();
                toks.push(TID(id));
                pos += id_match.end();
            }
            else {
                return Err(TokError::new("Could not parse", pos as i32))
            }
        }

        Ok(TokenVec { lst: toks })
    }
}
