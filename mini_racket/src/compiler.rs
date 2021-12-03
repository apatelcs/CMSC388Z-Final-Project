pub mod compiler {
    use uuid::Uuid;
    use crate::ast::ast::*;
    use crate::a86::a86::*;
    use crate::types::types::*;
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
                Int(i) => Mov(Rax, Im(int_to_bits(i))).to_asm(),
                Bool(b) => Mov(Rax, Im(bool_to_bits(b))).to_asm(),
                If(e1, e2, e3) => compile_if(*e1, *e2, *e3),
                Prim1(op, e) => compile_prim1(op, *e),
                // Prim2(op, e1, e2) => 
            }
        ]));

        Box::new(seq)
    }

    fn compile_if(e1: Expr, e2: Expr, e3: Expr) -> Box<dyn Asm> {
        let l1 = Uuid::new_v4().to_simple().to_string();
        let l2 = Uuid::new_v4().to_simple().to_string();
        let seq = Seq::new(Vec::from([
            compile_e(e1),
            Cmp(Rax, Im(VAL_FALSE)).to_asm(),
            Je(l1.clone()).to_asm(),
            compile_e(e2),
            Jmp(l2.clone()).to_asm(),
            Label(l1).to_asm(),
            compile_e(e3),
            Label(l2).to_asm()
        ]));

        Box::new(seq)
    }

    fn compile_prim1(op: String, e: Expr) -> Box<dyn Asm> {
        let seq = Seq::new(Vec::from([
            compile_e(e),
            match op.as_str() {
                "add1" => Add(Rax, Im(int_to_bits(1))).to_asm(),
                _ => Sub(Rax, Im(int_to_bits(1))).to_asm()
            }
        ]));

        Box::new(seq)
    }

}
