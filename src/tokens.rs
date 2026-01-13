use crate::parser::{Precedence, Parser};
use crate::bytecode::Chunk;
use crate::environment::Env;


#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum TokenType {

    //script operators
    PLUS,
    MINUS,
    MULT,
    DIV,
    ASSIGN,
    INITIALIZE,
    ANNOTATION, 
    INT,
    FLOAT,
    CHAR,
    BOOL,
    STR,

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
    NULL,

    COMMAND,
    PARAMETER,

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
    LOOP,



    WRITE,
    ERROR,
    EOF

}

impl TokenType {

    pub fn get_precedence(tok: &TokenType) -> Precedence {
        match tok{
            TokenType::AND => Precedence::AND,
            TokenType::OR => Precedence::OR,
            TokenType::NOT_EQ | TokenType::EQ => Precedence::EQ,
            TokenType::GREATER | TokenType::GREATER_EQ | TokenType::LESS | TokenType::LESS_EQ => Precedence::COMP,
            TokenType::PLUS | TokenType::MINUS  => Precedence::TERM,
            TokenType::MULT | TokenType::DIV | TokenType::INPUT | TokenType::OUTPUT | TokenType::PIPE => Precedence::FACTOR,
            TokenType::LEFT_PAREN => Precedence::CALL,
            _ => Precedence::NONE,

        }
    }

    pub fn get_prefix(tok: &TokenType) -> fn(&mut Parser, &mut Chunk, &mut Env) {
        match tok{
            TokenType::MINUS | TokenType::NOT => Parser::unary,
            TokenType::NUMBER => Parser::number,
            TokenType::LEFT_PAREN => Parser::grouping,
            TokenType::FALSE | TokenType::TRUE | TokenType::NULL | TokenType::STRING => Parser::literal,
            TokenType::IDENTIFIER => Parser::variable,
            TokenType::WRITE | TokenType::IF | TokenType::ELSE | TokenType::LOOP | TokenType::FN => Parser::statement,
            TokenType::COMMAND => Parser::command,
            TokenType::PARAMETER => Parser::command,
            _ => Parser::none,
        }
    }

    pub fn get_infix(tok: &TokenType) -> fn(&mut Parser, &mut Chunk, &mut Env) {
        match tok{
            TokenType::PLUS | TokenType::MINUS | TokenType::MULT | TokenType::DIV |
            TokenType::NOT_EQ | TokenType::EQ | TokenType::GREATER | TokenType::LESS | 
            TokenType::GREATER_EQ | TokenType::LESS_EQ | TokenType::PIPE | TokenType::INPUT |
            TokenType::OUTPUT => Parser::binary,
            TokenType::AND => Parser::and,
            TokenType::OR => Parser::or,
            TokenType::LEFT_PAREN => Parser::func_call,
            
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
