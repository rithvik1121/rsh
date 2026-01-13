use std::collections::HashMap;
use crate::tokens::{Token, TokenType};
use crate::bytecode::{OpCode, Chunk};
use crate::values::{ValueType, Function};
use crate::environment::{ Env, Variable} ;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    NONE,
    ASSIGN,
    OR,
    AND,
    EQ,
    COMP,
    TERM,
    FACTOR,
    UNARY,
    CALL,
    PRIMARY,
}


#[derive(Debug)]
pub struct Parser {
    stream: Vec<Token>,
    cur: usize,
    panic_mode: bool,
}

impl Parser {
    pub fn new(token_stream: Vec<Token>) -> Self{
        Self {
            cur : 0,
            stream : token_stream,
            panic_mode : false,
        }
    } 

    pub fn transform_command(&mut self, env: &mut Env) {
        let mut found_command = 0;
        for index in 0..self.stream.len() {
            if env.debug==true {
                println!("found command: {}", found_command);
            }
            match self.stream[index].token_type {
                TokenType::IDENTIFIER => {
                    if !env.globals.contains_key(&self.stream[index].source) {
                        if found_command == 0 {
                            if index >= 1 {
                                match self.stream[index - 1].token_type {
                                    TokenType::LET => println!("not command, initialization"),
                                    _ => self.stream[index].token_type = TokenType::COMMAND,
                                }
                            }
                            else {
                                println!("Command token transformed");
                                self.stream[index].token_type = TokenType::COMMAND;
                            }
                            found_command = 1;

                        } else {
                            match self.stream[index].token_type {
                                TokenType::PIPE => {
                                    found_command = 0; 
                                },
                                _ => {
                                    self.stream[index].token_type = TokenType::PARAMETER;
                                }
                            };
                        }
                    }
                },
                _ => (),
            }

        }

    }
    
    pub fn parse(&mut self, into: &mut Chunk, env: &mut Env) {
        //self.transform_command(env);
        //println!("Source: {:?}", self.stream);
        loop {
            match self.stream[self.cur].token_type {
                TokenType::EOF => {
                    break;
                },
                _ => {
                    self.declaration(into, env);
                }
            }
        }
        into.ops.push(OpCode::RETURN);
    }

    pub fn insert_byte(&mut self, byte: OpCode, into: &mut Chunk) {
        into.push_code(byte, 0);
    }

    pub fn advance(&mut self) {
        self.cur += 1;
    }

    pub fn declare_variable(&mut self, into: &mut Chunk, env: &mut Env) {
        self.advance();
        let mut name = String::new();
        match self.stream[self.cur].token_type {
            TokenType::IDENTIFIER => {
                name = self.stream[self.cur].source.to_string();

                into.vals.push(ValueType::str(name.to_string()));        
                into.ops.push(OpCode::CONSTANT(into.vals.len()-1));
            },
            _ => eprintln!("Expected identifier, got {:?} on line {}", self.stream[self.cur], self.stream[self.cur].line),
        };

        self.advance();
        match self.stream[self.cur].token_type {
            TokenType::INITIALIZE => {
                self.advance();
                self.expression(into, env);
            },
            _ => into.vals.push(ValueType::null),
        };


        //self.advance();
        match self.stream[self.cur].token_type {
            TokenType::SEMICOLON => {
                into.ops.push(OpCode::VAR(env.scope_depth));
                into.ops.push(OpCode::POP);
            },
            TokenType::EOF => into.ops.push(OpCode::RETURN),
            _ => {
                if env.debug==true {
                    println!("Current token: {:?}", self.stream[self.cur]);
                    eprintln!("varible declaration missing Semicolon on line {}", self.stream[self.cur].line);
                }
            }
        };
        
    }


    pub fn declaration(&mut self, into: &mut Chunk, env: &mut Env) {
        //println!("Current scope at declaration call: {}", env.scope_depth);
        match self.stream[self.cur].token_type {
            TokenType::LET => self.declare_variable(into, env),
            _ => self.statement(into, env),
        }
    }

    pub fn statement(&mut self, into: &mut Chunk, env: &mut Env) {
        //println!("Current scope at statement call: {}\n\n\n", env.scope_depth);
        match self.stream[self.cur].token_type {
            TokenType::WRITE => {
                self.advance();
                self.expression(into, env);
                //match self.stream[self.cur].token_type {
                //    TokenType::COMMA => self.number(into, values),
                //    _ => (),
                //}
                self.advance();
                into.ops.push(OpCode::WRITE);
            }
            TokenType::IF => {
                self.if_statement(into, env);

            }
            TokenType::WHILE => {},
            TokenType::FN => {
                self.func(into, env);
            },

            TokenType::LOOP => {
                self.loop_statement(into, env);
            },
            TokenType::FOR => {},
            TokenType::LEFT_BRACE => {
                Self::begin_scope(into, env);
                //println!("Scope depth: {}", env.scope_depth);
                self.block(into, env);
                Self::end_scope(into, env);
            }
            _ => {
                self.expression(into, env);
                match self.stream[self.cur].token_type {
                    TokenType::SEMICOLON => {
                        //println!("Found semicolon on line {}", self.stream[self.cur].line);
                        into.ops.push(OpCode::POP);
                    }
                    TokenType::EOF => eprintln!("Semicolon missing on line {}", self.stream[self.cur].line),

                    _ => (),
                }
            },
        };
    }

    pub fn expression(&mut self, into: &mut Chunk, env: &mut Env) {
        self.parse_precedence(into, env, Precedence::ASSIGN as isize);
    }

    pub fn func(&mut self, into: &mut Chunk, env: &mut Env) {
        self.advance();
        let fn_name = self.stream[self.cur].source.clone();
        self.advance();
        match self.stream[self.cur].token_type {
            TokenType::LEFT_PAREN => self.advance(),
            _ => eprintln!("Function parameters not properly formatted"),
        };

        let mut new_fn = Function::new();

        loop {
            match self.stream[self.cur].token_type {
                TokenType::RIGHT_PAREN => break,
                TokenType::IDENTIFIER => {
                    if env.debug==true {
                        println!("Parsing func param");
                    }
                    new_fn.params.push(self.stream[self.cur].source.clone());
                },
                _ => (),
            };
            if env.debug==true {
                println!("{:?}", self.stream[self.cur]);
            }
            self.advance();
        }
        
        self.advance(); 

        if into.ops.len() != 0 {
            new_fn.location = into.ops.len()-1;
        } 
        else {
            new_fn.location = 0;
        }
        self.statement(&mut new_fn.chk, env);

        env.functions.insert(fn_name, new_fn);
        

    }

    pub fn func_call(&mut self, into: &mut Chunk, env: &mut Env) {
        if env.debug==true {
            println!("Function called: {:?}",self.stream[self.cur-1].source);        
        }
        //let name = self.stream[self.cur-1].source.clone();
        let mut count = 0;
        self.advance();
        if env.debug==true {
            println!("Current token before expression ingestion loop: {:?}", self.stream[self.cur]);
        }
        match self.stream[self.cur].token_type {
            TokenType::RIGHT_PAREN => (),
            _ => {
                loop {
                    if env.debug==true {
                        println!("Current token within loop: {:?}", self.stream[self.cur]);
                    }
                    self.expression(into, env);
                    count+=1;
                    match self.stream[self.cur].token_type {
                        TokenType::COMMA => self.advance(),
                        _ => break,
                    };
                }
            },
        };

        match self.stream[self.cur].token_type {
            TokenType::RIGHT_PAREN => self.advance(),
            _ => eprintln!("Function call not properly closed"),
        };
        into.ops.push(OpCode::CALL(count));
    }

    pub fn if_statement(&mut self, into: &mut Chunk, env: &mut Env) {
        self.advance();
        match self.stream[self.cur].token_type {
            TokenType::LEFT_PAREN => self.advance(),
            _ => eprintln!("IF condition not properly formatted"),
        };


        self.expression(into, env);



        match self.stream[self.cur].token_type {
            TokenType::RIGHT_PAREN => self.advance(),
            _ => eprintln!("IF condition not properly formatted"),
        };

        into.ops.push(OpCode::IF(0));
        let cur_ind = into.ops.len()-1;

        into.ops.push(OpCode::POP);
        self.statement(into, env); 
        let fin_ind = into.ops.len() ;
        into.ops[cur_ind] = OpCode::IF(fin_ind - cur_ind + 1 as usize);
    

        into.ops.push(OpCode::JUMP(0));
        let else_start = into.ops.len()-1;
        
        into.ops.push(OpCode::POP);
        match self.stream[self.cur].token_type {
            TokenType::ELSE => {
                self.statement(into, env);
                let else_ind = into.ops.len();
                into.ops[else_start] = OpCode::JUMP((else_ind - else_start) as isize);
            },
            _ => (), 
        };
    }

    pub fn loop_statement(&mut self, into: &mut Chunk, env: &mut Env) {
        self.advance();
        match self.stream[self.cur].token_type {
            TokenType::LEFT_PAREN => self.advance(),
            _ => eprintln!("LOOP condition not properly formatted"),
        };


        let start_cond = into.ops.len() - 1;
        self.expression(into, env);

        match self.stream[self.cur].token_type {
            TokenType::RIGHT_PAREN => self.advance(),
            _ => eprintln!("LOOP condition not properly formatted"),
        };

        into.ops.push(OpCode::IF(0));
        let cur_ind = into.ops.len()-1;

        self.statement(into, env); 
        let fin_ind = into.ops.len() ;
        let distance = fin_ind - start_cond;
        into.ops[cur_ind] = OpCode::IF(distance + 1);
        into.ops.push(OpCode::POP);
        into.ops.push(OpCode::JUMP( -1 * (distance as isize)));
        into.ops.push(OpCode::POP);
    }

    pub fn block(&mut self, into: &mut Chunk, env: &mut Env) {
        //println!("\n\ncode block, scope: {}\n\n", env.scope_depth);
        self.advance();
        loop {
            match self.stream[self.cur].token_type {
                TokenType::RIGHT_BRACE => {
                    self.advance();
                    break;
                }
                TokenType::EOF => {
                    eprintln!("Unclosed braces");
                    break;
                }
                _ => self.declaration(into, env),
            }
        }
    }

    pub fn begin_scope(into: &mut Chunk, env: &mut Env) {
        into.ops.push(OpCode::SCOPEUP);
        env.scope_depth += 1;
    }

    pub fn end_scope(into: &mut Chunk, env: &mut Env) {
        into.ops.push(OpCode::SCOPEDOWN);
        env.scope_depth -= 1;
    }

    pub fn grouping(&mut self, into: &mut Chunk, env: &mut Env) {
        self.expression(into, env);

        match self.stream[self.cur].token_type {
            TokenType::RIGHT_PAREN => self.advance(),
            _ => eprintln!("Unclosed parentheses on line {}", self.stream[self.cur].line),
        };
    }

    pub fn unary(&mut self, into: &mut Chunk, env: &mut Env) {

        let optype: &TokenType = &(self.stream[self.cur - (1 as usize)].token_type.clone());

        self.parse_precedence(into, env, Precedence::UNARY as isize);

        match optype {
            TokenType::MINUS => {
                self.insert_byte(OpCode::NEGATE, into);
            },
            TokenType::NOT => {
                into.ops.push(OpCode::NOT);
            },
            _ => (),
        };
    }

    pub fn command(&mut self, into: &mut Chunk, _env: &mut Env) {
        let pre = &self.stream[self.cur - (1 as usize)];

        match pre.token_type {
            TokenType::COMMAND => {
                into.vals.push(ValueType::cmd(pre.source.clone()));
                into.ops.push(OpCode::COMMAND(into.vals.len()-1));  
            },
            TokenType::PARAMETER => {
                into.vals.push(ValueType::str(pre.source.clone()));
                into.ops.push(OpCode::ARGUMENT(into.vals.len()-1));
            },
            TokenType::SEMICOLON | TokenType::EOF => into.ops.push(OpCode::EXEC),
            _ => eprintln!("Expected Command or Parameter token, got {:?}", pre.token_type),
        }
    }

    pub fn binary(&mut self, into: &mut Chunk, env: &mut Env) {
        
        let pre: &TokenType = &self.stream[self.cur - (1 as usize)].token_type.clone();
        

        self.parse_precedence(into, env, (TokenType::get_precedence(&pre) as isize) + (1 as isize));


        match pre {
            TokenType::PIPE => {
                self.insert_byte(OpCode::PIPE, into);
            },
            TokenType::INPUT => {
                self.insert_byte(OpCode::INPUT, into);
            },
            TokenType::OUTPUT => {
                self.insert_byte(OpCode::OUTPUT, into);
            },
            TokenType::PLUS => {
                self.insert_byte(OpCode::ADD, into);
            },
            TokenType::MINUS => {
                self.insert_byte(OpCode::SUBTRACT, into);
            },
            TokenType::MULT => {
                self.insert_byte(OpCode::MULT, into);
            },
            TokenType::DIV => {
                self.insert_byte(OpCode::DIV, into);
            },
            TokenType::NOT_EQ => {
                self.insert_byte(OpCode::EQ, into);
                self.insert_byte(OpCode::NOT, into);
            },
            TokenType::EQ => {
                self.insert_byte(OpCode::EQ, into);
            },
            TokenType::GREATER => {
                self.insert_byte(OpCode::GREATER, into);
            },
            TokenType::GREATER_EQ => {
                self.insert_byte(OpCode::LESS, into);
                self.insert_byte(OpCode::NOT, into);
            },
            TokenType::LESS => {
                self.insert_byte(OpCode::LESS, into);
            },
            TokenType::LESS_EQ => {
                self.insert_byte(OpCode::GREATER, into);
                self.insert_byte(OpCode::NOT, into);
            },
            _ => (),
        };
    }

    pub fn none(&mut self, _into: &mut Chunk, _env: &mut Env) {}

    pub fn and(&mut self, into: &mut Chunk, env: &mut Env) {
        into.ops.push(OpCode::IF(0)); 
        let cur_ind = into.ops.len() - 1;
        into.ops.push(OpCode::POP);

        self.parse_precedence(into, env, Precedence::AND as isize);

        let end_ind = into.ops.len();
        into.ops[cur_ind] = OpCode::IF(end_ind - cur_ind);
    }
    pub fn or(&mut self, into: &mut Chunk, env: &mut Env) {
        into.ops.push(OpCode::OR(0));
        let cur_ind = into.ops.len() - 1;
        into.ops.push(OpCode::POP);

        self.parse_precedence(into, env, Precedence::OR as isize); 
        let fin_ind = into.ops.len();

        into.ops[cur_ind] = OpCode::OR(fin_ind-cur_ind);
    }

    pub fn number(&mut self, into: &mut Chunk, env: &mut Env) {
        let src = &self.stream[self.cur - (1 as usize)].source;

        if src.contains(".") {
            let float: f32 = src.parse::<f32>().unwrap();
            into.vals.push(ValueType::float(float));
            into.ops.push(OpCode::CONSTANT(into.vals.len() - 1));
        }
        else {
            let int: i32 = self.stream[self.cur - (1 as usize)].source.parse::<i32>().unwrap();
            into.vals.push(ValueType::int(int));
            into.ops.push(OpCode::CONSTANT(into.vals.len() - 1));
        }
    }

    pub fn literal(&mut self, into: &mut Chunk, env: &mut Env) {
        match self.stream[self.cur - (1 as usize)].token_type {
            TokenType::FALSE => into.ops.push(OpCode::FALSE),
            TokenType::TRUE => into.ops.push(OpCode::TRUE),
            TokenType::STRING => {
                into.vals.push(ValueType::str(self.stream[self.cur - (1 as usize)].source.clone()));
                into.ops.push(OpCode::CONSTANT(into.vals.len()-1));
            },
            TokenType::NULL => into.ops.push(OpCode::NULL),
            _ => (),
        };
    }

    pub fn variable(&mut self, into: &mut Chunk, env: &mut Env) {
        self.named_variable(self.stream[self.cur - (1 as usize)].source.clone(), into, env); 
    }

    pub fn named_variable(&mut self, previous_name: String, into: &mut Chunk, env: &mut Env) {
        into.vals.push(ValueType::str(previous_name));
        into.ops.push(OpCode::CONSTANT(into.vals.len()-1));
        match self.stream[self.cur].token_type {
            TokenType::ASSIGN => {
                self.advance();
                self.expression(into, env);
                into.ops.push(OpCode::SETVAR);
            },
            TokenType::LEFT_PAREN => {
                self.func_call(into, env);
            },
            _ => into.ops.push(OpCode::GETVAR),
        }

    }

    pub fn parse_precedence(&mut self, into: &mut Chunk, env: &mut Env, prec: isize) {
        match self.stream[self.cur].token_type {
            TokenType::EOF => { into.ops.push(OpCode::RETURN);}
            _ => (),
        };
        self.advance();
        let prefix_rule = TokenType::get_prefix(&self.stream[self.cur - (1 as usize)].token_type);
        //println!("token: {:?}, prefix rule:{:?}", self.stream[self.cur - (1 as usize)], prefix_rule);

        if prefix_rule == Self::none {
            match self.stream[self.cur - (1 as usize)].token_type {
                TokenType::SEMICOLON | TokenType::RIGHT_BRACE => (),
                _ => eprintln!("Expected prefix expression for {:?}", self.stream[self.cur - (1 as usize)]),
            }
            return;
        }

        prefix_rule(self, into, env);

        while prec <= TokenType::get_precedence(&self.stream[self.cur].token_type) as isize {
            self.advance();
            let infix_rule = TokenType::get_infix(&self.stream[self.cur - (1 as usize)].token_type);
            infix_rule(self, into, env);
        }
    }

    pub fn expect(_ttype: TokenType) {

    }

    pub fn parse_command(&mut self) {

    }
}
