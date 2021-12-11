pub mod compiler {
    use crate::ast::ast::*;
    use crate::a86::a86::*;
    use crate::types::types::*;
    use Expr::*;
    use Value::*;
    use Instruct::*;
    use Prog;
    use std::sync::atomic::{AtomicUsize, Ordering};

    // Make return type Result<Asm, &'static str>
    pub fn compile(e: Expr) -> Box<dyn Asm> {
        let entry = String::from("entry");
        let prog = Prog::new (Vec::from([
            Global(entry.clone()).to_asm(),
            Label(entry.clone()).to_asm(),
            compile_e(e, Vec::new()),
            Ret.to_asm()
        ]));

        Box::new(prog)
    }
    
    // Make return type Result<Asm, &'static str>
    fn compile_e(e: Expr, cenv: Vec<&'static str>) -> Box<dyn Asm> {
        let seq= Seq::new(Vec::from([
            match e {
                Int(i) => Mov(Rax, Im(int_to_bits(i))).to_asm(),
                Bool(b) => Mov(Rax, Im(bool_to_bits(b))).to_asm(),
                If(e1, e2, e3) => compile_if(*e1, *e2, *e3, cenv),
                Prim1(op, e) => compile_prim1(op, *e, cenv),
                Prim2(op, e1, e2) => compile_prim2(op, *e1, *e2, cenv),
                // TODO: EEE
                Var(x) => compile_variable(x, cenv),
                Let(id, e_id, e) => compile_let(id, *e_id, *e, cenv)
            }
        ]));

        Box::new(seq)
    }

    fn compile_if(e1: Expr, e2: Expr, e3: Expr, cenv: Vec<&'static str>) -> Box<dyn Asm> {
        let l1 = gensym(None);
        let l2 = gensym(None);
        let seq = Seq::new(Vec::from([
            compile_e(e1, cenv.clone()),
            Cmp(Rax, Im(VAL_FALSE)).to_asm(),
            Je(l1.clone()).to_asm(),
            compile_e(e2, cenv.clone()),
            Jmp(l2.clone()).to_asm(),
            Label(l1).to_asm(),
            compile_e(e3, cenv.clone()),
            Label(l2).to_asm()
        ]));

        Box::new(seq)
    }

    fn compile_prim1(op: &'static str, e: Expr, cenv: Vec<&'static str>) -> Box<dyn Asm> {
        let seq = Seq::new(Vec::from([
            compile_e(e, cenv),
            match op {
                "add1" => Add(Rax, Im(int_to_bits(1))).to_asm(),
                _ => Sub(Rax, Im(int_to_bits(1))).to_asm()
            }
        ]));

        Box::new(seq)
    }

    fn compile_prim2(op: &'static str, e1: Expr, e2: Expr, cenv: Vec<&'static str>) -> Box<dyn Asm> {
        let mut pushed_cenv = cenv.clone();
        pushed_cenv.insert(0, "#f");
        let seq = Seq::new(Vec::from([
            compile_e(e1, cenv),
            Push(Rax).to_asm(),
            compile_e(e2, pushed_cenv),
            Pop(R8).to_asm(),
            match op {
                "+" => Add(Rax, R8).to_asm(),
                _ => {
                    Box::new(Seq::new(Vec::from([
                        Sub(R8, Rax).to_asm(),
                        Mov(Rax, R8).to_asm()
                    ])))
                }
            }
        ]));

        Box::new(seq)
    }

    fn compile_variable(x: &'static str, cenv: Vec<&'static str>) -> Box<dyn Asm>{
        let pos = lookup(x, cenv);

        let seq = Seq::new(Vec::from([
            Mov(Rax, Offset(Box::new(Rsp), pos)).to_asm()
        ]));

        seq.to_asm()
    }

    fn compile_let(id: &'static str, e_id: Expr, e: Expr, cenv: Vec<&'static str>) -> Box<dyn Asm> {
        let mut cenv_new = cenv.clone();
        cenv_new.insert(0, id);

        let seq = Seq::new(Vec::from([
            compile_e(e_id, cenv.clone()),
            Push(Rax).to_asm(),
            compile_e(e, cenv_new),
            Add(Rsp, Im(8)).to_asm()
        ]));

        seq.to_asm()
    }

    fn lookup(x: &'static str, cenv: Vec<&'static str>) -> i32 {
        let mut pos = 0;

        for id in cenv {
            if x == id {
                return pos;
            }
            pos += 8;
        }

        panic!("Failed to find variable!");
    }

    fn gensym(val: Option<String>) -> String {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        let num = COUNTER.fetch_add(1, Ordering::Relaxed);
        let str = match val {
            Some(s) => s,
            None => String::from("g")
        };
        format!("{}{}", str, num)
    }

}
