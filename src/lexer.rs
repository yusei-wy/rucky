use crate::token::Token;

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        let mut l = Lexer {
            input: input.to_string(),
            position: 0,
            read_position: 0,
            ch: 0,
        };
        l.read_char();
        l
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> u8 {
        if self.read_position >= self.input.len() {
            0
        } else {
            self.input.as_bytes()[self.read_position]
        }
    }

    pub fn next_token(&mut self) -> Token {
        let tok: Token;

        self.skip_whitespace();

        match self.ch {
            b'=' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    tok = Token::Equal;
                } else {
                    tok = Token::Assign;
                }
            }
            b'+' => tok = Token::Plus,
            b'-' => tok = Token::Minus,
            b'!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    tok = Token::NotEqual;
                } else {
                    tok = Token::Bang;
                }
            }
            b'*' => tok = Token::Asterisk,
            b'/' => tok = Token::Slash,
            b'<' => tok = Token::Lt,
            b'>' => tok = Token::Gt,
            b',' => tok = Token::Comma,
            b';' => tok = Token::Semicolon,
            b'(' => tok = Token::Lparen,
            b')' => tok = Token::Rparen,
            b'{' => tok = Token::Lbrace,
            b'}' => tok = Token::Rbrace,
            0 => tok = Token::EOF,
            _ => {
                if is_letter(&self.ch) {
                    tok = self.consume_identifier();
                    return tok;
                } else if is_digit(&self.ch) {
                    tok = self.consume_number();
                    return tok;
                } else {
                    tok = Token::Illegal
                }
            }
        }

        self.read_char();

        tok
    }

    fn read_identifier(&mut self) -> &str {
        let position = self.position;
        while is_letter(&self.ch) {
            self.read_char();
        }
        &self.input[position..self.position]
    }

    fn read_number(&mut self) -> &str {
        let position = self.position;
        while is_digit(&self.ch) {
            self.read_char();
        }
        &self.input[position..self.position]
    }

    fn consume_identifier(&mut self) -> Token {
        let literal = self.read_identifier();
        match literal {
            "fn" => return Token::Fn,
            "let" => return Token::Let,
            "true" => return Token::True,
            "false" => return Token::False,
            "if" => return Token::If,
            "else" => return Token::Else,
            "return" => return Token::Return,
            _ => return Token::Ident(literal.to_string()),
        }
    }

    fn consume_number(&mut self) -> Token {
        Token::Int(self.read_number().parse::<i64>().unwrap())
    }

    fn skip_whitespace(&mut self) {
        while self.ch == b' ' || self.ch == b'\t' || self.ch == b'\n' || self.ch == b'\r' {
            self.read_char();
        }
    }
}

fn is_letter(ch: &u8) -> bool {
    match ch {
        b'a'...b'z' | b'A'...b'Z' | b'_' => return true,
        _ => return false,
    }
}

fn is_digit(ch: &u8) -> bool {
    match ch {
        b'0'...b'9' => return true,
        _ => return false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        const INPUT: &str = "let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
    return true;
} else {
    return false;
}

10 == 10;
10 != 9;
";

        let types: Vec<Token> = vec![
            Token::Let,
            Token::Ident(String::from("five")),
            Token::Assign,
            Token::Int(5),
            Token::Semicolon,
            //
            Token::Let,
            Token::Ident(String::from("ten")),
            Token::Assign,
            Token::Int(10),
            Token::Semicolon,
            //
            Token::Let,
            Token::Ident(String::from("add")),
            Token::Assign,
            Token::Fn,
            Token::Lparen,
            Token::Ident(String::from("x")),
            Token::Comma,
            Token::Ident(String::from("y")),
            Token::Rparen,
            Token::Lbrace,
            //
            Token::Ident(String::from("x")),
            Token::Plus,
            Token::Ident(String::from("y")),
            Token::Semicolon,
            //
            Token::Rbrace,
            Token::Semicolon,
            //
            Token::Let,
            Token::Ident(String::from("result")),
            Token::Assign,
            Token::Ident(String::from("add")),
            Token::Lparen,
            Token::Ident(String::from("five")),
            Token::Comma,
            Token::Ident(String::from("ten")),
            Token::Rparen,
            Token::Semicolon,
            //
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int(5),
            Token::Semicolon,
            //
            Token::Int(5),
            Token::Lt,
            Token::Int(10),
            Token::Gt,
            Token::Int(5),
            Token::Semicolon,
            //
            Token::If,
            Token::Lparen,
            Token::Int(5),
            Token::Lt,
            Token::Int(10),
            Token::Rparen,
            Token::Lbrace,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::Rbrace,
            Token::Else,
            Token::Lbrace,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::Rbrace,
            //
            Token::Int(10),
            Token::Equal,
            Token::Int(10),
            Token::Semicolon,
            //
            Token::Int(10),
            Token::NotEqual,
            Token::Int(9),
            Token::Semicolon,
            Token::EOF,
        ];

        let mut lexer = Lexer::new(INPUT);

        for (i, token) in types.iter().enumerate() {
            let tok = lexer.next_token();

            if tok != *token {
                panic!(
                    "types[{}] - token wrong. expected={:?}, got={:?}",
                    i, token, tok
                );
            }
        }
    }
}
