#[derive(Debug)]
pub struct Source {
    text: Vec<u8>,
    current: usize,
}


impl Source {
    pub fn new(raw_text: &str) -> Self {
        Self {
            text: raw_text.as_bytes().to_vec(),
            current: 0
        }
    }

    pub fn current(&self) -> char {
        char::from(self.text[self.current])
    }

    pub fn advance(&mut self) -> char {
        if self.peek() != '\0' {
            self.current+=1;
            return char::from(self.text[self.current]);
        }
        self.current+=1;
        return '\0';
    }
    pub fn peek(&self) -> char {
        if self.current == self.text.len()-1 {
//            println!("eof");
            return '\0';
        }
        char::from(self.text[self.current+1])
    }
}
