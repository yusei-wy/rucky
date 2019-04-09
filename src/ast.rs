#[derive(Debug, PartialEq)]
pub struct Ident(pub String);

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
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Int(i64),
    String(String),
}

pub type BlockStmt = Vec<Stmt>;

pub type Program = BlockStmt;
