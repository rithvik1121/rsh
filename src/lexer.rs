use crate::tokens::TokenType;
use crate::tokens::Token;
use crate::source::Source;
use std::str::Chars;
use std::iter::Peekable;

//lexer
//
#[derive(Debug)]
pub struct Lexer {
    line: i32,
    source: Source,
    pub current: char,

}


impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            line: 1,
            source: Source::new(source),
            current: source.chars().nth(0).unwrap(),
        }
    }

    pub fn lex(&mut self) -> Token {
        match self.current {
            ' ' | '\t' | '\r' => self.advance(),
            '\n' => {self.line+=1; self.advance();},
            _ => (),
        };
        match self.current {
            '(' => Token::new(TokenType::LEFT_PAREN, "(", self.line),
            ')' => Token::new(TokenType::RIGHT_PAREN, ")", self.line),
            '{' => Token::new(TokenType::LEFT_BRACE, "{", self.line),
            '}' => Token::new(TokenType::RIGHT_BRACE, "}", self.line),
            ';' => Token::new(TokenType::SEMICOLON, ";", self.line),
            ',' => Token::new(TokenType::COMMA, ",", self.line),
            '.' => Token::new(TokenType::DOT, ".", self.line),
            '+' => Token::new(TokenType::PLUS, "+", self.line),
            '-' => Token::new(TokenType::MINUS, "-", self.line),
            '*' => Token::new(TokenType::MULT, "*", self.line),
            '/' => Token::new(TokenType::DIV, "/", self.line), 
            '!' => {
                match self.source.peek(){
                    '=' => {
                        self.advance();
                        Token::new(TokenType::NOT_EQ, "!=", self.line)
                    },
                    _ => Token::new(TokenType::NOT, "!", self.line),
                }
            }
            '=' => {
                match self.source.peek() {
                    '=' => {
                        self.advance();
                        Token::new(TokenType::EQ, "==", self.line)
                    },
                    _ => Token::new(TokenType::ASSIGN, "=", self.line),
                }
            }
            '<' => {
                match self.source.peek() {
                    '=' => {
                        self.advance();
                        Token::new(TokenType::LESS_EQ, "<=", self.line)
                    },
                    _ => Token::new(TokenType::LESS, "<", self.line),
                }
            }
            '>' => {
                match self.source.peek() {
                    '='  => {
                        self.advance();
                        Token::new(TokenType::GREATER_EQ, ">=", self.line)
                    },
                    _ => Token::new(TokenType::GREATER, ">", self.line),
                }
            }
            '"' => {
                let mut str_raw = "".to_string();
                self.advance();
                while (!self.eof() && self.current != '"') {
                    if self.current == '\n' {
                        self.line += 1;
                    }
//                    println!("{}", self.current);
                    str_raw.push(self.current);
                    self.advance();
                }
                if self.eof() {
                    return Token::new(TokenType::ERROR, "Unterminated string", self.line);
                }
                let str_tok = Token::new(TokenType::STRING, &str_raw, self.line);
                return str_tok;
            }
            
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                let mut num_raw = "".to_string();
//                println!("{}", self.current);
  //              println!("{}", self.source.peek());
                while (self.current <= '9' && self.current >= '0') {
                    //println!("{}", self.current);
                    num_raw.push(self.current);
                    if (self.source.peek() <= '9') && (self.source.peek() >= '0') {
                        self.advance();
                    }
                    else {
                        break;
                    }
                }

                let num_tok = Token::new(TokenType::NUMBER, &num_raw, self.line);
                return num_tok;
            }


            '\0' => Token::new(TokenType::EOF, "", self.line),
            
            _ => {
                if Self::is_alpha(self.current) {
                    let mut ident_raw = "".to_string();
                    while (Self::is_alpha(self.current)) {
                        ident_raw.push(self.current);
                        self.advance();
                    }

                        match char::from(ident_raw.as_bytes()[0]) {
                            'f' => {
                                match ident_raw.as_str() {
                                    "for" => Token::new(TokenType::FOR, "for", self.line),
                                    "fn" => Token::new(TokenType::FN, "fn", self.line),
                                    "false" => Token::new(TokenType::FALSE, "false", self.line),
                                    _ => Token::new(TokenType::IDENTIFIER, &ident_raw, self.line),
                                }
                                 
                            }
                            't' => {
                                match ident_raw.as_str() {
                                    "this" => Token::new(TokenType::THIS, "this", self.line),
                                    "true" => Token::new(TokenType::TRUE, "true", self.line),
                                    _ => Token::new(TokenType::IDENTIFIER, &ident_raw, self.line),
                                }
                            }
                            'i' => {
                                match ident_raw.as_str() {
                                    "if" => return Token::new(TokenType::IF, "if", self.line),
                                    _ => Token::new(TokenType::IDENTIFIER, &ident_raw, self.line),
                                }
                            }
                            'r' => {
                                match ident_raw.as_str() {
                                    "return" => Token::new(TokenType::RETURN, "return", self.line),
                                    _ => Token::new(TokenType::IDENTIFIER, &ident_raw, self.line),
                                
                                }
                            }
                            'p' => {
                                match ident_raw.as_str() {
                                    "public" => Token::new(TokenType::PUBLIC, "public", self.line),
                                    _ => Token::new(TokenType::IDENTIFIER, &ident_raw, self.line),
                                }
                            }
                            'c' => {
                                match ident_raw.as_str() {
                                    "class" => Token::new(TokenType::CLASS, "class", self.line),
                                    _ => Token::new(TokenType::IDENTIFIER, &ident_raw, self.line),
                                }
                            }
                            'w' => {
                                match ident_raw.as_str() {
                                    "while" => Token::new(TokenType::WHILE, "while", self.line),
                                    _ => Token::new(TokenType::IDENTIFIER, &ident_raw, self.line),
                                }
                                    
                            }
                            'l' => {
                                match ident_raw.as_str() {
                                    "let" => Token::new(TokenType::LET, "let", self.line),
                                    _ => Token::new(TokenType::IDENTIFIER, &ident_raw, self.line),
                                }
                            }
                           _ => Token::new(TokenType::IDENTIFIER, &ident_raw, self.line),

                    }
                }
                        
                else {
                    let mut err_tok_raw = "Invalid token".to_string();
                    err_tok_raw.push(self.current);
                    return Token::new(TokenType::ERROR, &err_tok_raw, self.line);
                }

            }

        }
    }

    pub fn is_alpha(c: char) -> bool {
       return (c >= 'a' && c <= 'z') ||
                (c >= 'A' && c <= 'Z') ||
                (c == '_');
    }

    pub fn eof(&mut self) -> bool{
        if self.current == '\0' {
            return true;
        }
        return false;
    }

    pub fn advance(&mut self) {
        //println!("{:?}", self.source.peek());
        if self.source.peek() != '\0' {
            self.current = self.source.advance();
        }
        else {
            self.current = '\0';
        }
    }
}
