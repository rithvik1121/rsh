use crate::tokens::TokenType;
use crate::tokens::Token;
use crate::source::Source;

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

    pub fn lex(&mut self) -> Option<Token> {
        match self.current {
            ' ' | '\t' | '\r' => {
                self.advance();
                return None;
            }
            '\n' => {
                self.line+=1; self.advance();
                return None;
            },
            _ => (),
        };
        let token = match self.current {
            '(' => Token::new(TokenType::LEFT_PAREN, "(", self.line),
            ')' => Token::new(TokenType::RIGHT_PAREN, ")", self.line),
            '{' => Token::new(TokenType::LEFT_BRACE, "{", self.line),
            '}' => Token::new(TokenType::RIGHT_BRACE, "}", self.line),
            ';' => Token::new(TokenType::SEMICOLON, ";", self.line),
            ',' => Token::new(TokenType::COMMA, ",", self.line),
            '+' => Token::new(TokenType::PLUS, "+", self.line),
            '-' => Token::new(TokenType::MINUS, "-", self.line),
            '*' => Token::new(TokenType::MULT, "*", self.line),
            '/' => Token::new(TokenType::DIV, "/", self.line), 
            '&' => {
                match self.source.peek() {
                    '&' => {
                        self.advance();
                        Token::new(TokenType::AND, "&&", self.line)
                    }
                    _ => Token::new(TokenType::IDENTIFIER, "&", self.line),
                }
            },
            '.' => {
                match self.source.peek() {
                    '.' => {
                        self.advance();
                        Token::new(TokenType::IDENTIFIER, "..", self.line)
                    }
                    _ => Token::new(TokenType::DOT, ".", self.line),
                }
            },
            ':' => {
                match self.source.peek() {
                    '=' => {
                        self.advance();
                        Token::new(TokenType::INITIALIZE, ":=", self.line)
                    }
                    _ => Token::new(TokenType::ANNOTATION, ":", self.line),
                }
            },
            '|' => {
                match self.source.peek() {
                    '|' => {
                        self.advance();
                        Token::new(TokenType::OR, "||", self.line)
                    }
                    _ => Token::new(TokenType::PIPE, "|", self.line),
                }
            },
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
                    '<' => {
                        self.advance();
                        Token::new(TokenType::INPUT, "<<", self.line)
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
                    '>' => {
                        self.advance();
                        Token::new(TokenType::OUTPUT, ">>", self.line)
                    },
                    _ => Token::new(TokenType::GREATER, ">", self.line),
                }
            }
            '"' => {
                let mut str_raw = "".to_string();
                self.advance();
                while (!self.eof()) && (self.current != '"') {
                    if self.current == '\n' {
                        self.line += 1;
                    }
                    str_raw.push(self.current);
                    self.advance();
                }
                if self.eof() {
                    return Some(Token::new(TokenType::ERROR, "Unterminated string", self.line));
                }
                let str_tok = Token::new(TokenType::STRING, &str_raw, self.line);
                str_tok
            }
            
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                let mut num_raw = "".to_string();
                while (self.current <= '9') && (self.current >= '0') || self.current == '.' {
                    num_raw.push(self.current);
                    if (self.source.peek() <= '9') && (self.source.peek() >= '0') || self.source.peek() == '.'{
                        self.advance();
                    }
                    else {
                        break;
                    }
                }

                let num_tok = Token::new(TokenType::NUMBER, &num_raw.trim(), self.line);
                num_tok
            }

            '\0' => Token::new(TokenType::EOF, "", self.line),
            
            _ => {
                if Self::is_alpha(self.current) {
                    let mut ident_raw = "".to_string();
                loop {
                        ident_raw.push(self.current);
                        if Self::is_alpha(self.source.peek()) {
                            self.advance();
                        }
                        else {
                            break;
                        }
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
                                "if" => Token::new(TokenType::IF, "if", self.line),
                                _ => Token::new(TokenType::IDENTIFIER, &ident_raw, self.line),
                            }
                        }
                        'e' => {
                            match ident_raw.as_str() {
                                "else" => Token::new(TokenType::ELSE, "else", self.line),
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
                                "write" => Token::new(TokenType::WRITE, "write", self.line),
                                _ => Token::new(TokenType::IDENTIFIER, &ident_raw, self.line),
                            }
                                
                        }
                        'l' => {
                            match ident_raw.as_str() {
                                "let" => Token::new(TokenType::LET, "let", self.line),
                                "loop" => Token::new(TokenType::LOOP, "loop", self.line),
                                _ => Token::new(TokenType::IDENTIFIER, &ident_raw, self.line),
                            }
                        }
                        'n' => {
                            match ident_raw.as_str() {
                                "null" => Token::new(TokenType::NULL, "null", self.line),
                                _ => Token::new(TokenType::IDENTIFIER, &ident_raw, self.line),
                            }
                        }
                       _ => Token::new(TokenType::IDENTIFIER, &ident_raw, self.line),

                    }
                }
                else {
                    let mut err_tok_raw = "(Invalid token)-> ".to_string();
                    err_tok_raw.push(self.current);
                    Token::new(TokenType::ERROR, &err_tok_raw, self.line)
                }
            }
        };
        return Some(token);
    }

    pub fn is_alpha(c: char) -> bool {
       (c >= 'a' && c <= 'z') ||
                (c >= 'A' && c <= 'Z') ||
                (c == '_') ||
                Self::is_num(c)
    }
    pub fn is_num(c: char) -> bool {
        c >= '0' && c <= '9'
    }

    pub fn eof(&mut self) -> bool{
        if self.current == '\0' {
            return true;
        }
        false
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

    pub fn lex_command(&mut self) {

    }
}
