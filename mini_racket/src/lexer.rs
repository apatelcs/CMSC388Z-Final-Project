pub mod lexer {
    use std::result::Result;
    use regex::Regex;
    use lazy_static::lazy_static;
    use crate::tokens::tokens::*;
    use Tokens::*;
    use crate::errors::errors::TokError;

    pub struct TokenVec(Vec<Tokens>);

    
    impl std::fmt::Display for TokenVec {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            let mut comma_separated = String::new();

            for num in &self.0[0..self.0.len() - 1] {
                comma_separated.push_str(&num.to_string());
                comma_separated.push_str(", ");
            }

            comma_separated.push_str(&self.0[self.0.len() - 1].to_string());
            write!(f, "{}", comma_separated)
        }
    }

    lazy_static! {
        static ref INT_RE: Regex = Regex::new(r"^[0-9]+").unwrap();
    }
    
    pub fn tokenize(text: String) -> Result<TokenVec, TokError> {
        let mut toks = Vec::<Tokens>::new();
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
            else {
                return Err(TokError::new("Could not parse", pos))
            }
        }

        Ok(TokenVec(toks))
    }
}
