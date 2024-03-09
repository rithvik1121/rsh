#[derive(Debug, Clone)]
pub enum OpCode {
    RETURN,
    CONSTANT(usize),
    NEGATE,

    ADD,
    SUBTRACT,

    MULT,
    DIV,


    COMMAND,
    ARGUMENT,

    EOF,
}




#[derive(Debug,Clone, Copy)]
pub enum ValueType {
    auto,
    bool(bool),
    int(i32),
    float(f32),
    char(char),
    null,
}

impl ValueType {
    pub fn extract_int(vt: ValueType) -> Option<i32> {
        match vt {
            ValueType::int(i) => Some(i),
            _ => None,
        }

    }
    pub fn extract_float(vt: ValueType) -> Option<f32> {
        match vt {
            ValueType::float(f) => Some(f),
            _ => None,
        }
    }
    pub fn extract_bool(vt: ValueType) -> Option<bool> {
        match vt {
            ValueType::bool(b) => Some(b),
            _ => None,
        }
    }
    pub fn extract_char(vt: ValueType) -> Option<char> {
        match vt {
            ValueType::char(c) => Some(c),
            _ => None,
        }
    }

    pub fn parse_type(){}

}

#[derive(Debug)]
pub struct Chunk {
    pub ops: Vec<OpCode>,
    pub vals: Vec<ValueType>,
    pub lines: Vec<i32>,
}


impl Chunk {

    pub fn new() -> Self {
        Self {
            ops: Vec::new(),
            vals: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn disassemble_chunk(&mut self) {
        //println!("== {:?} ==", codes);
        for mut index in 0..self.ops.len() {
            println!("{}, {}", index, self.ops.len());
            index = self.disassemble_instruction(&index);
        }
    }

    pub fn disassemble_instruction(&mut self, index: &usize) -> usize {
        let code = &self.ops[*index];
        match code {

            OpCode::CONSTANT(s) => self.simple_instruct(code, index), 
            OpCode::NEGATE => self.simple_instruct(code, index),
            OpCode::RETURN => self.simple_instruct(code, index),

            
            _ => {
                println!("Error: Unknown opcode {:?}, line: {}", code, self.lines[*index]);
                panic!();
            },
        }
    }


    pub fn simple_instruct(&self, code: &OpCode, index: &usize) -> usize {
        println!("code: {:?}, line: {}", code, self.lines[*index]);
        return *index;
    }

    pub fn constant_instruction(&self, code: &OpCode, vals: &[ValueType], index: &usize) -> usize {
        println!("code: {:?}, value: {:?}, line: {}", code, vals[*index], self.lines[*index]);
        let offset = *index + 1;
        return offset;
        //return *index;
    }
    
    pub fn push_code(&mut self, code: OpCode, line: i32) {
        self.ops.push(code);
        self.lines.push(line);
    }

    pub fn push_value(&mut self, val: ValueType) -> usize {
        self.vals.push(val);
        return self.vals.len()-1 as usize;
    }

    pub fn add_constant(&mut self, val: ValueType, line: i32) {
        let ind = self.push_value(val);
        self.push_code(OpCode::CONSTANT(ind), line);
    }


    pub fn extract_val(value: &ValueType) {
        println!("bruh");
    }

}
