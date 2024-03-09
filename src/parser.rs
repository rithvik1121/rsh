use crate::tokens::{Token, TokenType};
use crate::bytecode::{ValueType, OpCode, Chunk};

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

    pub fn transform_command(&mut self) {
        if self.stream.len() == 1 {
            match self.stream[0].token_type {
                TokenType::IDENTIFIER => {
                    self.stream[0].token_type = TokenType::COMMAND;
                },
                _ => {
                    eprintln!("Invalid token");
                },
            };
        }
        else {
            for token in &mut self.stream {
                match token.token_type {
                    TokenType::LESS => {token.token_type = TokenType::INPUT;},
                    TokenType::GREATER => {token.token_type = TokenType::OUTPUT;},
                    _ => (),
                    
                }
            }
        }
    }
    
    pub fn parse(&mut self, into: &mut Vec<OpCode>, values: &mut Vec<ValueType>) {
        self.expression(into, values);
    }

    pub fn insert_byte(&mut self, byte: OpCode, into: &mut Vec<OpCode>) {
        into.push(byte);
    }

    pub fn advance(&mut self) {
        self.cur += 1;
    }

    pub fn expression(&mut self, into: &mut Vec<OpCode>, values: &mut Vec<ValueType>) {
        println!("Parsing token: {:?}", self.stream[self.cur]);
        self.parse_precedence(into, values, Precedence::ASSIGN as isize);
    }

    pub fn grouping(&mut self, into: &mut Vec<OpCode>, values: &mut Vec<ValueType>) {
        self.expression(into, values);

        match self.stream[self.cur].token_type {
            TokenType::RIGHT_PAREN => self.advance(),
            _ => eprintln!("Unclosed parentheses"),
        };
    }

    pub fn unary(&mut self, into: &mut Vec<OpCode>, values: &mut Vec<ValueType>) {

        let optype: &TokenType = &(self.stream[self.cur - (1 as usize)].token_type.clone());

        self.parse_precedence(into, values, Precedence::UNARY as isize);

        match optype {
            TokenType::MINUS => {
                self.insert_byte(OpCode::NEGATE, into);
            }
            _ => (),
        };
    }

    pub fn binary(&mut self, into: &mut Vec<OpCode>, values: &mut Vec<ValueType>) {
        let pre: &TokenType = &self.stream[self.cur - (1 as usize)].token_type.clone();

        self.parse_precedence(into, values, (TokenType::get_precedence(&pre) as isize) + (1 as isize));


        match pre {
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
            _ => (),
        };
    }

    pub fn none(&mut self, _into: &mut Vec<OpCode>, _values: &mut Vec<ValueType>) {}


    pub fn number(&mut self, into: &mut Vec<OpCode>, values: &mut Vec<ValueType>) {
        let int: i32 = self.stream[self.cur - (1 as usize)].source.parse::<i32>().unwrap();
        values.push(ValueType::int(int));
        into.push(OpCode::CONSTANT(values.len() - 1));
    }

    pub fn parse_precedence(&mut self, into: &mut Vec<OpCode>, values: &mut Vec<ValueType>, prec: isize) {
        self.advance();
        let prefix_rule = TokenType::get_prefix(&self.stream[self.cur - (1 as usize)].token_type);
        println!("token: {:?}, prefix rule:{:?}", self.stream[self.cur - (1 as usize)], prefix_rule);

        if prefix_rule == Self::none {
            eprintln!("Expected expression");
            return;
        }

        prefix_rule(self, into, values);

        while prec <= TokenType::get_precedence(&self.stream[self.cur].token_type) as isize {
            self.advance();
            let infix_rule = TokenType::get_infix(&self.stream[self.cur - (1 as usize)].token_type);
            infix_rule(self, into, values);
        }
    }

    pub fn expect(_ttype: TokenType) {

    }

    pub fn parse_command(&mut self) {

    }
}
