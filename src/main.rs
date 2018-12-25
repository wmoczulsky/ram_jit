use std::env;

mod parser;


pub type T = i64;

#[derive(Debug, Clone)]
pub enum Op {
    Integer(T),
    Memory(T),
    Pointer(T),

}

#[derive(Debug, Clone)]
pub struct Label {
    text: String,
}

#[derive(Debug, Clone)]
pub enum Instr {
    Load(Op),
    Store(Op),
    Add(Op),
    Sub(Op),
    Mult(Op),
    Div(Op),
    Read(Op),
    Write(Op),
    Jump(Label),
    Jgtz(Label),
    Jzero(Label),
    Halt(),
}


#[derive(Debug, Clone)]
pub struct Stmt {
    pos: usize,
    instr: Instr,
    labels: Vec<Label>,
}

#[derive(Debug, Clone)]
pub struct Program {
    stmts: Vec<Stmt>,
}

struct Config {
    code_file: String,
}






fn run(cfg: &Config) {
    eprintln!("executing {}", &cfg.code_file);
    match parser::parse(&cfg.code_file) {
        Ok(program) => println!("{:#?}", program),
        Err(e) => eprintln!("Unable to parse: {}", e),
    }
}


fn help() {
    eprintln!("please provide one argument, which is filename");
}


fn main() {
    let args: Vec<String> = env::args().collect();
    
    match args.len() {
        2 => {
            run(&Config {code_file: args[1].clone()})
        },
        _ => help()
    }
}