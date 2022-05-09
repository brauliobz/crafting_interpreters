use crate::{
    ast::*,
    scanner::{Token, TokenType, TokenType::*},
};

pub struct Parser<'tokens> {
    tokens: &'tokens Vec<Token<'tokens>>,
    next: usize,
}

pub fn parse(tokens: &Vec<Token>) -> Vec<Statement> {
    let mut parser = Parser::new(tokens);
    parser.statements()
}

pub fn parse_expr(tokens: &Vec<Token>) -> Expr {
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

    fn consume(&mut self, token_type: TokenType) -> &Token {
        if !self.matches(token_type) {
            panic!("Expected {:?}", token_type); // FIXME do not panic
        } else {
            self.previous()
        }
    }

    fn consume_or_error(&mut self, token_type: TokenType, error_msg: &str) -> &Token {
        if !self.matches(token_type) {
            panic!("{}", error_msg); // FIXME do not panic
        } else {
            self.previous()
        }
    }

    fn previous(&self) -> &Token {
        if let Some(token) = self.tokens.get(self.next - 1) {
            token
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

    fn statements(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            statements.push(self.statement());
        }

        statements
    }

    fn statement(&mut self) -> Statement {
        if self.matches(Print) {
            self.print_stmt()
        } else {
            self.expr_stmt()
        }
    }

    fn print_stmt(&mut self) -> Statement {
        let value = self.expr();
        self.consume_or_error(Semicolon, "Expected ';' after value.");
        Statement::Print(value)
    }

    fn expr_stmt(&mut self) -> Statement {
        let expr = self.expr();
        self.consume_or_error(Semicolon, "Expected ';' after expression.");
        Statement::Expr(expr)
    }

    fn expr(&mut self) -> Expr {
        self.equality_expr()
    }

    fn equality_expr(&mut self) -> Expr {
        let mut expr = self.boolean_expr();

        while self.matches(EqualEqual) || self.matches(BangEqual) {
            let op = self.previous();
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                op: op.type_,
                right: Box::new(self.boolean_expr()),
            });
        }

        expr
    }

    fn boolean_expr(&mut self) -> Expr {
        let mut expr = self.comparison_expr();

        while self.matches(And) || self.matches(Or) {
            let op = self.previous();
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                op: op.type_,
                right: Box::new(self.comparison_expr()),
            });
        }

        expr
    }

    fn comparison_expr(&mut self) -> Expr {
        let mut expr = self.term_expr();

        while self.matches(Greater)
            || self.matches(GreaterEqual)
            || self.matches(Less)
            || self.matches(LessEqual)
        {
            let op = self.previous();
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                op: op.type_,
                right: Box::new(self.term_expr()),
            });
        }

        expr
    }

    fn term_expr(&mut self) -> Expr {
        let mut expr = self.factor_expr();

        while self.matches(Plus) || self.matches(Minus) {
            let op = self.previous();
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                op: op.type_,
                right: Box::new(self.factor_expr()),
            });
        }

        expr
    }

    fn factor_expr(&mut self) -> Expr {
        let mut expr = self.unary_expr();

        while self.matches(Star) || self.matches(Slash) {
            let op = self.previous();
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                op: op.type_,
                right: Box::new(self.unary_expr()),
            });
        }

        expr
    }

    fn unary_expr(&mut self) -> Expr {
        if self.matches(Bang) || self.matches(Minus) {
            let op = self.previous().type_;
            Expr::Unary(UnaryExpr {
                op,
                expr: Box::new(self.unary_expr()),
            })
        } else {
            self.primary_expr()
        }
    }

    fn primary_expr(&mut self) -> Expr {
        if self.matches(False) {
            Expr::Literal(LiteralExpr::Boolean(false))
        } else if self.matches(True) {
            Expr::Literal(LiteralExpr::Boolean(true))
        } else if self.matches(Nil) {
            Expr::Literal(LiteralExpr::Nil)
        } else if self.matches(String) {
            let lexeme = self.previous().lexeme;
            let string = lexeme[1..(lexeme.len() - 1)].into(); // remove the ""
            Expr::Literal(LiteralExpr::String(string))
        } else if self.matches(NumberLiteral) {
            let result = self.previous().lexeme.parse::<f64>();
            match result {
                Ok(number) => Expr::Literal(LiteralExpr::Number(number)),
                Err(_) => todo!(),
            }
        } else if self.matches(Identifier) {
            Expr::Identifier(self.previous().lexeme.into())
        } else if self.matches(LeftParen) {
            let expr = self.equality_expr();
            self.consume(RightParen);
            Expr::Grouping(Box::new(expr))
        } else {
            todo!()
        }
    }
}
