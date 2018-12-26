use std::env;

mod parser;
mod vm;
use std::io;


pub type T = i64;

#[derive(Debug, Clone)]
pub enum Op {
    Integer(T),
    Memory(T),
    Pointer(T),

}

#[derive(Debug, Clone)]
pub struct Label { // TODO data is redundant
    text: String,
    stmt: Option<usize>,
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
    pos: usize, // position in source code
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




fn get_input() -> Vec<T> {
    let mut a_str = String::new();

    io::stdin().read_line(&mut a_str).expect("read error");

    a_str.split_whitespace()
        .map(|x| x.parse::<T>().expect("input parse error"))
        .collect::<Vec<T>>()
}

fn run(cfg: &Config) -> Result<(), String> {
    eprintln!("executing {}", &cfg.code_file);

    let program = parser::parse(&cfg.code_file)?;

    // let program = match parser::parse(&cfg.code_file) {
    //     Ok(p) => p,
    //     Err(e) => { return Err(format!("Unable to parse: {}", e)) }
    // };

    println!("{:#?}", program);

    vm::execute(program, get_input())?;

    Ok(())
}


fn help() {
    eprintln!("please provide one argument, which is filename");
}


fn main() {
    let args: Vec<String> = env::args().collect();
    
    match args.len() {
        2 => {
            match run(&Config {code_file: args[1].clone()}) {
                Err(e) => eprintln!("{}", e),
                _ => ()
            }
        },
        _ => help()
    }
}