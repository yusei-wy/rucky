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
    Lt,
    Gt,

    // Delimiter
    Comma,
    Semicolon,

    Lparen,
    Rparen,
    Lbrace,
    Rbrace,

    // Keyword
    Fn,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}
