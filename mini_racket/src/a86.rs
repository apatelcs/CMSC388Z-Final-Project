pub mod a86 {

    // Instruct enum to define x86 asm instructions
    pub enum Instruct {
        Global(String),
        Label(String),
        Ret,
        Mov(String, String)
    }

    // Implement Display for Instruct enum to convert them to strings
    impl std::fmt::Display for Instruct {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            let text = match &self {
                Instruct::Global(lbl) => format!("global _{}", lbl),
                Instruct::Label(lbl) => format!("label _{}", lbl),
                Instruct::Ret => String::from("ret"),
                Instruct::Mov(d, s) => format!("mov {}, {}", d, s)
            };

            write!(f, "{}", text)
        }
    }
}
