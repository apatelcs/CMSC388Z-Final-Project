// Defines the Abstract Syntax Tree for MiniRacket

pub mod ast {
    // type Expr = Int (i32)
    #[derive(Debug)]
    pub enum Expr {
        Int(i32),
        Prim1(String, Box<Expr>)
    }

    // Helpful display implementation for tokens
    impl std::fmt::Display for Expr {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }
}
