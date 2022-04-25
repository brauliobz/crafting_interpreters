use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
    Object
}

pub struct Memory {
    pub values: HashMap<String, Value>,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            values: HashMap::new(),
        }
    }
}