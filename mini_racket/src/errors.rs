// Defines the errors to be thrown throughout the project

pub mod errors {
    use crate::tokens::tokens::Token;
    use crate::tokens::tokens::TokenVec;
    use std::fmt;


    pub struct TokError {
        details: String,
        err_ind: i32
    }

    impl TokError {
        pub fn new(msg: &str, ind: i32) -> TokError {
            TokError { details: msg.to_string(), err_ind: ind}
        }

        pub fn to_string(&self) -> String {
            format!("Tokenizer error: {} at index {}", &self.details, &self.err_ind)
        }
    }

    impl fmt::Display for TokError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", &self.to_string())
        }
    }


    pub struct ParseError {
        details: String,
        err_tok: Option<Token>,
        err_vec: Option<TokenVec>
    }

    impl ParseError {
        pub fn new(msg: &str, err_tok: Option<Token>, err_vec: Option<TokenVec>) -> ParseError {
            ParseError { details: msg.to_string(), err_tok: err_tok, err_vec: err_vec}
        }

        pub fn to_string(&self) -> String {
            let err_tok = match &self.err_tok {
                None => format!("None"),
                Some(e) => format!("{}", e)
            };
            let err_vec = match &self.err_vec {
                None => format!("None"),
                Some(e) => format!("{}", e)
            };
            format!("Parser error: {} {} in {}", &self.details, err_tok, err_vec)
        }
    }

    impl fmt::Display for ParseError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", &self.to_string())
        }
    }

    impl fmt::Debug for ParseError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", &self.to_string())
        }
    }

    pub struct CompileError {
        details: String,
    }

    impl CompileError {
        pub fn new(msg: &str) -> CompileError {
            CompileError { details: msg.to_string()}
        }

        pub fn to_string(&self) -> String {
            format!("Compile error: {}", &self.details)
        }
    }

    impl fmt::Display for CompileError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", &self.to_string())
        }
    }

    impl fmt::Debug for CompileError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", &self.to_string())
        }
    }
}
