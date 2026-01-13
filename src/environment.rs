use std::collections::HashMap;
use crate::values::{ValueType, Function};
use crate::bytecode::OpCode;

#[derive(Debug)]
pub struct Env {
    pub vars: HashMap<String, Variable>,
    pub globals: HashMap<String, ValueType>,
    pub aliases: HashMap<String, String>,
    pub functions: HashMap<String, Function>,
    pub scope_depth: i32,
    pub debug: bool,
}

impl Env {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
            globals: HashMap::new(),
            functions: HashMap::new(),
            aliases: HashMap::new(),
            scope_depth: 0,
            debug: false,
        }
    }
}

#[derive(Debug)]
pub struct Variable {
    pub value: ValueType,
    pub scope: i32,
}
