use std::ops::*;
use std::collections::HashMap;
use crate::bytecode::{OpCode, Chunk};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum ValueType {
    auto,
    bool(bool),
    int(i32),
    float(f32),
    char(char),
    str(String),
    cmd(String),
    object,
    func,
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

    pub fn is_number(vt: ValueType) -> bool {
        match vt {
            ValueType::int(_) | ValueType::float(_) => true,
            _ => false,
        }
    }

    pub fn extract_str(vt: ValueType) -> Option<String> {
        match vt {
            ValueType::str(s) => Some(s),
            ValueType::cmd(c) => Some(c),
            _ => None,
        }
    }

    pub fn infer_type(value_source: String) {
    }

}

impl Add for ValueType {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match self {
            ValueType::bool(_) => {
                eprintln!("Cannot perform addition on boolean");
                ValueType::null
            },
            ValueType::int(li) => {
                match other {
                    ValueType::int(ri) => ValueType::int(li+ri),
                    ValueType::float(rf) => ValueType::float(li as f32 +rf),
                    _ => {
                        eprintln!("Mismatched types");
                        ValueType::null
                    },
                }
            }

            ValueType::float(lf) => {
                match other {
                    ValueType::int(ri) => ValueType::float(lf + ri as f32),
                    ValueType::float(rf) => ValueType::float(lf + rf),
                    _ => {
                        eprintln!("Mismatched types");
                        ValueType::null
                    },
                }
            }
            ValueType::char(_) => {
                eprintln!("cannot perform addition on char");
                ValueType::null
            }
            _ => ValueType::null
        }
    }

}
impl Sub for ValueType {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match self {
            ValueType::bool(_) => {
                eprintln!("Cannot perform subtraction on boolean");
                ValueType::null
            },
            ValueType::int(li) => {
                match other {
                    ValueType::int(ri) => ValueType::int(li-ri),
                    ValueType::float(rf) => ValueType::float(li as f32 -rf),
                    _ => {
                        eprintln!("Mismatched types");
                        ValueType::null
                    },
                }
            }

            ValueType::float(lf) => {
                match other {
                    ValueType::int(ri) => ValueType::float(lf - ri as f32),
                    ValueType::float(rf) => ValueType::float(lf - rf),
                    _ => {
                        eprintln!("Mismatched types");
                        ValueType::null
                    },
                }
            }
            ValueType::char(_) => {
                eprintln!("cannot perform subtraction on char");
                ValueType::null
            }
            _ => ValueType::null
        }
    }

}
impl Mul for ValueType {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match self {
            ValueType::bool(_) => {
                eprintln!("Cannot perform multiplication on boolean");
                ValueType::null
            },
            ValueType::int(li) => {
                match other {
                    ValueType::int(ri) => ValueType::int(li*ri),
                    ValueType::float(rf) => ValueType::float(li as f32 *rf),
                    _ => {
                        eprintln!("Mismatched types");
                        ValueType::null
                    },
                }
            }

            ValueType::float(lf) => {
                match other {
                    ValueType::int(ri) => ValueType::float(lf * ri as f32),
                    ValueType::float(rf) => ValueType::float(lf * rf),
                    _ => {
                        eprintln!("Mismatched types");
                        ValueType::null
                    },
                }
            }
            ValueType::char(_) => {
                eprintln!("cannot perform multiplication on char");
                ValueType::null
            }
            _ => ValueType::null
        }
    }

}
impl Div for ValueType {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        match self {
            ValueType::bool(_) => {
                eprintln!("Cannot perform division on boolean");
                ValueType::null
            },
            ValueType::int(li) => {
                match other {
                    ValueType::int(ri) => ValueType::int(li/ri),
                    ValueType::float(rf) => ValueType::float(li as f32 /rf),
                    _ => {
                        eprintln!("Mismatched types");
                        ValueType::null
                    },
                }
            }

            ValueType::float(lf) => {
                match other {
                    ValueType::int(ri) => ValueType::float(lf / ri as f32),
                    ValueType::float(rf) => ValueType::float(lf / rf),
                    _ => {
                        eprintln!("Mismatched types");
                        ValueType::null
                    },
                }
            }
            ValueType::char(_) => {
                eprintln!("cannot perform division on char");
                ValueType::null
            }
            _ => ValueType::null
        }
    }
}

impl std::fmt::Display for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueType::bool(b) => {
                write!(f, "Value: {}", b)
            },
            ValueType::int(i) => {
                write!(f, "Value: {}", i)
            },
            ValueType::float(fl) => {
                write!(f, "Value: {}", fl)
            },
            ValueType::char(c) => {
                write!(f, "Value: {}", c)
            },
            ValueType::str(s) => {
                write!(f, "Value: {}", s)
            }
            _ => write!(f, "Huh? how did this happen? type not matched")
        }
    }
}


#[derive(Debug)]
pub struct Function {
    pub params: Vec<String>,
    pub ops: Vec<OpCode>,
    pub vals: Vec<ValueType>,
    pub chk: Chunk,
    pub scope: i32,
    pub location: usize,
}

impl Function {
    pub fn new() -> Self {
        Self {
            params: Vec::new(),
            ops: Vec::new(),
            vals: Vec::new(),
            chk: Chunk::new(),
            location: 0,
            scope: 0,
        }
    }
}

#[derive(Debug)]
pub struct Object {

}


pub struct Int {

}


pub struct Float {

}

pub struct Bool {

}

pub struct Char {

}
