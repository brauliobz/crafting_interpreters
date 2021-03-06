use std::rc::Rc;

use crate::{
    ast::*,
    error::{compilation_error, ice, CompilationError, ErrorOrEarlyReturn, ICE},
    scanner::{Token, TokenType, TokenType::*},
    Result,
};

pub struct Parser<'tokens> {
    tokens: &'tokens Vec<Token<'tokens>>,
    next: usize,
    inside_function: u32,
}

pub fn parse(tokens: &Vec<Token>) -> Result<Vec<Statement>> {
    let mut parser = Parser::new(tokens);
    parser.declarations()
}

pub fn parse_expr(tokens: &Vec<Token>) -> Result<Expr> {
    let mut parser = Parser::new(tokens);
    parser.equality_expr()
}

impl<'tokens> Parser<'tokens> {
    fn new(tokens: &'tokens Vec<Token>) -> Parser<'tokens> {
        Parser {
            tokens,
            next: 0,
            inside_function: 0,
        }
    }

    fn matches(&mut self, token_type: TokenType) -> bool {
        if let Some(token) = self.tokens.get(self.next) {
            if token.type_ == token_type {
                self.next += 1;
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        matches!(self.tokens.get(self.next), Some(Token { type_, ..}) if *type_ == token_type)
    }

    fn consume(&mut self, token_type: TokenType) -> Result<&Token> {
        if self.is_at_end() {
            Err(compilation_error(CompilationError::ExpectedToken(
                format!("{:?}", token_type),
                "Eof".into(),
            )))
        } else if !self.matches(token_type) {
            Err(compilation_error(CompilationError::ExpectedToken(
                format!("{:?}", token_type),
                self.peek().unwrap().into(),
            )))
        } else {
            self.previous()
        }
    }

    fn consume_or_error(
        &mut self,
        token_type: TokenType,
        error: ErrorOrEarlyReturn,
    ) -> Result<&Token> {
        if !self.matches(token_type) {
            Err(error)
        } else {
            self.previous()
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.next)
    }

    fn previous(&self) -> Result<&Token> {
        if let Some(token) = self.tokens.get(self.next - 1) {
            Ok(token)
        } else {
            Err(ice(ICE::Generic("Could not get previous token".into())))
        }
    }

    fn is_at_end(&self) -> bool {
        matches!(
            self.tokens.get(self.next),
            Some(&Token { type_: Eof, .. }) | None
        )
    }

    fn declarations(&mut self) -> Result<Vec<Statement>> {
        let mut decls = Vec::new();

        while !self.is_at_end() {
            decls.push(self.declaration()?);
        }

        Ok(decls)
    }

    fn declaration(&mut self) -> Result<Statement> {
        if self.matches(Var) {
            self.var_declaration()
        } else if self.matches(Fun) {
            self.fun_declaration()
        } else {
            self.statement()
        }
    }

    fn var_declaration(&mut self) -> Result<Statement> {
        let name = self
            .consume_or_error(
                Identifier,
                compilation_error(CompilationError::ExpectedNameAfterVar),
            )?
            .lexeme
            .into();

        let mut initializer = None;
        if self.matches(Equal) {
            initializer = Some(self.expr()?);
        }

        self.consume_or_error(
            Semicolon,
            compilation_error(CompilationError::ExpectedSemicolonAfterVarDecl),
        )?;

        Ok(Statement::VariableDecl(name, initializer))
    }

    fn fun_declaration(&mut self) -> Result<Statement> {
        // function name

        let name = self.consume(Identifier)?.lexeme.to_owned();

        // parameters

        self.consume(LeftParen)?;
        let mut params = vec![];

        while !self.check(RightParen) {
            params.push(self.consume(Identifier)?.lexeme.to_owned());

            if params.len() > 256 {
                return Err(compilation_error(CompilationError::GenericError(
                    "Can't have more than 256 arguments in a function definition.".into(),
                )));
            }

            if !self.matches(Comma) {
                break;
            }
        }

        self.consume(RightParen)?;

        // function body

        self.consume(LeftBrace)?;
        self.inside_function += 1;

        let mut body = vec![];
        while !self.check(RightBrace) && !self.is_at_end() {
            let maybe_stmt = self.declaration();
            match maybe_stmt {
                Ok(stmt) => body.push(stmt),
                Err(err) => {
                    self.inside_function -= 1;
                    return Err(err);
                }
            }
        }

        self.consume(RightBrace)?;
        self.inside_function -= 1;

        Ok(Statement::FunDecl(Rc::new(FunctionDecl {
            name,
            params,
            body,
        })))
    }

    fn statement(&mut self) -> Result<Statement> {
        if self.matches(If) {
            self.if_stmt()
        } else if self.matches(Print) {
            self.print_stmt()
        } else if self.matches(LeftBrace) {
            self.block_stmt()
        } else if self.matches(While) {
            self.while_stmt()
        } else if self.matches(For) {
            self.for_stmt()
        } else if self.matches(Return) {
            self.return_stmt()
        } else {
            self.expr_stmt()
        }
    }

    fn if_stmt(&mut self) -> Result<Statement> {
        self.consume(LeftParen)?;
        let cond = self.expr()?;
        self.consume(RightParen)?;

        let then_branch = Box::new(self.statement()?);

        let mut else_branch = None;
        if self.matches(Else) {
            else_branch = Some(Box::new(self.statement()?));
        }

        Ok(Statement::If(IfStatement {
            cond,
            then_branch,
            else_branch,
        }))
    }

    fn print_stmt(&mut self) -> Result<Statement> {
        let value = self.expr()?;
        self.consume_or_error(
            Semicolon,
            compilation_error(CompilationError::GenericError(
                "Expected ';' after value.".into(),
            )),
        )?; // TODO create specific error
        Ok(Statement::Print(value))
    }

    fn expr_stmt(&mut self) -> Result<Statement> {
        let expr = self.expr()?;
        self.consume_or_error(
            Semicolon,
            compilation_error(CompilationError::GenericError(
                "Expected ';' after expression.".into(),
            )),
        )?; // TODO create specific error
        Ok(Statement::Expr(expr))
    }

    fn block_stmt(&mut self) -> Result<Statement> {
        let mut statements = Vec::new();

        while !self.check(RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume_or_error(
            RightBrace,
            compilation_error(CompilationError::GenericError(
                "Expected '}' at end of block.".into(),
            )),
        )?; // TODO create specific error

        Ok(Statement::Block(statements))
    }

    fn while_stmt(&mut self) -> Result<Statement> {
        self.consume(LeftParen)?;
        let cond = self.expr()?;
        self.consume(RightParen)?;

        let stmt = Box::new(self.statement()?);

        Ok(Statement::While(WhileStatement { cond, stmt }))
    }

    fn for_stmt(&mut self) -> Result<Statement> {
        self.consume(LeftParen)?;

        // initialization clause

        let initialization = if self.matches(Semicolon) {
            None
        } else if self.matches(Var) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expr_stmt()?)
        };

        // condition clause

        let condition = if self.check(Semicolon) {
            None
        } else {
            Some(self.expr()?)
        };
        self.consume(Semicolon)?;

        // increment clause

        let increment = if self.check(RightParen) {
            None
        } else {
            Some(Statement::Expr(self.expr()?))
        };
        self.consume(RightParen)?;

        // looped statement

        let body = self.statement()?;

        // desugar into a while loop

        let mut while_body = vec![body];
        while_body.extend(increment.into_iter());

        let mut gen_body = vec![];
        gen_body.extend(initialization.into_iter());
        gen_body.push(Statement::While(WhileStatement {
            cond: condition.unwrap_or(Expr::Literal(LiteralExpr::Boolean(true))),
            stmt: Box::new(Statement::Block(while_body)),
        }));

        Ok(Statement::Block(gen_body))
    }

    fn return_stmt(&mut self) -> Result<Statement> {
        let expr = if self.check(Semicolon) {
            None
        } else {
            Some(self.expr()?)
        };

        self.consume(Semicolon)?;

        if self.inside_function > 0 {
            Ok(Statement::Return(expr))
        } else {
            Err(compilation_error(CompilationError::ReturnOutsideFunction))
        }
    }

    fn expr(&mut self) -> Result<Expr> {
        self.assignment_expr()
    }

    fn assignment_expr(&mut self) -> Result<Expr> {
        let expr = self.or_expr()?;

        if self.matches(Equal) {
            match expr {
                Expr::Identifier(ref name) => {
                    let rvalue = self.assignment_expr()?;
                    return Ok(Expr::Assignment(name.clone(), Box::new(rvalue)));
                }
                _ => {
                    return Err(compilation_error(CompilationError::GenericError(format!(
                        "Invalid assignment target {:?}",
                        expr
                    ))))
                } // TODO create specific error
            }
        }

        Ok(expr)
    }

    fn or_expr(&mut self) -> Result<Expr> {
        let mut expr = self.and_expr()?;

        while self.matches(Or) {
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                op: Or,
                right: Box::new(self.and_expr()?),
            });
        }

        Ok(expr)
    }

    fn and_expr(&mut self) -> Result<Expr> {
        let mut expr = self.equality_expr()?;

        while self.matches(And) {
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                op: And,
                right: Box::new(self.equality_expr()?),
            });
        }

        Ok(expr)
    }

    fn equality_expr(&mut self) -> Result<Expr> {
        let mut expr = self.comparison_expr()?;

        while self.matches(EqualEqual) || self.matches(BangEqual) {
            let op = self.previous()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                op: op.type_,
                right: Box::new(self.comparison_expr()?),
            });
        }

        Ok(expr)
    }

    fn comparison_expr(&mut self) -> Result<Expr> {
        let mut expr = self.term_expr()?;

        while self.matches(Greater)
            || self.matches(GreaterEqual)
            || self.matches(Less)
            || self.matches(LessEqual)
        {
            let op = self.previous()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                op: op.type_,
                right: Box::new(self.term_expr()?),
            });
        }

        Ok(expr)
    }

    fn term_expr(&mut self) -> Result<Expr> {
        let mut expr = self.factor_expr()?;

        while self.matches(Plus) || self.matches(Minus) {
            let op = self.previous()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                op: op.type_,
                right: Box::new(self.factor_expr()?),
            });
        }

        Ok(expr)
    }

    fn factor_expr(&mut self) -> Result<Expr> {
        let mut expr = self.unary_expr()?;

        while self.matches(Star) || self.matches(Slash) {
            let op = self.previous()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                op: op.type_,
                right: Box::new(self.unary_expr()?),
            });
        }

        Ok(expr)
    }

    fn unary_expr(&mut self) -> Result<Expr> {
        if self.matches(Bang) || self.matches(Minus) {
            let op = self.previous()?.type_;
            Ok(Expr::Unary(UnaryExpr {
                op,
                expr: Box::new(self.unary_expr()?),
            }))
        } else {
            self.call_expr()
        }
    }

    fn call_expr(&mut self) -> Result<Expr> {
        let mut expr = self.primary_expr()?;

        while self.matches(LeftParen) {
            expr = self.finish_call_expr(expr)?;
        }

        Ok(expr)
    }

    fn finish_call_expr(&mut self, callee: Expr) -> Result<Expr> {
        let mut args = vec![];

        if self.check(Comma) {
            return Err(compilation_error(CompilationError::GenericError(
                "Expected an expression or closing parenthesis, not a comma.".into(),
            )));
        }

        if !self.check(RightParen) {
            loop {
                args.push(self.expr()?);

                if args.len() > 256 {
                    return Err(compilation_error(CompilationError::GenericError(
                        "Can't have more than 256 arguments in a function call.".into(),
                    )));
                }

                if self.check(RightParen) {
                    break;
                } else if self.check(Comma) {
                    self.consume(Comma)?;
                    continue;
                } else {
                    return Err(compilation_error(CompilationError::GenericError(
                        "Expected an expression or closing parenthesis, not a comma.".into(),
                    )));
                }
            }
        }

        self.consume(RightParen)?;

        Ok(Expr::Call(CallExpr {
            callee: Box::new(callee),
            args,
        }))
    }

    fn primary_expr(&mut self) -> Result<Expr> {
        if self.matches(False) {
            Ok(Expr::Literal(LiteralExpr::Boolean(false)))
        } else if self.matches(True) {
            Ok(Expr::Literal(LiteralExpr::Boolean(true)))
        } else if self.matches(Nil) {
            Ok(Expr::Literal(LiteralExpr::Nil))
        } else if self.matches(String) {
            let lexeme = self.previous()?.lexeme;
            let string = lexeme[1..(lexeme.len() - 1)].into(); // remove the ""
            Ok(Expr::Literal(LiteralExpr::String(string)))
        } else if self.matches(NumberLiteral) {
            let result = self.previous()?.lexeme.parse::<f64>();
            match result {
                Ok(number) => Ok(Expr::Literal(LiteralExpr::Number(number))),
                Err(_) => Err(compilation_error(CompilationError::InvalidLiteral(
                    "number".into(),
                    self.previous()?.lexeme.into(),
                ))),
            }
        } else if self.matches(Identifier) {
            Ok(Expr::Identifier(self.previous()?.lexeme.into()))
        } else if self.matches(LeftParen) {
            let expr = self.assignment_expr()?;
            self.consume(RightParen)?;
            Ok(Expr::Grouping(Box::new(expr)))
        } else {
            Err(compilation_error(CompilationError::GenericError(
                "Expression expected".into(),
            )))
        }
    }
}
