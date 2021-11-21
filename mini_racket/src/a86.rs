pub mod a86 {

    static OS_TYPE: &str = std::env::consts::OS;

    fn get_label(label: &String) -> String {
        if OS_TYPE == "macos" {
            format!("_{}",label)
        }
        else {
            label.to_string()
        }
    }

    pub trait Asm: std::fmt::Display { 
        fn to_asm(self) -> Box<dyn Asm>;
    }
    
    // Instruct enum to define x86 asm instructions
    pub enum Instruct {
        Mov(Value, Value),
        Global(String),
        Label(String),
        Ret,
    }

    impl Asm for Instruct {
        fn to_asm(self) -> Box<dyn Asm> {
            Box::new(self)
        }
    }

    pub enum Value {
        Im(i32),
        Rax,
        Rbx,
        R8,
        R9,
        R10,
        R11,
        R12
    }

    impl Value {
        fn value(&self) -> String {
            match *self {
                Value::Im(i) => i.to_string(),
                Value::Rax => String::from("rax"),
                Value::Rbx => String::from("rbx"),
                Value::R8 => String::from("r8"),
                Value::R9 => String::from("r9"),
                Value::R10 => String::from("r10"),
                Value::R11 => String::from("r11"),
                Value::R12 => String::from("r12"),
            }
        }
    }

    // Implement Display for Instruct enum to convert them to strings
    impl std::fmt::Display for Instruct {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            let text = match &self {
                Instruct::Global(lbl) => format!("\tglobal {}", get_label(lbl)),
                Instruct::Label(lbl) => format!("{}:", get_label(lbl)),
                Instruct::Ret => String::from("\tret"),
                Instruct::Mov(d, s) => format!("\tmov {}, {}", d.value(), s.value())
            };

            write!(f, "{}", text)
        }
    }

    pub struct Seq {
        pub lst: Vec<Box<dyn Asm>>
    }

    impl Seq {
        pub fn new(lst: Vec<Box<dyn Asm>>) -> Self {  Self { lst } }
    }

    impl Asm for Seq {
        fn to_asm(self) -> Box<dyn Asm> {
            Box::new(self)
        }
    }

    // Turns list of Instruct into asm instructions.
    impl std::fmt::Display for Seq {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            let mut seq = String::new();
            
            for instruct in &self.lst[0..self.lst.len() - 1] {
                seq.push_str(&instruct.to_string());
                seq.push_str("\n");
            }

            seq.push_str(&self.lst[self.lst.len() - 1].to_string());
            write!(f, "{}", seq)
        }
    }

    pub struct Prog {
        lst: Vec<Box<dyn Asm>>
    }

    impl Prog {
        pub fn new(lst: Vec<Box<dyn Asm>>) -> Self {  Self { lst } }
    }

    impl Asm for Prog {
        fn to_asm(self) -> Box<dyn Asm> {
            Box::new(self)
        }
    }

    // Turns list of Instruct into asm instructions.
    impl std::fmt::Display for Prog {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            let mut prog = String::new();
            let start = format!("\tglobal {}\n\tdefault rel\n\tsection .text\n", get_label(&String::from("entry"))).to_string();
            prog.push_str(&start);
            for instruct in &self.lst[0..self.lst.len() - 1] {
                prog.push_str(&instruct.to_string());
                prog.push_str("\n");
            }

            prog.push_str(&self.lst[self.lst.len() - 1].to_string());
            write!(f, "{}", prog)
        }
    }
}
