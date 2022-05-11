use crate::{
    ast::*,
    scanner::{Token, TokenType, TokenType::*}, Result,
};

pub struct Parser<'tokens> {
    tokens: &'tokens Vec<Token<'tokens>>,
    next: usize,
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
        Parser { tokens, next: 0 }
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
        if !self.matches(token_type) {
            panic!("Expected {:?}", token_type); // FIXME do not panic
        } else {
            self.previous()
        }
    }

    fn consume_or_error(&mut self, token_type: TokenType, error_msg: &str) -> Result<&Token> {
        if !self.matches(token_type) {
            panic!("{}", error_msg); // FIXME do not panic
        } else {
            self.previous()
        }
    }

    fn previous(&self) -> Result<&Token> {
        if let Some(token) = self.tokens.get(self.next - 1) {
            Ok(token)
        } else {
            panic!("Could not get previous token")
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
        } else {
            self.statement()
        }
    }

    fn var_declaration(&mut self) -> Result<Statement> {
        let name = self
            .consume_or_error(Identifier, "Expected a variable name")?
            .lexeme
            .into();

        let mut initializer = None;
        if self.matches(Equal) {
            initializer = Some(self.expr()?);
        }

        self.consume_or_error(Semicolon, "Expected ';' after variable declaration.")?;

        Ok(Statement::VariableDecl(name, initializer))
    }

    fn statement(&mut self) -> Result<Statement> {
        if self.matches(Print) {
            self.print_stmt()
        } else if self.matches(LeftBrace) {
            self.block_stmt()
        } else {
            self.expr_stmt()
        }
    }

    fn print_stmt(&mut self) -> Result<Statement> {
        let value = self.expr()?;
        self.consume_or_error(Semicolon, "Expected ';' after value.")?;
        Ok(Statement::Print(value))
    }

    fn expr_stmt(&mut self) -> Result<Statement> {
        let expr = self.expr()?;
        self.consume_or_error(Semicolon, "Expected ';' after expression.")?;
        Ok(Statement::Expr(expr))
    }

    fn block_stmt(&mut self) -> Result<Statement> {
        let mut statements = Vec::new();

        while !self.check(RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume_or_error(RightBrace, "Expected '}' after block.")?;

        Ok(Statement::Block(statements))
    }

    fn expr(&mut self) -> Result<Expr> {
        self.assignment_expr()
    }

    fn assignment_expr(&mut self) -> Result<Expr> {
        let expr = self.equality_expr()?;

        if self.matches(Equal) {
            match expr {
                Expr::Identifier(ref name) => {
                    let rvalue = self.assignment_expr()?;
                    return Ok(Expr::Assignment(name.clone(), Box::new(rvalue)));
                }
                _ => panic!("Invalid assignment target {:?}", expr), // TODO do not panic
            }
        }

        Ok(expr)
    }

    fn equality_expr(&mut self) -> Result<Expr> {
        let mut expr = self.boolean_expr()?;

        while self.matches(EqualEqual) || self.matches(BangEqual) {
            let op = self.previous()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                op: op.type_,
                right: Box::new(self.boolean_expr()?),
            });
        }

        Ok(expr)
    }

    fn boolean_expr(&mut self) -> Result<Expr> {
        let mut expr = self.comparison_expr()?;

        while self.matches(And) || self.matches(Or) {
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
            self.primary_expr()
        }
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
                Err(_) => todo!(), // TODO generate error
            }
        } else if self.matches(Identifier) {
            Ok(Expr::Identifier(self.previous()?.lexeme.into()))
        } else if self.matches(LeftParen) {
            let expr = self.equality_expr()?;
            self.consume(RightParen)?;
            Ok(Expr::Grouping(Box::new(expr)))
        } else {
            todo!() // TODO generate error
        }
    }
}
