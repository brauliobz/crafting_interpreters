use std::{borrow::BorrowMut, collections::HashMap, rc::Rc};

use crate::ast::*;

pub struct Resolver {
    scopes: Vec<Scope>,
}

type Scope = HashMap<String, bool>;

impl Resolver {
    pub fn resolve(&mut self, ast: &mut Program) {
        for stmt in &mut ast.statements {
            self.resolve_stmt(stmt);
        }
    }

    fn new_scope(&mut self) {
        self.scopes.push(HashMap::new())
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn register_identifier(&mut self, name: &str) {
        if let Some(scope) = self.scopes.last_mut() {
            // TODO check if it already exists
            scope.insert(name.to_owned(), false);
        }
    }

    fn initialize_identifier(&mut self, name: &str) {
        todo!();
    }

    fn find_identifier(&self, name: &str) -> (&Scope, ResolvingDepth) {
        todo!()
    }

    fn resolve_identifier(&mut self, expr: &mut Expr) {
        if let Some(scope) = self.scopes.last() {
            // TODO find the depth
            // TODO Save in the AST
        }
    }

    fn resolve_stmt(&mut self, stmt: &mut Statement) {
        match stmt {
            Statement::Expr(expr) => self.resolve_expr(expr),
            Statement::VariableDecl(name, maybe_expr) => self.resolve_var_decl(name, maybe_expr),
            Statement::Block(stmts) => self.resolve_block(stmts),
            Statement::If(if_stmt) => self.resolve_if(if_stmt),
            Statement::While(while_stmt) => self.resolve_while(while_stmt),
            Statement::FunDecl(fun_decl) => {
                self.resolve_fun_decl(Rc::get_mut(fun_decl).unwrap_or_else(|| unreachable!()))
            }
            Statement::Return(maybe_expr) => self.resolve_return(maybe_expr),
            _ => (),
        }
    }

    fn resolve_var_decl(&mut self, name: &str, maybe_expr: &mut Option<Expr>) {
        todo!()
    }

    fn resolve_if(&mut self, if_stmt: &mut IfStatement) {
        todo!()
    }

    fn resolve_block(&mut self, stmts: &mut Vec<Statement>) {
        self.new_scope();

        todo!();

        self.end_scope();
    }

    fn resolve_while(&mut self, while_stmt: &mut WhileStatement) {
        todo!();
    }

    fn resolve_fun_decl(&mut self, fun_decl: &mut FunctionDecl) {
        todo!();
    }

    fn resolve_return(&mut self, maybe_expr: &mut Option<Expr>) {
        todo!();
    }

    fn resolve_expr(&mut self, expr: &mut Expr) {
        match expr {
            Expr::Identifier(_, _) => self.resolve_identifier(expr),
            Expr::Literal(_) | Expr::Unary(_) => (),
            Expr::Binary(BinaryExpr { left, op: _, right }) => {
                self.resolve_expr(left);
                self.resolve_expr(right);
            }
            Expr::Grouping(expr) => self.resolve_expr(expr),
            Expr::Assignment(var_name, expr) => self.resolve_assignment(var_name, expr),
            Expr::Call(call_expr) => self.resolve_call(call_expr),
        }
    }

    fn resolve_assignment(&mut self, var_name: &str, expr: &mut Expr) {
        todo!();
    }

    fn resolve_call(&mut self, call_expr: &mut CallExpr) {
        todo!();
    }
}
