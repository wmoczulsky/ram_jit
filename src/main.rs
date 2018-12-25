use std::env;
use std::fs;

mod parser;


pub type T = i64;

#[derive(Debug, Clone)]
pub enum Op {
    Integer(T),
    Memory(T),
    Pointer(T),

}

#[derive(Debug, Clone)]
pub struct Label<'a> {
    text: String,
    stmt: Option<&'a Stmt<'a>>,
}

#[derive(Debug, Clone)]
pub enum Instr<'a> {
    Load(Op),
    Store(Op),
    Add(Op),
    Sub(Op),
    Mult(Op),
    Div(Op),
    Read(Op),
    Write(Op),
    Jump(Label<'a>),
    Jgtz(Label<'a>),
    Jzero(Label<'a>),
    Halt(),
}


#[derive(Debug, Clone)]
pub struct Stmt<'a> {
    pos: usize,
    instr: Instr<'a>,
    labels: Vec<Label<'a>>,
}

#[derive(Debug, Clone)]
pub struct Program<'a> {
    stmts: Vec<Stmt<'a>>,
}

struct Config {
    code_file: String,
}




fn read(file: &str) -> String {
    fs::read_to_string(file).expect("Couldn't open file")
}


fn run(cfg: &Config) {
    let content = read(&cfg.code_file);

    eprintln!("executing {}", &cfg.code_file);
    match parser::parse(&content) {
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