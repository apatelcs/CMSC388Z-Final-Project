pub mod errors {

    pub struct TokError {
        details: String,
        err_ind: usize
    }

    impl TokError {
        pub fn new(msg: &str, ind: usize) -> TokError {
            TokError { details: msg.to_string(), err_ind: ind}
        }

        pub fn to_string(&self) -> String {
            format!("Tokenizer error: {} at index {}", &self.details, &self.err_ind)
        }
    }

    impl std::fmt::Display for TokError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", &self.to_string())
        }
    }
}