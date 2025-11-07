#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Let,
    Print,

    // Types
    Identifier(String, bool),
    Integer(i64),
    Float(f64),
    Char(char),
    String(String),

    // Operators
    Plus,
    PlusEquals,
    Minus,
    MinusEquals,
    Star,
    StarEquals,
    Exp,
    ExpEquals,
    Slash,
    SlashEquals,
    Increment,
    Decrement,
    Equals,
    EqualsEquals,
    LessEquals,
    GreaterEquals,

    // Punctuation
    Dot,
    Comma,
    Colon,
    Semicolon,

    // Brackets
    LParen,
    RParen,
    LBracket,
    RBracket,
    LCurly,
    RCurly,
    LAngle,
    RAngle,

    // Iteration
    Iter,
    IterIncl,
    Ellipsis,
    Arrow,
    DArrow,

    // Specials
    EOF,
    Unknown(char),
}