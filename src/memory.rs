use std::{collections::HashMap, fmt::Display};

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

impl Display for Value {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Nil => write!(fmt, "Nil"),
            Value::Boolean(b) => write!(fmt, "{}", b),
            Value::Number(n) => write!(fmt, "{}", n),
            Value::String(s) => write!(fmt, "{}", s),
            Value::Object => todo!(),
        }
    }
}