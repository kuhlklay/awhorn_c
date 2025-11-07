#[derive(Debug, Clone)]
pub enum Expr {
    Integer(i64),
    Float(f64),
    Char(char),
    String(String),
    Identifier(String),
    Binary(Box<Expr>, Operator, Box<Expr>),
    Unary(Operator, Box<Expr>),
    Call(String, Vec<Expr>),
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Let(String, Expr),
    Print(Expr),
    Expr(Expr),
}

#[derive(Debug, Clone)]
pub enum Operator {
    Plus,
    Minus,
    Star,
    Slash,
}