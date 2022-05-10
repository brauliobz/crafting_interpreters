use std::{collections::HashMap, fmt::Display};

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
    Object,
}

pub struct Environment {
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: &str, value: Value) {
        self.values.insert(name.into(), value);
    }

    pub fn get(&self, name: &str) -> &Value {
        self.values
            .get(name)
            .unwrap_or_else(|| panic!("Undefined variable '{}'.", name))
    }

    pub fn assign(&mut self, name: &str, new_value: Value) {
        match self.values.get_mut(name) {
            Some(value) => *value = new_value,
            None => panic!("Undefined variable '{}'.", name),
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
