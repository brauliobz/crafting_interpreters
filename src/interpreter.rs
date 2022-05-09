use std::io::Write;

use crate::{
    ast::{Expr, LiteralExpr, Statement},
    memory::{Memory, Value},
    scanner::TokenType,
};

pub struct Interpreter<'stdout> {
    memory: Memory,
    stdout: &'stdout mut dyn Write,
}

impl <'output> Interpreter<'output> {
    pub fn new(stdout: &'output mut dyn Write) -> Self {
        Interpreter {
            memory: Memory::new(),
            stdout,
        }
    }

    pub fn exec_stmt(&mut self, stmt: &Statement) -> Value {
        match stmt {
            Statement::Expr(expr) => self.calc_expr(expr),
            Statement::Print(expr) => self.print_stmt(expr),
            Statement::VariableDecl(name, value) => todo!(),
        }
    }

    pub fn calc_expr(&self, expr: &Expr) -> Value {
        match expr {
            Expr::Literal(lit) => calc_lit(lit),
            Expr::Identifier(id) => self.calc_identifier(id),
            Expr::Unary(unary) => self.calc_unary(unary.op, unary.expr.as_ref()),
            Expr::Binary(bin) => self.calc_binary(bin.left.as_ref(), bin.op, bin.right.as_ref()),
            Expr::Grouping(expr) => self.calc_expr(expr.as_ref()),
        }
    }

    pub fn print_stmt(&mut self, expr: &Expr) -> Value {
        let value = self.calc_expr(expr);
        let output = format!("{}", value);
        self.stdout.write_all(output.as_bytes()).expect("I/O error"); // TODO deal with it
        Value::Nil
    }

    fn calc_identifier(&self, id: &str) -> Value {
        if let Some(value) = self.memory.values.get(id) {
            value.clone()
        } else {
            Value::Nil
        }
    }

    fn calc_unary(&self, op: TokenType, expr: &Expr) -> Value {
        match op {
            TokenType::Minus => match self.calc_expr(expr) {
                Value::Number(n) => Value::Number(-n),
                other => panic!("Expected number for unary minus, but got {:?}", other), // TODO do not panic
            },
            TokenType::Bang => match self.calc_expr(expr) {
                Value::Boolean(b) => Value::Boolean(!b),
                other => panic!("Expected boolean for unary not, but got {:?}", other), // TODO do not panic
            },
            _ => panic!("Invalid unary operator {:?}", op), // TODO do not panic
        }
    }

    fn calc_binary(&self, left: &Expr, op: TokenType, right: &Expr) -> Value {
        use TokenType::*;
        use Value::*;

        let left = self.calc_expr(left);
        let right = self.calc_expr(right);

        match (left, op, right) {
            // booleans
            (Boolean(l), And, Boolean(r)) => Boolean(l && r), // TODO short circuit?
            (Boolean(l), Or, Boolean(r)) => Boolean(l || r),  // TODO short circuit?

            // numbers
            (Number(l), Plus, Number(r)) => Number(l + r),
            (Number(l), Minus, Number(r)) => Number(l - r),
            (Number(l), Star, Number(r)) => Number(l * r),
            (Number(l), Slash, Number(r)) => Number(l / r), // TODO division by zero?

            // comparisons
            (l, EqualEqual, r) => Boolean(l == r),
            (l, BangEqual, r) => Boolean(l != r),
            (Number(l), Greater, Number(r)) => Boolean(l > r),
            (Number(l), GreaterEqual, Number(r)) => Boolean(l >= r),
            (Number(l), Less, Number(r)) => Boolean(l < r),
            (Number(l), LessEqual, Number(r)) => Boolean(l <= r),

            // strings
            (left, op, right) => panic!(
                "Invalid operator application: {:?} over values {:?} and {:?}",
                op, left, right
            ), // TODO do not panic
        }
    }
}

fn calc_lit(lit: &LiteralExpr) -> Value {
    use LiteralExpr::*;
    match lit {
        Boolean(b) => Value::Boolean(*b),
        Number(n) => Value::Number(*n),
        String(s) => Value::String(s.clone()),
        Nil => Value::Nil,
    }
}
