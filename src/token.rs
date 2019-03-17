#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal,
    EOF,

    // Identifier + Literal
    Ident(String),
    Int(i64),

    // Operator
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    Equal,
    NotEqual,
    LessThan,
    GreaterThan,

    // Delimiter
    Comma,
    Semicolon,

    Lparen,
    Rparen,
    Lbrace,
    Rbrace,

    // Keyword
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}
