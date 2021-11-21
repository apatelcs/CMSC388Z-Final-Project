pub mod compiler {
    use crate::ast::ast::*;
    use crate::a86::a86::*;
    use Expr::*;
    use Value::*;
    use Instruct::*;
    use Prog;

    // Make return type Result<Asm, &'static str>
    pub fn compile(e: Expr) -> Box<dyn Asm> {
        let entry = String::from("entry");
        let prog = Prog::new (Vec::from([
            Global(entry.clone()).to_asm(),
            Label(entry.clone()).to_asm(),
            compile_e(e),
            Ret.to_asm()
        ]));

        Box::new(prog)
    }
    
    // Make return type Result<Asm, &'static str>
    fn compile_e(e: Expr) -> Box<dyn Asm> {
        let seq= Seq::new(Vec::from([
            match e {
                Int(i) => Mov(Rax, Im(i)).to_asm(),
            }
        ]));

        Box::new(seq)
    }
}
