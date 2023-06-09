use std::any::Any;
use std::collections::HashMap;

use eyre::Result;

#[derive(PartialEq, Eq, Debug)]
enum Term {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Array(Vec<Term>),
    Struct(HashMap<String, Term>),
}

#[derive(Debug)]
enum Command {
    Let(String, Ty, Box<dyn Any>),
}

fn main() -> Result<()> {
    let cmd = Command::Let("x".into(), Ty::Int, Box::new(32));

    println!("{:?}", cmd);

    Ok(())
}
