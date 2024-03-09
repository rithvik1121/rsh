use crate::bytecode::{OpCode, ValueType, Chunk};
use crate::parser::Parser;
use crate::lexer::Lexer;
use crate::tokens::{Token, TokenType};
use std::collections::BinaryHeap;


pub struct VM {
    unordered_values: Vec<ValueType>,
    ip: usize,
    values: Vec<ValueType>,
    ops: Vec<OpCode>,
}

impl VM {
    pub fn new() -> Self {
        Self {
            unordered_values: Vec::new(),
            values: Vec::new(),
            ip: 0,
            ops: Vec::new(),
        }
    }
    pub fn init() {
                    
    }

    pub fn free(&mut self) {
        self.unordered_values = Vec::new();
        self.values = Vec::new();
        self.ip = 0;
        self.ops = Vec::new();
    }

    pub fn interpret(&mut self, source: &str) -> Option<IResult> {
        self.ip = 0;
        self.unordered_values = Vec::new();

        let mut token_stream: Vec<Token> = Vec::new();
        let mut lexer = Lexer::new(source);
        loop {
            let lexed = lexer.lex();
            println!("lexed: {:?}", lexed);
            match lexed.token_type {
                TokenType::EOF => {
                    token_stream.push(lexed);
                    break;
                },
                _ => (),
            };
            token_stream.push(lexed);
            lexer.advance();
        }

        println!("\n\nlexing done, beginning parsing\n\n");
        
        let mut parser = Parser::new(token_stream);

        //parse token_stream, return a stream of bytecode - probably inserted into chunk
        parser.parse(&mut self.ops, &mut self.unordered_values);
    
        println!("done parsing\n");

        self.ops.push(OpCode::RETURN);
        println!("operations at end of parsetime: {:?}", self.ops); 
        println!("unordered values: {:?}", self.unordered_values);


        self.run()
    }


    pub fn run(&mut self) -> Option<IResult> {


        println!("operations vector at start of runtime: {:?}", self.ops);
        for instruction in &self.ops{
            println!("executing {:?}", instruction);
            let result: Option<IResult> = match instruction {
                OpCode::CONSTANT(_) => { 
                    let new_val = self.read_constant();
                    self.values.push(*new_val);
                    None
                },

                OpCode::NEGATE => {
                    match self.values.pop().unwrap() {
                        ValueType::int(s) => {
                            self.values.push(ValueType::int(-s));
                            None
                        },

                        ValueType::float(f) => {
                            self.values.push(ValueType::float(-f));
                            None
                        },

                        _ => None,
                   }
                },

                OpCode::ADD => {
                    println!("Adding");
                    let right = ValueType::extract_int(self.values.pop().unwrap()).unwrap();
                    let left = ValueType::extract_int(self.values.pop().unwrap()).unwrap();

                    println!("{:?} + {:?}", left, right);
                    
                    self.values.push(ValueType::int(left + right));                    
                    None
                },
                
                OpCode::SUBTRACT => {
                    let right = ValueType::extract_int(self.values.pop().unwrap()).unwrap();
                    let left = ValueType::extract_int(self.values.pop().unwrap()).unwrap();

                    println!("{} - {}", left, right);
                    

                    self.values.push(ValueType::int(left - right));
                    None
                },

                OpCode::MULT => {
                    let right = ValueType::extract_int(self.values.pop().unwrap()).unwrap();
                    let left = ValueType::extract_int(self.values.pop().unwrap()).unwrap();

                    println!("{} * {}", left, right);

                    self.values.push(ValueType::int(left * right));
                    None
                },

                OpCode::DIV => {
                    let right = ValueType::extract_int(self.values.pop().unwrap()).unwrap();
                    let left = ValueType::extract_int(self.values.pop().unwrap()).unwrap();
                    
                    println!("{} / {}", left, right);

                    self.values.push(ValueType::int(left / right));
                    None
                },

                OpCode::RETURN => {
                    
                    println!("returned {:?} ", self.values.pop());
                    return Some(IResult::OK);
                },
                _ => Some(IResult::CE),
            };
            println!("values after operation: {:?}", self.values);
            self.ip += 1;
            //return result;
        }
        Some(IResult::OK)
    }

    pub fn debug_trace_stack(&mut self) {
        for va in &self.values{
            println!("{:?}", va);
        }
    }

    pub fn read_constant(&self) -> &ValueType {
        let ind = &self.ops[self.ip];    
        let exind: Option<usize> = match ind {
            OpCode::CONSTANT(s) => Some(*s),
            _ => None,
        };
        &self.unordered_values[exind.unwrap()]
    } 

}

pub enum IResult {
    OK,
    CE,
    RE,
}



