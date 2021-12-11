// Defines the Abstract Syntax Tree for MiniRacket



pub mod ast {
    // type Expr = Int (i32)
    #[derive(Debug)]
    pub enum Expr {
        Int(i32),
        Bool(bool),
        If(Box<Expr>, Box<Expr>, Box<Expr>),
        Prim1(&'static str, Box<Expr>),
        Prim2(&'static str, Box<Expr>, Box<Expr>),
        Let(&'static str, Box<Expr>, Box<Expr>),
        Var(&'static str),
    }

    // Helpful display implementation for expressions
    impl std::fmt::Display for Expr {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }
}
