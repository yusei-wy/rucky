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
    fn parse_expr(&mut self, precendence: Precedence) -> Option<Expr> {
        // prefix
        let left = match self.cur_token {
            Token::Ident(_) => self.parse_ident_expr(),
            Token::Int(_) => self.parse_int_expr(),
            Token::Bang | Token::Plus | Token::Minus => self.parse_prefix_expr(),
            _ => return None,
        };

        if !self.peek_token_is_infix() {
            return left;
        }

        self.next_token();

        match left {
            Some(expr) => self.parse_infix_expr(expr),
            _ => None,
        }
    }

    /// Parse identifier expression
    fn parse_ident_expr(&self) -> Option<Expr> {
        match self.parse_ident() {
            Some(ident) => Some(Expr::Ident(ident)),
            _ => None,
        }
    }

    /// Parse integer literal expression
    fn parse_int_expr(&self) -> Option<Expr> {
        match self.cur_token {
            Token::Int(ref int) => Some(Expr::Literal(Literal::Int(int.clone()))),
            _ => None,
        }
    }

    /// Parser prefix expression
    fn parse_prefix_expr(&mut self) -> Option<Expr> {
        let prefix = match self.cur_token {
            Token::Bang => Prefix::Bang,
            Token::Plus => Prefix::Plus,
            Token::Minus => Prefix::Minus,
            _ => return None,
        };

        self.next_token();

        match self.parse_expr(Precedence::Lowest) {
            Some(expr) => Some(Expr::Prefix(prefix, Box::new(expr))),
            _ => None,
        }
    }

    /// Parser infix expression
    fn parse_infix_expr(&mut self, left: Expr) -> Option<Expr> {
        let infix = match self.cur_token {
            Token::Plus => Infix::Plus,
            Token::Minus => Infix::Minus,
            Token::Asterisk => Infix::Asterisk,
            Token::Slash => Infix::Slash,
            Token::Lt => Infix::Lt,
            Token::Gt => Infix::Gt,
            Token::Equal => Infix::Equal,
            Token::NotEqual => Infix::NotEqual,
            _ => return None,
        };

        self.next_token();

        match self.parse_expr(Precedence::Lowest) {
            Some(expr) => Some(Expr::Infix(infix, Box::new(left), Box::new(expr))),
            _ => None,
        }
    }

    fn cur_token_is(&self, tok: Token) -> bool {
        self.cur_token == tok
    }

    fn peek_token_is(&self, tok: &Token) -> bool {
        self.peek_token == *tok
    }

    fn peek_token_is_infix(&self) -> bool {
        match self.peek_token {
            Token::Plus
            | Token::Minus
            | Token::Asterisk
            | Token::Slash
            | Token::Lt
            | Token::Gt
            | Token::Equal
            | Token::NotEqual => true,
            _ => false,
        }
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

    #[test]
    fn test_prefix_expr() {
        let tests: Vec<(&str, Vec<Stmt>)> = vec![
            (
                "!5;",
                vec![Stmt::Expr(Expr::Prefix(
                    Prefix::Bang,
                    Box::new(Expr::Literal(Literal::Int(5))),
                ))],
            ),
            (
                "-15;",
                vec![Stmt::Expr(Expr::Prefix(
                    Prefix::Minus,
                    Box::new(Expr::Literal(Literal::Int(15))),
                ))],
            ),
        ];

        for (input, expected) in tests {
            let mut parser = Parser::new(Lexer::new(input));
            let program = parser.parse_program();
            let len = program.len();

            if len == 0 {
                panic!("Program has not enought statments. got={}", len);
            }

            if program != expected {
                panic!("got={:?}. expected={:?}", program, expected);
            }
        }
    }

    #[test]
    fn test_infix_expr() {
        let tests: Vec<(&str, Vec<Stmt>)> = vec![
            (
                "5 + 5;",
                vec![Stmt::Expr(Expr::Infix(
                    Infix::Plus,
                    Box::new(Expr::Literal(Literal::Int(5))),
                    Box::new(Expr::Literal(Literal::Int(5))),
                ))],
            ),
            (
                "5 - 5;",
                vec![Stmt::Expr(Expr::Infix(
                    Infix::Minus,
                    Box::new(Expr::Literal(Literal::Int(5))),
                    Box::new(Expr::Literal(Literal::Int(5))),
                ))],
            ),
            (
                "5 * 5;",
                vec![Stmt::Expr(Expr::Infix(
                    Infix::Asterisk,
                    Box::new(Expr::Literal(Literal::Int(5))),
                    Box::new(Expr::Literal(Literal::Int(5))),
                ))],
            ),
            (
                "5 / 5;",
                vec![Stmt::Expr(Expr::Infix(
                    Infix::Slash,
                    Box::new(Expr::Literal(Literal::Int(5))),
                    Box::new(Expr::Literal(Literal::Int(5))),
                ))],
            ),
            (
                "5 < 5;",
                vec![Stmt::Expr(Expr::Infix(
                    Infix::Lt,
                    Box::new(Expr::Literal(Literal::Int(5))),
                    Box::new(Expr::Literal(Literal::Int(5))),
                ))],
            ),
            (
                "5 > 5;",
                vec![Stmt::Expr(Expr::Infix(
                    Infix::Gt,
                    Box::new(Expr::Literal(Literal::Int(5))),
                    Box::new(Expr::Literal(Literal::Int(5))),
                ))],
            ),
            (
                "5 == 5;",
                vec![Stmt::Expr(Expr::Infix(
                    Infix::Equal,
                    Box::new(Expr::Literal(Literal::Int(5))),
                    Box::new(Expr::Literal(Literal::Int(5))),
                ))],
            ),
            (
                "5 != 5;",
                vec![Stmt::Expr(Expr::Infix(
                    Infix::NotEqual,
                    Box::new(Expr::Literal(Literal::Int(5))),
                    Box::new(Expr::Literal(Literal::Int(5))),
                ))],
            ),
        ];

        for (input, expected) in tests {
            let mut parser = Parser::new(Lexer::new(input));
            let program = parser.parse_program();
            let len = program.len();

            if len == 0 {
                panic!("Program has not enought statments. got={}", len);
            }

            if program != expected {
                panic!("got={:?}. expected={:?}", program, expected);
            }
        }
    }
}
