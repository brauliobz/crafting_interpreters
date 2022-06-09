use std::{cell::RefCell, collections::HashMap, fmt::Display, rc::Rc};

use crate::{
    ast::FunctionDecl,
    error::{runtime_error, RuntimeError},
    Result,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
    Object,
    Function(Function),
    NativeFunction(NativeFunction),
}

#[derive(Debug, Clone)]
pub struct Function {
    pub ast: Rc<FunctionDecl>,
    pub closure: Env,
}

#[derive(Debug, PartialEq, Clone)]
pub struct NativeFunction {
    pub name: &'static str,
    pub exec: fn(Env) -> Result<Value>,
}

pub type Env = Rc<RefCell<Environment>>;

#[derive(Debug)]
pub struct Environment {
    values: HashMap<String, Value>,
    pub parent: Option<Env>,
}

impl Environment {
    pub fn new(parent: Option<Env>) -> Self {
        Environment {
            values: HashMap::new(),
            parent,
        }
    }

    pub fn define(&mut self, name: &str, value: Value) {
        self.values.insert(name.into(), value);
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        match self.values.get(name).cloned() {
            Some(value) => Some(value),
            None => self.parent.as_deref()?.borrow().get(name),
        }
    }

    pub fn assign(&mut self, name: &str, value: Value) -> Result<()> {
        match self.values.get_mut(name) {
            Some(dest) => *dest = value,
            None => match self.parent {
                Some(ref parent) => parent.borrow_mut().assign(name, value)?,
                None => return Err(runtime_error(RuntimeError::UndefinedVariable(name.into()))),
            },
        }
        Ok(())
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
            Value::Function(function) => write!(fmt, "fun {}", &function.ast.name),
            Value::NativeFunction(fun) => write!(fmt, "native fun {}", fun.name),
        }
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.ast == other.ast && self.closure.as_ptr() == other.closure.as_ptr()
    }
}
