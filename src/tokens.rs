#[allow(non_camel_case_types)]
#[derive(Debug)]
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

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    source: String,
    line: i32,
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
