pub mod tokens {

    // Token representations of our source language
    #[derive(Debug, Clone, PartialEq)]
    pub enum Token {
        LParen,
        RParen,
        TInt(i32),
        TBool(bool),
        TIf,
        TOp1(String),
        // TOp2(String)
    }

    // Helpful display implementation for tokens
    impl std::fmt::Display for Token {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    // Token Vector structure to represent list of tokens
    #[derive(Debug, Clone)]
    pub struct TokenVec {
        pub lst: Vec<Token>
    }

    // Helpful display implementation for token vectors
    impl std::fmt::Display for TokenVec {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            let mut comma_separated = String::new();
            
            for num in &self.lst[0..self.lst.len() - 1] {
                comma_separated.push_str(&num.to_string());
                comma_separated.push_str(", ");
            }

            comma_separated.push_str(&self.lst[self.lst.len() - 1].to_string());
            write!(f, "{}", comma_separated)
        }
    }
}
