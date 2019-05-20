#[derive(Debug, PartialEq)]
pub struct Ident(pub String);

#[derive(Debug, PartialEq)]
pub enum Prefix {
    Plus,
}

#[derive(Debug, PartialEq)]
pub enum Infix {
    Plus,
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Blank,
    Let(Ident, Expr),
    Return(Expr),
    Expr(Expr),
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Ident(Ident),
    Literal(Literal),
    Prefix(Prefix, Box<Expr>),
    Infix(Infix, Box<Expr>, Box<Expr>),
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Int(i64),
    String(String),
}

pub type BlockStmt = Vec<Stmt>;

pub type Program = BlockStmt;

#[derive(Debug, PartialEq)]
pub enum Precedence {
    Lowest,
    Equals,      // ==
    LessGreater, // > or <
    Sum,         // +
    Product,     // *
    Prefix,      // -X or !X
    Call,        // myFunction(X)
}
