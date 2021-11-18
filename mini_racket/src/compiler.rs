pub mod compiler {
    use crate::ast::ast::*;
    use crate::a86::a86::*;
    use Expr::*;
    use Instruct::*;

    // Make return type Result<Prog, &'static str>
    pub fn compile(e: Expr) -> Result<String, &'static str> {
       unimplemented!()
    }
    
    // Make return type Result<Seq, &'static str>
    fn compile_e(e: Expr) -> Result<String, &'static str> {
        unimplemented!()
    }
}
