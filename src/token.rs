#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal(String),
    EOF,

    // Identifier + Literal
    Ident(String),
    Int(i64),

    // Operator
    Assign,
    Plus,

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
}