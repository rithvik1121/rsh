use crate::parser::{Precedence, Parser};
use crate::bytecode::{OpCode, ValueType};


#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum TokenType {

    //script operators
    PLUS,
    MINUS,
    MULT,
    DIV,
    ASSIGN,

    LESS,
    GREATER,
    LESS_EQ,
    GREATER_EQ,
    EQ,
    NOT,
    NOT_EQ,

    OR,
    AND,
    
    AMPERSAND,

    //command operators
    OUTPUT,
    INPUT,
    PIPE,


    //symbols
    SEMICOLON,
    LEFT_BRACE,
    RIGHT_BRACE,
    LEFT_PAREN,
    RIGHT_PAREN,
    DOT,
    COMMA,



    IDENTIFIER,
    STRING,
    NUMBER,
    TRUE,
    FALSE,

    COMMAND,

    //keywords
    FOR,
    IF,
    ELSE,
    WHILE,
    RETURN,
    FN,
    PUBLIC,
    CLASS,
    THIS,
    LET,



    ERROR,
    EOF

}

impl TokenType {

    pub fn get_precedence(tok: &TokenType) -> Precedence {
        match tok{
            TokenType::PLUS | TokenType::MINUS => Precedence::TERM,
            TokenType::MULT | TokenType::DIV => Precedence::FACTOR,
            TokenType::COMMAND => Precedence::PRIMARY,

            _ => Precedence::NONE,

        }
    }

    pub fn get_prefix(tok: &TokenType) -> fn(&mut Parser, &mut Vec<OpCode>, &mut Vec<ValueType>) {
        match tok{
            TokenType::MINUS => Parser::unary,
            TokenType::NUMBER => Parser::number,
            TokenType::LEFT_PAREN => Parser::grouping,
            _ => Parser::none,
        }
    }

    pub fn get_infix(tok: &TokenType) -> fn(&mut Parser, &mut Vec<OpCode>, &mut Vec<ValueType>) {
        match tok{
            TokenType::PLUS | TokenType::MINUS | TokenType::MULT | TokenType::DIV => Parser::binary,
            _ => Parser::none,
        }
    }
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub source: String,
    pub line: i32,
}

impl Token {
    pub fn new(ttype: TokenType, source: &str, line: i32) -> Self {
        Self {
            token_type: ttype,
            source: source.to_string(),
            line: line,
        }
    }
    

}


//    impl Ord for Token{
//
//        fn cmp(&self, other: &Self) -> Ordering {
//            println!("Comparing: {:?} with {:?}", self, other);
//            Self::get_precedence(self).cmp(&Self::get_precedence(other))
//        }
//    }

//   impl PartialOrd for Token {
//        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//            Some(self.cmp(other)) 
//        }
//    }


//   impl PartialEq for Token {
//        fn eq(&self, other: &Self) -> bool {
//            Self::get_precedence(self) == Self::get_precedence(other)
//        }
//    }
