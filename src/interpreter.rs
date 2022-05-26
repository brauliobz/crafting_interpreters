use std::io::Write;

use crate::{
    ast::{Expr, IfStatement, LiteralExpr, Statement},
    environment::{Environment, Value},
    error::{ice, runtime_error, RuntimeError, ICE},
    scanner::TokenType,
    Result,
};

pub struct Interpreter<'stdout> {
    envs: Vec<Environment>,
    stdout: &'stdout mut dyn Write,
}

impl<'output> Interpreter<'output> {
    pub fn new(stdout: &'output mut dyn Write) -> Self {
        Interpreter {
            envs: vec![Environment::new()],
            stdout,
        }
    }

    pub fn exec_stmt(&mut self, stmt: &Statement) -> Result<Value> {
        match stmt {
            Statement::Expr(expr) => self.calc_expr(expr),
            Statement::Print(expr) => self.print_stmt(expr),
            Statement::VariableDecl(name, value) => self.var_decl(name, value),
            Statement::Block(statements) => self.exec_block(statements),
            Statement::If(if_statement) => self.if_stmt(if_statement),
        }
    }

    pub fn calc_expr(&mut self, expr: &Expr) -> Result<Value> {
        match expr {
            Expr::Literal(lit) => Ok(calc_lit(lit)),
            Expr::Identifier(id) => self.calc_identifier(id),
            Expr::Unary(unary) => self.calc_unary(unary.op, unary.expr.as_ref()),
            Expr::Binary(bin) => self.calc_binary(bin.left.as_ref(), bin.op, bin.right.as_ref()),
            Expr::Grouping(expr) => self.calc_expr(expr.as_ref()),
            Expr::Assignment(var_name, rvalue) => self.calc_assignment(var_name, rvalue),
        }
    }

    pub fn print_stmt(&mut self, expr: &Expr) -> Result<Value> {
        let value = self.calc_expr(expr)?;
        let output = format!("{}\n", value);
        self.stdout.write_all(output.as_bytes())?;
        Ok(Value::Nil)
    }

    fn get_curr_env_mut(&mut self) -> &mut Environment {
        self.envs.last_mut().expect("Stack underflow")
    }

    fn push_new_env(&mut self) {
        self.envs.push(Environment::new());
    }

    fn pop_env(&mut self) {
        self.envs.pop();
    }

    fn calc_identifier(&mut self, id: &str) -> Result<Value> {
        self.envs
            .iter()
            .rev()
            .find_map(|env| env.get(id))
            .cloned()
            .ok_or_else(|| runtime_error(RuntimeError::UndefinedVariable(id.into())))
    }

    fn calc_unary(&mut self, op: TokenType, expr: &Expr) -> Result<Value> {
        match op {
            TokenType::Minus => match self.calc_expr(expr)? {
                Value::Number(n) => Ok(Value::Number(-n)),
                other => Err(runtime_error(RuntimeError::TypeMismatch(
                    "number".into(),
                    format!("{}", other),
                ))),
            },
            TokenType::Bang => match self.calc_expr(expr)? {
                Value::Boolean(b) => Ok(Value::Boolean(!b)),
                other => Err(runtime_error(RuntimeError::TypeMismatch(
                    "boolean".into(),
                    format!("{}", other),
                ))),
            },
            _ => Err(ice(ICE::Generic(format!(
                "Invalid unary operator '{:?}'",
                op
            )))),
        }
    }

    fn calc_binary(&mut self, left: &Expr, op: TokenType, right: &Expr) -> Result<Value> {
        use TokenType::*;
        use Value::*;

        let left = self.calc_expr(left)?;

        if op == And {
            Ok(Boolean(
                is_truthy(&left) && is_truthy(&self.calc_expr(right)?),
            ))
        } else if op == Or {
            Ok(Boolean(
                is_truthy(&left) || is_truthy(&self.calc_expr(right)?),
            ))
        } else {
            let right = self.calc_expr(right)?;
            match (left, op, right) {
                // numbers
                (Number(l), Plus, Number(r)) => Ok(Number(l + r)),
                (Number(l), Minus, Number(r)) => Ok(Number(l - r)),
                (Number(l), Star, Number(r)) => Ok(Number(l * r)),
                (Number(_), Slash, Number(r)) if r == 0.0 => {
                    Err(runtime_error(RuntimeError::DivisionByZero))
                }
                (Number(l), Slash, Number(r)) => Ok(Number(l / r)),

                // comparisons
                (l, EqualEqual, r) => Ok(Boolean(l == r)),
                (l, BangEqual, r) => Ok(Boolean(l != r)),
                (Number(l), Greater, Number(r)) => Ok(Boolean(l > r)),
                (Number(l), GreaterEqual, Number(r)) => Ok(Boolean(l >= r)),
                (Number(l), Less, Number(r)) => Ok(Boolean(l < r)),
                (Number(l), LessEqual, Number(r)) => Ok(Boolean(l <= r)),

                // strings
                (Value::String(l), Plus, Value::String(r)) => Ok(Value::String(l + &r)),

                (left, op, right) => Err(runtime_error(RuntimeError::InvalidOperator(
                    op,
                    format!("{}", left),
                    format!("{}", right),
                ))),
            }
        }
    }

    fn var_decl(&mut self, name: &str, expr: &Option<Expr>) -> Result<Value> {
        let value = expr
            .as_ref()
            .map_or(Ok(Value::Nil), |expr| self.calc_expr(expr))?;

        self.get_curr_env_mut().define(name, value);

        Ok(Value::Nil)
    }

    fn calc_assignment(&mut self, var_name: &str, rvalue: &Expr) -> Result<Value> {
        let value = self.calc_expr(rvalue)?;

        self.assign(var_name, value.clone())?;

        Ok(value)
    }

    fn assign(&mut self, name: &str, new_value: Value) -> Result<()> {
        let old_value = self.envs.iter_mut().rev().find_map(|env| env.get_mut(name));

        match old_value {
            Some(value) => *value = new_value,
            None => return Err(runtime_error(RuntimeError::UndefinedVariable(name.into()))),
        }

        Ok(())
    }

    fn exec_block(&mut self, statements: &[Statement]) -> Result<Value> {
        self.push_new_env();

        for stmt in statements {
            self.exec_stmt(stmt)?;
        }

        self.pop_env();

        Ok(Value::Nil)
    }

    fn if_stmt(&mut self, if_statement: &IfStatement) -> Result<Value> {
        let cond_value = self.calc_expr(&if_statement.cond)?;

        if is_truthy(&cond_value) {
            self.exec_stmt(&if_statement.then_branch)?;
        } else if let Some(else_branch) = &if_statement.else_branch {
            self.exec_stmt(else_branch)?;
        }

        Ok(Value::Nil)
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

fn is_truthy(value: &Value) -> bool {
    !matches!(value, Value::Boolean(false) | Value::Nil)
}
