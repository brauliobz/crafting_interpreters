use std::{collections::HashMap, rc::Rc};

use crate::ast::*;

#[derive(Default)]
pub struct Resolver {
    scopes: Vec<Scope>,
}

type Scope = HashMap<String, bool>;

impl Resolver {
    pub fn new() -> Resolver {
        Resolver::default()
    }

    pub fn resolve(&mut self, ast: &mut Program) {
        self.new_scope();

        self.register_identifier("clock");
        self.initialize_identifier("clock", 0);

        for stmt in &mut ast.statements {
            self.resolve_stmt(stmt);
        }
        self.end_scope();
    }

    fn new_scope(&mut self) {
        println!("Push scope");
        self.scopes.push(HashMap::new())
    }

    fn end_scope(&mut self) {
        println!("Pop scope");
        self.scopes.pop();
    }

    fn register_identifier(&mut self, name: &str) {
        println!("Register {} at depth {}", name, self.scopes.len() - 1);
        if let Some(scope) = self.scopes.last_mut() {
            // TODO check if it already exists
            scope.insert(name.to_owned(), false);
        }
    }

    fn initialize_identifier(&mut self, name: &str, depth: u8) {
        self.scopes
            .iter_mut()
            .rev()
            .nth(depth as usize)
            .map(|s| s.insert(name.to_owned(), true));
    }

    fn find_resolving_depth(&self, name: &str) -> Option<u8> {
        for (depth, scope) in self.scopes.iter().rev().enumerate() {
            if scope.contains_key(name) {
                return Some(depth as u8);
            }
        }
        None
    }

    fn resolve_identifier(&mut self, identifier_expr: &mut Expr) {
        if let Expr::Identifier(name, resolving_depth) = identifier_expr {
            println!(
                "Resolved {} with depth {:?} [IDENTIFIER]",
                name,
                self.find_resolving_depth(name)
            );
            *resolving_depth = self.find_resolving_depth(name);
        }
    }

    fn resolve_stmt(&mut self, stmt: &mut Statement) {
        println!("Resolve stmt {:?}", stmt);
        println!("Scopes: {:?}", self.scopes);
        match stmt {
            Statement::Expr(expr) => self.resolve_expr(expr),
            Statement::VariableDecl(name, maybe_expr) => self.resolve_var_decl(name, maybe_expr),
            Statement::Block(stmts) => self.resolve_block(stmts),
            Statement::If(if_stmt) => self.resolve_if(if_stmt),
            Statement::While(while_stmt) => self.resolve_while(while_stmt),
            Statement::FunDecl(fun_decl) => {
                // TODO generate ICE
                self.resolve_fun_decl(
                    Rc::get_mut(fun_decl).expect("Could not get function declaration as mut"),
                )
            }
            Statement::Return(maybe_expr) => self.resolve_return(maybe_expr),
            Statement::Print(expr) => self.resolve_expr(expr),
        }
    }

    fn resolve_var_decl(&mut self, name: &str, maybe_expr: &mut Option<Expr>) {
        if let Some(expr) = maybe_expr {
            self.resolve_expr(expr);
        }
        self.register_identifier(name);
        if maybe_expr.is_some() {
            self.initialize_identifier(name, 0);
        }
    }

    fn resolve_if(&mut self, if_stmt: &mut IfStatement) {
        self.resolve_expr(&mut if_stmt.cond);
        self.resolve_stmt(&mut if_stmt.then_branch);
        if let Some(stmt) = &mut if_stmt.else_branch {
            self.resolve_stmt(stmt)
        }
    }

    fn resolve_block(&mut self, stmts: &mut Vec<Statement>) {
        self.new_scope();

        for stmt in stmts {
            self.resolve_stmt(stmt);
        }

        self.end_scope();
    }

    fn resolve_while(&mut self, while_stmt: &mut WhileStatement) {
        self.resolve_expr(&mut while_stmt.cond);
        self.resolve_stmt(&mut while_stmt.stmt);
    }

    fn resolve_fun_decl(&mut self, fun_decl: &mut FunctionDecl) {
        self.register_identifier(&fun_decl.name);
        self.new_scope();

        for param_name in &fun_decl.params {
            self.register_identifier(param_name);
            self.initialize_identifier(param_name, 0);
        }

        for stmt in &mut fun_decl.body {
            self.resolve_stmt(stmt);
        }

        self.end_scope();
    }

    fn resolve_return(&mut self, maybe_expr: &mut Option<Expr>) {
        if let Some(expr) = maybe_expr {
            self.resolve_expr(expr);
        }
    }

    fn resolve_expr(&mut self, expr: &mut Expr) {
        println!("Resolve expr {:?}", expr);
        println!("Scopes: {:?}", self.scopes);
        match expr {
            Expr::Identifier(_, _) => self.resolve_identifier(expr),
            Expr::Literal(_) | Expr::Unary(_) => (),
            Expr::Binary(BinaryExpr { left, op: _, right }) => {
                self.resolve_expr(left);
                self.resolve_expr(right);
            }
            Expr::Grouping(expr) => self.resolve_expr(expr),
            Expr::Assignment(var_name, resolving_depth, expr) => {
                self.resolve_assignment(var_name, expr, resolving_depth)
            }
            Expr::Call(call_expr) => self.resolve_call(call_expr),
        }
    }

    fn resolve_assignment(&mut self, var_name: &str, expr: &mut Expr, depth: &mut Option<u8>) {
        println!(
            "Resolved {} with depth {:?} [ASSIGNMENT]",
            var_name,
            self.find_resolving_depth(var_name)
        );
        *depth = self.find_resolving_depth(var_name);
        self.resolve_expr(expr);
        self.initialize_identifier(var_name, depth.unwrap()); // TODO do not unwrap
    }

    fn resolve_call(&mut self, call_expr: &mut CallExpr) {
        self.resolve_expr(&mut call_expr.callee);
        for arg in &mut call_expr.args {
            self.resolve_expr(arg);
        }
    }
}
