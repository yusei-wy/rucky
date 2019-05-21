use crate::ast::*;
use crate::lexer::Lexer;
use crate::token::Token;

pub struct Parser {
    l: Lexer,
    cur_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

impl Parser {
    pub fn new(l: Lexer) -> Parser {
        let mut p = Parser {
            l,
            cur_token: Token::EOF,
            peek_token: Token::EOF,
            errors: vec![],
        };

        p.next_token();
        p.next_token();

        p
    }

    fn next_token(&mut self) {
        std::mem::swap(&mut self.cur_token, &mut self.peek_token);
        self.peek_token = self.l.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program: Program = vec![];

        while !self.cur_token_is(Token::EOF) {
            match self.parse_stmt() {
                Some(stmt) => program.push(stmt),
                None => {}
            }
            self.next_token();
        }

        program
    }

    /// Parse statement
    fn parse_stmt(&mut self) -> Option<Stmt> {
        match self.cur_token {
            Token::Let => self.parse_let_stmt(),
            Token::Return => self.parse_return_stmt(),
            _ => self.parse_expr_stmt(),
        }
    }

    /// Parse let statement
    fn parse_let_stmt(&mut self) -> Option<Stmt> {
        match self.peek_token {
            Token::Ident(_) => self.next_token(),
            _ => return None,
        }

        let name = match self.parse_ident() {
            Some(name) => name,
            None => return None,
        };

        if !self.consume_token(Token::Assign) {
            return None;
        }

        self.next_token();

        let expr = match self.parse_expr(Precedence::Lowest) {
            Some(expr) => expr,
            None => return None,
        };

        while !self.cur_token_is(Token::Semicolon) {
            self.next_token();
        }

        Some(Stmt::Let(name, expr))
    }

    /// Parse return statement
    fn parse_return_stmt(&mut self) -> Option<Stmt> {
        self.next_token();

        let expr = match self.parse_expr(Precedence::Lowest) {
            Some(expr) => expr,
            None => return None,
        };

        while !self.cur_token_is(Token::Semicolon) {
            self.next_token();
        }

        Some(Stmt::Return(expr))
    }

    fn parse_ident(&self) -> Option<Ident> {
        match self.cur_token {
            Token::Ident(ref ident) => Some(Ident(ident.clone())),
            _ => None,
        }
    }

    /// Parse expression statement
    fn parse_expr_stmt(&mut self) -> Option<Stmt> {
        match self.parse_expr(Precedence::Lowest) {
            Some(expr) => {
                self.consume_token(Token::Semicolon);
                Some(Stmt::Expr(expr))
            }
            _ => None,
        }
    }

    /// Parse expression
    fn parse_expr(&self, precendence: Precedence) -> Option<Expr> {
        let left = match self.cur_token {
            Token::Ident(_) => self.parse_ident_expr(),
            Token::Int(_) => self.parse_int_expr(),
            _ => return None,
        };

        left
    }

    fn parse_ident_expr(&self) -> Option<Expr> {
        match self.parse_ident() {
            Some(ident) => Some(Expr::Ident(ident)),
            _ => None,
        }
    }

    fn parse_int_expr(&self) -> Option<Expr> {
        match self.cur_token {
            Token::Int(ref int) => Some(Expr::Literal(Literal::Int(int.clone()))),
            _ => None,
        }
    }

    fn cur_token_is(&self, tok: Token) -> bool {
        self.cur_token == tok
    }

    fn peek_token_is(&self, tok: &Token) -> bool {
        self.peek_token == *tok
    }

    fn consume_token(&mut self, tok: Token) -> bool {
        if self.peek_token_is(&tok) {
            self.next_token();
            true
        } else {
            self.peek_error(&tok);
            false
        }
    }

    fn peek_error(&mut self, tok: &Token) {
        let msg = format!(
            "expected next Some(token to be {:?}, got {:?} instead",
            tok, self.peek_token,
        );
        self.errors.push(msg);
    }
}

pub fn check_parser_errors(p: &Parser) {
    if p.errors.len() == 0 {
        return;
    }

    eprintln!("parser has {} erros", p.errors.len());
    for msg in &p.errors {
        eprintln!("parser error: {}", msg);
    }
    panic!("");
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use ast::*;
    use lexer::Lexer;
    use parser::{check_parser_errors, Parser};

    #[test]
    fn test_let_statements() {
        let input = r#"
let x = 5;
let y = 10;
let foobar = 838383;
"#;

        let l = Lexer::new(input);
        let mut p = Parser::new(l);

        let program = p.parse_program();
        let len = program.len();

        check_parser_errors(&p);

        if len == 0 {
            panic!("parse_program() returned empty");
        } else if len != 3 {
            panic!("Program does not contain 3 statements. got={}", len);
        }

        let tests: Vec<Stmt> = vec![
            Stmt::Let(Ident(String::from("x")), Expr::Literal(Literal::Int(5))),
            Stmt::Let(Ident(String::from("y")), Expr::Literal(Literal::Int(10))),
            Stmt::Let(
                Ident(String::from("foobar")),
                Expr::Literal(Literal::Int(838383)),
            ),
        ];

        for (i, tt) in tests.iter().enumerate() {
            let stmt = &program[i];
            if stmt != tt {
                panic!("got={:?}. expected={:?}", stmt, tt);
            }
        }
    }

    #[test]
    fn test_return_statements() {
        let input = r#"
return 5;
return 10;
return 993322;
"#;

        let l = Lexer::new(input);
        let mut p = Parser::new(l);

        let program = p.parse_program();
        let len = program.len();

        check_parser_errors(&p);

        if len == 0 {
            panic!("parse_program() returned empty");
        } else if len != 3 {
            panic!("Program does not contain 3 statements. got={}", len);
        }

        let tests: Vec<Stmt> = vec![
            Stmt::Return(Expr::Literal(Literal::Int(5))),
            Stmt::Return(Expr::Literal(Literal::Int(10))),
            Stmt::Return(Expr::Literal(Literal::Int(993322))),
        ];

        for (i, tt) in tests.iter().enumerate() {
            let stmt = &program[i];
            if stmt != tt {
                panic!("got={:?}. expected={:?}", stmt, tt);
            }
        }
    }

    #[test]
    fn test_ident_expr() {
        let input = "foobar;";

        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        let len = program.len();

        check_parser_errors(&p);

        if len == 0 {
            panic!("Program has not enought statments. got={}", len);
        }

        let tests: Vec<Stmt> = vec![Stmt::Expr(Expr::Ident(Ident(String::from("foobar"))))];

        if program != tests {
            panic!("got={:?}. expected={:?}", program, tests);
        }
    }

    #[test]
    fn test_int_expr() {
        let input = "5;";

        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program();
        let len = program.len();

        check_parser_errors(&p);

        if len == 0 {
            panic!("Program has not enought statments. got={}", len);
        }

        let tests: Vec<Stmt> = vec![Stmt::Expr(Expr::Literal(Literal::Int(5)))];

        if program != tests {
            panic!("got={:?}. expected={:?}", program, tests);
        }
    }
}
