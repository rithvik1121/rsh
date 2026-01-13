use std::collections::HashMap;
use std::str;
use crate::bytecode::{OpCode, Chunk};
use crate::parser::Parser;
use crate::lexer::Lexer;
use crate::tokens::{Token, TokenType};
use crate::values::ValueType;
use crate::command::Command;
use crate::error::IResult;
use crate::environment::{Env, Variable};



pub struct VM {
    ip: usize,
    values: Vec<ValueType>,
    pub env: Env,
    chunk: Chunk,
    pub command_mode: bool,
}

impl VM {
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
            ip: 0,
            command_mode: false,
            env: Env::new(),
            chunk: Chunk {
                ops: Vec::new(),
                vals: Vec::new(),
                lines: Vec::new(),
            }
        }
    }
    pub fn init() {
                    
    }

    pub fn free(&mut self) {
        self.values = Vec::new();
        self.ip = 0;
        self.chunk = Chunk::new();
        self.env.vars = HashMap::new();
        self.env.scope_depth = 0;
        self.env.globals = HashMap::new();
        self.env.debug = false;
    }

    pub fn alloc() {

    }

    pub fn dealloc() {

    }

    pub fn interpret(&mut self, source: &str) -> Option<IResult> {
        self.ip = 0;

        let mut token_stream: Vec<Token> = Vec::new();
        let mut lexer = Lexer::new(source);
        loop {
            let lexed = lexer.lex();
            if self.env.debug == true {
                println!("lexed: {:?}", lexed);
            }
            match lexed {
                Some(tok) => {
                    match tok.token_type {
                        TokenType::EOF => {
                            token_stream.push(tok);
                            break;
                        },
                        _ => { 
                            token_stream.push(tok);
                            lexer.advance();
                        }
                    };
                },
                None => (),
            };
        }

        if self.env.debug == true {
            println!("\n\nlexing done, beginning parsing\n\n");
        }
        let mut parser = Parser::new(token_stream);
        
        //parse token_stream, return a stream of bytecode - probably inserted into chunk
        parser.parse(&mut self.chunk, &mut self.env);
        if self.env.debug == true {
            println!("done parsing\n");

            println!("operations at end of parsetime: {:?}", self.chunk.ops); 
            println!("unordered values: {:?}", self.chunk.vals);
            println!("environment: {:?}", self.env);
        }

        let result = self.run();

        match result {
            Some(IResult::RE) => {
                println!("Runtime Error");
                result
            }
            _ => result, 
        }
    }


    pub fn run(&mut self) -> Option<IResult> {

        if self.env.debug == true {
            println!("operations vector at start of runtime: {:?}", self.chunk.ops);
        }
        loop {
            if self.ip == self.chunk.ops.len() {
                break;
            }
            if self.env.debug == true {
                println!("executing {:?}", self.chunk.ops[self.ip]);
            }
            let result: Option<IResult> = match self.chunk.ops[self.ip] {
                OpCode::EXEC => {

                    None
                },
                OpCode::COMMAND(_) => {
                    let new_val = self.read_constant();

                    let mut newc = Command::new(&ValueType::extract_str(new_val.clone()).unwrap());
                    newc.match_execute();
                    None
                },
                OpCode::ARGUMENT(_) => {
                    let new_val = self.read_constant();
                    None
                },
                
                OpCode::INPUT => {
                    None
                },
                OpCode::OUTPUT => {
                    None
                },

                OpCode::PIPE => {
                    None
                },

                OpCode::CONSTANT(_) => { 
                    //TODO: Custom String type so I can change this to deref new_val instead of
                    //clone;

                    let new_val = self.read_constant();
                    self.values.push(new_val.clone());
                    None
                },
                OpCode::SCOPEUP => {
                    self.env.scope_depth+=1;
                    None
                },
                OpCode::SCOPEDOWN => {
                    self.env.scope_depth -= 1;
                    None
                },

                OpCode::VAR(scope) => {
                    let name = &ValueType::extract_str(self.values[self.values.len()-2].clone()).unwrap();
                    let var = Variable { 
                        value: self.values.pop().unwrap(),
                        scope: scope,
                    };
                    if self.env.debug == true {
                        println!("variable scope: {}", scope);
                        println!("Variable name: {}", name);
                        println!("Variable value: {}", &var.value);
                    }
                    self.env.vars.insert(
                        name.to_string(), 
                        var
                    );
                    None
                },
                OpCode::GETVAR => {
                    let name_wrapped = &self.values.pop().unwrap();
                    let name = ValueType::extract_str(name_wrapped.clone()).unwrap().clone();
                    let mut var_found = false;
                    if !self.env.vars.contains_key(&name) {
                        eprintln!("Variable {} does not exist", name); 
                        break;
                    }
                    else if self.env.vars[&name].scope > self.env.scope_depth {
                        eprintln!("Varible {} is out of scope", name);
                        println!("\ncurrent scope: {}, var scope: {}", self.env.scope_depth, self.env.vars[&name].scope);
                        break;
                    }
                    else {
                        let value = &self.env.vars[&name];
                        self.values.push(value.value.clone());
                        var_found = true;
                        //println!("Variable: {:?} current scope: {}", value, self.env.scope_depth);
                    }

                    if var_found == false {
                        return Some(IResult::RE);
                    }
                    None
                },
                OpCode::SETVAR => {
                    
                    let name = ValueType::extract_str(self.values[self.values.len()-2].clone()).unwrap();
                    let vscope = self.env.vars[&name].scope;
                    //println!("variable name retrieved: {}", name);
                    if !self.env.vars.contains_key(&name) {
                        eprintln!("Variable {} does not exist", name); 
                    }
                    else if self.env.vars[&name].scope > self.env.scope_depth {
                        eprintln!("Varible {} is out of scope", name);
                        println!("\ncurrent scope: {}, var scope: {}", self.env.scope_depth, self.env.vars[&name].scope);
                        break;
                    }
                    else {
                        //println!("replacing variable value with: ");
                        
                        self.env.vars.insert(name, 
                            Variable {
                                value: self.values.pop().unwrap(),
                                scope: vscope,
                            }
                        );
                    }
                    None
                },

                OpCode::NULL => {
                    self.values.push(ValueType::null);
                    None
                },
                OpCode::TRUE => {
                    self.values.push(ValueType::bool(true));
                    None
                }
                OpCode::FALSE => {
                    self.values.push(ValueType::bool(false));
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

                        _ => {
                            eprintln!("Cannot negate non-numerical operand");
                            Some(IResult::RE)
                        },
                   }
                },
                OpCode::NOT => {
                    match self.values.pop().unwrap() {
                        ValueType::bool(b) => {
                            self.values.push(ValueType::bool(!b));
                            None
                        },
                        _ => {
                            eprintln!("Cannot take logical negation of non-boolean operand");
                            Some(IResult::RE)
                        }
                    }
                }

                OpCode::EQ => {
                    let right = self.values.pop().unwrap();
                    let left = self.values.pop().unwrap();
                    let mut result = ValueType::bool(false);

                    match left {
                        ValueType::bool(lb) => {
                            match right {
                                ValueType::bool(rb) => {
                                    result = ValueType::bool(lb == rb);
                                },
                                _ => (),
                            }
                        }
                        ValueType::int(li) => {
                            match right {
                                ValueType::int(ri) => {
                                    result = ValueType::bool(li == ri);
                                },
                                _ => (),
                            }
                        },
                        ValueType::float(lf) => {
                            match right {
                                ValueType::float(rf) => {
                                    result = ValueType::bool(lf == rf);
                                },
                                _ => (),
                            }
                        },
                        ValueType::char(lc) => {
                            match right {
                                ValueType::char(rc) => {
                                    result = ValueType::bool(lc == rc);
                                },
                                _ => (),
                            }
                        },
                        _ => (),
                    };
                    self.values.push(result);
                    None
                },

                OpCode::GREATER => {
                    let right = self.values.pop().unwrap();
                    let left = self.values.pop().unwrap();

                    let mut result = ValueType::bool(false);

                    match left {
                        ValueType::int(li) => {
                            match right {
                                ValueType::int(ri) => result = ValueType::bool(li > ri),
                                ValueType::float(rf) => result = ValueType::bool(li as f32 > rf),
                                _ => (),
                            };
                        },
                        ValueType::float(lf) => {
                            match right {
                                ValueType::int(ri) => result = ValueType::bool(lf > ri as f32),
                                ValueType::float(rf) => result = ValueType::bool(lf > rf),
                                _ => (),
                            };
                        },
                        _ => (),
                    };
                    self.values.push(result);
                    None
                },

                OpCode::LESS=> {
                    let right = self.values.pop().unwrap();
                    let left = self.values.pop().unwrap();

                    let mut result = ValueType::bool(false);

                    match left {
                        ValueType::int(li) => {
                            match right {
                                ValueType::int(ri) => result = ValueType::bool(li < ri),
                                ValueType::float(rf) => result = ValueType::bool((li as f32) < rf),
                                _ => (),
                            };
                        },
                        ValueType::float(lf) => {
                            match right {
                                ValueType::int(ri) => result = ValueType::bool(lf < ri as f32),
                                ValueType::float(rf) => result = ValueType::bool(lf < rf),
                                _ => (),
                            };
                        },
                        _ => (),
                    };
                    self.values.push(result);
                    None
                },
                OpCode::ADD => {
                    let right = self.values.pop().unwrap();
                    let left = self.values.pop().unwrap();

                    self.values.push(left + right);                    
                    None
                },
                
                OpCode::SUBTRACT => {
                    let right = self.values.pop().unwrap();
                    let left = self.values.pop().unwrap();

                    self.values.push(left - right);
                    None
                },

                OpCode::MULT => {
                    let right = self.values.pop().unwrap();
                    let left = self.values.pop().unwrap();


                    self.values.push(left * right);
                    None
                },

                OpCode::DIV => {
                    let right = self.values.pop().unwrap();
                    let left = self.values.pop().unwrap();
                    

                    self.values.push(left / right);
                    None
                },
                OpCode::WRITE => {
                    match self.values.pop().unwrap() {
                        ValueType::int(i) => println!("integer: {}", i),
                        ValueType::bool(b) => println!("boolean: {}", b),
                        ValueType::char(c) => println!("char: {}", c),
                        ValueType::float(f) => println!("float: {}", f),
                        ValueType::str(s) => println!("string: {}", s),
                        ValueType::null => println!("null"),
                        _ => (),
                    };
                    None
                },
                OpCode::IF(s) => {
                    match self.values[0] {
                        ValueType::bool(false) => {
                            self.ip += s;
                            continue;
                        },
                        ValueType::bool(true) => (),
                        _ => {
                            eprintln!("IF condition must evaluate to a boolean");
                            break;
                        }
                    };
                    None
                },
                OpCode::JUMP(s) => {
                    if !(s == 0) {
                        if s < 0 {
                            let signed_ip = self.ip as isize;
                            self.ip = (signed_ip + s) as usize;
                            continue;
                        }
                        else {
                            self.ip += (s - 1) as usize;
                            continue;
                        }
                    }
                    None
                },
                OpCode::ENDLOOP(s) => {
                    None
                },
                OpCode::OR(s) => {
                    match self.values[0] {
                        ValueType::bool(true) => {
                            self.ip += s as usize;
                            continue;
                        },
                        ValueType::bool(false) => (),
                        _ => {
                            eprintln!("OR condition must evaluate to a boolean");
                            break;
                        }
                    };
                    None
                },

                OpCode::CALL(count) => {
                    let fname = ValueType::extract_str(self.values[self.values.len() - 1 - count].clone()).unwrap(); 
                    let mut ind = 1;
                    if self.env.debug==true {
                        println!("function name hopefully: {:?}", fname);
                        println!("FUnction ops: {:?}", self.env.functions[&fname]);
                    }
                    for oper in &self.env.functions[&fname].chk.ops {
                        self.chunk.ops.insert(self.ip+ind, oper.clone());
                        if self.env.debug==true {
                            println!("Inserted op: {:?}", oper);
                        }
                        ind+=1;
                    }
                    for val in &self.env.functions[&fname].chk.vals {
                        self.values.push(val.clone());
                        if self.env.debug==true {
                            println!("Inserted val: {:?}", val);
                        }
                    }

                    None
                },
                OpCode::POP => {
                    self.values.pop();
                    None
                },

                OpCode::RETURN => {
                    
                    if self.env.debug==true {
                        println!("returned {:?} ", self.values.pop());
                    }

                    return Some(IResult::OK);
                },
                _ => Some(IResult::CE),
            };
            if self.env.debug == true {
                println!("operation executed: {:?}\n", self.chunk.ops[self.ip]);
                println!("values after operation: {:?}\n", self.values);
                println!("chunk values after operation: {:?}\n", self.chunk.vals);
                println!("Variables after operation: {:?}\n", self.env.vars);
                println!("Operations left: {:?}", &self.chunk.ops[self.ip+1..self.chunk.ops.len()-1]);
                println!("\n\n\n");
            }
            self.ip += 1;
        }
        Some(IResult::OK)
    }

    pub fn debug_trace_stack(&mut self) {
        for va in &self.values{
            println!("{:?}", va);
        }
    }

    pub fn read_constant(&self) -> &ValueType {
        let ind = &self.chunk.ops[self.ip];    
        let exind: Option<usize> = match ind {
            OpCode::CONSTANT(s) => Some(*s),
            OpCode::COMMAND(c) => Some(*c),
            _ => None,
        };
        &self.chunk.vals[exind.unwrap()]
    } 

}



#[derive(Debug)]
pub struct Frame {

}

