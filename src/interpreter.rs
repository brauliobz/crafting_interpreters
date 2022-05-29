use std::{cell::RefCell, io::Write, rc::Rc};

use crate::{
    ast::{Expr, FunctionDecl, IfStatement, LiteralExpr, Statement, WhileStatement},
    environment::{Env, Environment, Function, Value},
    error::{ice, runtime_error, ErrorOrEarlyReturn, RuntimeError, ICE},
    scanner::TokenType,
    Result,
};

pub struct Interpreter<'stdout> {
    stack: Vec<Env>,
    current_env: Env,
    global_env: Env,
    stdout: &'stdout mut dyn Write,
}

impl<'output> Interpreter<'output> {
    pub fn new(stdout: &'output mut dyn Write) -> Self {
        let global_env = Rc::new(RefCell::new(Environment::new(None)));

        Interpreter {
            current_env: global_env.clone(),
            stack: vec![global_env.clone()],
            global_env,
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
            Statement::While(while_statement) => self.while_stmt(while_statement),
            Statement::FunDecl(function) => self.declare_fun(function),
            Statement::Return(expr) => self.return_stmt(expr.as_ref()),
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
            Expr::Call(call) => self.call_fun(&call.callee, &call.args),
        }
    }

    pub fn print_stmt(&mut self, expr: &Expr) -> Result<Value> {
        let value = self.calc_expr(expr)?;
        let output = format!("{}\n", value);
        self.stdout.write_all(output.as_bytes())?;
        Ok(Value::Nil)
    }

    fn push_new_env(&mut self, parent: Option<Env>) {
        let new = Rc::new(RefCell::new(Environment::new(
            parent.or_else(|| Some(self.current_env.clone())),
        )));
        self.current_env = new.clone();
        self.stack.push(new);
    }

    fn pop_env(&mut self) -> Result<()> {
        self.stack.pop();
        self.current_env = self
            .stack
            .iter()
            .last()
            .cloned()
            .ok_or_else(|| ice(ICE::Generic("Stack underflow".into())))?;
        Ok(())
    }

    fn calc_identifier(&mut self, id: &str) -> Result<Value> {
        self.current_env
            .borrow()
            .get(id)
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

        self.current_env.borrow_mut().define(name, value);

        Ok(Value::Nil)
    }

    fn calc_assignment(&mut self, var_name: &str, rvalue: &Expr) -> Result<Value> {
        let value = self.calc_expr(rvalue)?;
        self.current_env
            .borrow_mut()
            .assign(var_name, value.clone())?;
        Ok(value)
    }

    fn exec_block(&mut self, statements: &[Statement]) -> Result<Value> {
        self.push_new_env(None);

        let result = self.exec_statements(statements);

        self.pop_env()?;

        result
    }

    fn exec_statements(&mut self, statements: &[Statement]) -> Result<Value> {
        for stmt in statements {
            self.exec_stmt(stmt)?;
        }
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

    fn while_stmt(&mut self, while_statement: &WhileStatement) -> Result<Value> {
        let mut cond_value = self.calc_expr(&while_statement.cond)?;

        while is_truthy(&cond_value) {
            self.exec_stmt(&while_statement.stmt)?;
            cond_value = self.calc_expr(&while_statement.cond)?;
        }

        Ok(Value::Nil)
    }

    fn declare_fun(&mut self, function: &Rc<FunctionDecl>) -> Result<Value> {
        self.current_env.borrow_mut().define(
            &function.name,
            Value::Function(Function {
                ast: function.clone(),
            }),
        );

        Ok(Value::Nil)
    }

    fn return_stmt(&mut self, expr: Option<&Expr>) -> Result<Value> {
        let value = expr.map_or(Ok(Value::Nil), |expr: &Expr| self.calc_expr(expr))?;

        Err(ErrorOrEarlyReturn::EarlyReturn(value))
    }

    fn call_fun(&mut self, callee: &Expr, args: &Vec<Expr>) -> Result<Value> {
        // find function

        let fun = match self.calc_expr(callee)? {
            Value::Function(fun) => fun,
            _ => return Err(runtime_error(RuntimeError::UndefinedFunction("".into()))),
        };

        if args.len() != fun.ast.params.len() {
            return Err(runtime_error(RuntimeError::NumberOfArgumentsMismatch(
                fun.ast.params.len(),
                fun.ast.name.clone(),
                args.len(),
            )));
        }

        // compute args and push them to a new stack frame

        let computed_args = args
            .iter()
            .map(|expr| self.calc_expr(expr))
            .collect::<Result<Vec<Value>>>()?;

        self.push_new_env(Some(self.global_env.clone()));

        for (value, name) in computed_args.iter().zip(&fun.ast.params) {
            self.current_env.borrow_mut().define(name, value.clone());
        }

        // execute function

        let result = self.exec_statements(&fun.ast.body);
        self.pop_env()?;

        match result {
            Ok(value) => Ok(value),
            Err(ErrorOrEarlyReturn::EarlyReturn(value)) => Ok(value),
            Err(err) => Err(err),
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

fn is_truthy(value: &Value) -> bool {
    !matches!(value, Value::Boolean(false) | Value::Nil)
}
