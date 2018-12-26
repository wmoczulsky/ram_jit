use std::fs;
use std::collections::HashMap;
use super::*;

fn try_parse_op(arg_str: &str) -> Result<Op, String> {
    match arg_str.chars().next() {
        Some('=') if arg_str.len() > 1 => {
            match arg_str.get(1..).unwrap().parse::<T>() {
                Ok(num) => Ok(Op::Integer(num)),
                Err(_) => Err("Invalid operand".into()),
            }
        },
        Some('^') if arg_str.len() > 1 => {
            match arg_str.get(1..).unwrap().parse::<T>() {
                Ok(num) => Ok(Op::Mem(OpMem::Pointer(num))),
                Err(_) => Err("Invalid operand".into()),
            }
        },
        Some(_) => match arg_str.parse::<T>() {
                Ok(num) => Ok(Op::Mem(OpMem::Memory(num))),
                Err(_) => Err("Invalid operand".into()),
        },
        _ => Err("Invalid operand".into())
    }
}

fn try_parse_label(arg_str: &str) -> Result<Label, String> {
    match arg_str.len() {
        0 => Err(String::from("Label not provided")),
        _ => Ok(Label{text: String::from(arg_str), stmt: None}),
    }
}

fn parse_instr(instr_str: &str, arg_str: &str) -> Result<Instr, String> {
    match instr_str.to_lowercase().as_str() {
        "load" => Ok(Instr::Load(try_parse_op(arg_str)?)),
        "store" => Ok(Instr::Store(try_parse_op(arg_str)?.to_op_mem()?)),
        "add" => Ok(Instr::Add(try_parse_op(arg_str)?)),
        "sub" => Ok(Instr::Sub(try_parse_op(arg_str)?)),
        "mult" => Ok(Instr::Mult(try_parse_op(arg_str)?)),
        "div" => Ok(Instr::Div(try_parse_op(arg_str)?)),
        "read" => Ok(Instr::Read(try_parse_op(arg_str)?.to_op_mem()?)),
        "write" => Ok(Instr::Write(try_parse_op(arg_str)?)),
        "jump" => Ok(Instr::Jump(try_parse_label(arg_str)?)),
        "jgtz" => Ok(Instr::Jgtz(try_parse_label(arg_str)?)),
        "jzero" => Ok(Instr::Jzero(try_parse_label(arg_str)?)),
        "halt" => Ok(Instr::Halt()),
        _ => Err(format!("Incorrect instruction: {}", instr_str))
    }
}

fn parse_line(line_raw: &str) -> Result<(Option<Label>, Option<Instr>), String> {
    let mut label_opt: Option<Label> = None;

    let line = line_raw.split("#").next().unwrap(); // drop comment

    let mut tokens = line.split_whitespace().peekable();

    match tokens.peek() {
        Some(text) if text.ends_with(":") => { 
            let mut label = (*text).to_string();
            label.truncate(label.len() - 1);
            label_opt = Some(Label {text: label, stmt: None});
            tokens.next();
        }
        _ => ()
    }
    
    let tokens: Vec<&str> = tokens.collect();
    match tokens.len() {
        0 => Ok((label_opt, None)),
        1 => Ok((label_opt, Some(parse_instr(tokens[0], "")?))),
        2 => Ok((label_opt, Some(parse_instr(tokens[0], tokens[1])?))),
        _ => Err(format!("Wrong line format: \"{}\"", line)),
    }

}

impl Program {
    fn try_from(s: String) -> Result<Self, String> {
        let mut labels = Vec::new();
        let mut stmts = Vec::new();
        let lines = s.lines();
        for (line, line_number) in lines.zip(1..) {
            let (label_opt, instr_opt) = match parse_line(line) {
                Ok(result) => result,
                Err(e) => {return Err(format!("{} in line {}", e, line_number))},
            };
            match label_opt {
                Some(label) => labels.push(label),
                None => ()
            };
            match instr_opt {
                Some(instr) => {
                    stmts.push(Stmt{instr, labels:(labels.clone()), pos: line_number});
                    labels.clear();
                }
                None => ()
            }
        }

        // add Halt at the end, so each program ends with halt and all labels point to something
        stmts.push(Stmt{instr: Instr::Halt(), labels:(labels.clone()), pos: 0});  
        
        Ok(Program{stmts: stmts})
    }

    fn link_labels_to_statements(mut self) -> Result<Self, String> {
        let mut label_to_statement = HashMap::<String, usize>::new();

        for (stmt, i) in self.stmts.iter().zip(0..) {
            for label in stmt.labels.iter() {
                label_to_statement.insert(label.text.clone(), i);
            }
        }

        for stmt in self.stmts.iter_mut() {
            match &mut stmt.instr {
                Instr::Jump(label) | Instr::Jgtz(label) | Instr::Jzero(label) => {
                    match label_to_statement.get(label.text.as_str()) {
                        Some(i) => { label.stmt = Some(*i) }
                        None => return Err(format!("Undefined label \"{:?}\"", label.text))
                    }
                }
                _ => {}
            }
        }

        Ok(self)
    }
 }
 


fn read(file: &str) -> String {
    fs::read_to_string(file).expect("Couldn't open file")
}


pub fn parse(file: &str) -> Result<Program, String> {
    let content = read(file);
    let p = Program::try_from(content)?;
    Ok(p.link_labels_to_statements()?)
}
