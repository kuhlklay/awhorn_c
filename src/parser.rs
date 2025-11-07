use crate::lexer::Lexer;
use crate::token::Token;

pub struct Parser {
    lexer: Lexer,
    current: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current = lexer.next_token();
        Self { lexer, current }
    }

    fn advance(&mut self) {
        self.current = self.lexer.next_token();
    }

    fn expect(&mut self, expected: &Token) {
        if &self.current != expected {
            panic!("Erwartet {:?}, aber gefunden {:?}", expected, self.current);
        }
        self.advance();
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        while self.current != Token::EOF {
            stmts.push(self.parse_statement());
        }
        stmts
    }

    fn parse_statement(&mut self) -> Stmt {
        match &self.current {
            Token::Let => {
                self.advance();
                let name = match &self.current {
                    Token::Identifier(id) => id.clone(),
                    _ => panic!("Erwarteter Bezeichner nach 'let'"),
                };
                self.advance();
                self.expect(&Token::Equals);
                let expr = self.parse_expression();
                self.expect(&Token::Semicolon);
                Stmt::Let(name, expr)
            }
            Token::Print => {
                self.advance();
                self.expect(&Token::LParen);
                let expr = self.parse_expression();
                self.expect(&Token::RParen);
                self.expect(&Token::Semicolon);
                Stmt::Print(expr)
            }
            _ => {
                let expr = self.parse_expression();
                self.expect(&Token::Semicolon);
                Stmt::Expr(expr)
            }
        }
    }

    fn parse_expression(&mut self) -> Expr {
        self.parse_term()
    }

    fn parse_term(&mut self) -> Expr {
        let mut expr = self.parse_factor();

        while matches!(self.current, Token::Plus | Token::Minus) {
            let op = match self.current {
                Token::Plus => Operator::Plus,
                Token::Minus => Operator::Minus,
                _ => unreachable!(),
            };
            self.advance();
            let rhs = self.parse_factor();
            expr = Expr::Binary(Box::new(expr), op, Box::new(rhs));
        }

        expr
    }

    fn parse_factor(&mut self) -> Expr {
        let mut expr = self.parse_primary();

        while matches!(self.current, Token::Star | Token::Slash) {
            let op = match self.current {
                Token::Star => Operator::Star,
                Token::Slash => Operator::Slash,
                _ => unreachable!(),
            };
            self.advance();
            let rhs = self.parse_primary();
            expr = Expr::Binary(Box::new(expr), op, Box::new(rhs));
        }

        expr
    }

    fn parse_primary(&mut self) -> Expr {
        match &self.current {
            Token::Integer(n) => { let e = Expr::Integer(*n); self.advance(); e }
            Token::Float(f) => { let e = Expr::Float(*f); self.advance(); e }
            Token::String(s) => { let e = Expr::String(s.clone()); self.advance(); e }
            Token::Char(c) => { let e = Expr::Char(*c); self.advance(); e }
            Token::Identifier(id) => {
                let id_clone = id.clone();
                self.advance();
                if self.current == Token::LParen {
                    // function call
                    self.advance();
                    let mut args = Vec::new();
                    if self.current != Token::RParen {
                        args.push(self.parse_expression());
                        while self.current == Token::Comma {
                            self.advance();
                            args.push(self.parse_expression());
                        }
                    }
                    self.expect(&Token::RParen);
                    Expr::Call(id_clone, args)
                } else {
                    Expr::Identifier(id_clone)
                }
            }
            Token::LParen => {
                self.advance();
                let expr = self.parse_expression();
                self.expect(&Token::RParen);
                expr
            }
            _ => panic!("Unerwartetes Token in Ausdruck: {:?}", self.current),
        }
    }
}