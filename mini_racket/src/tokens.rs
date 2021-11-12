pub mod tokens {

    #[derive(Debug, Clone, Copy)]
    pub enum Tokens {
        LParen,
        RParen,
        TInt(i32)
    }

    impl std::fmt::Display for Tokens {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }
}
