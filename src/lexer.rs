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

    pub fn next_token(&mut self) -> Token {
        let tok: Token;
        match self.ch {
            b'+' => tok = Token::Plus,
            b'=' => tok = Token::Assign,
            b',' => tok = Token::Comma,
            b';' => tok = Token::Semicolon,
            b'(' => tok = Token::Lparen,
            b')' => tok = Token::Rparen,
            b'{' => tok = Token::Lbrace,
            b'}' => tok = Token::Rbrace,
            0 => tok = Token::EOF,
            _ => tok = Token::Illegal(self.ch.to_string()),
        }

        self.read_char();

        tok
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        const INPUT: &str = "=+(){},;";

        let types: Vec<Token> = vec![
            Token::Assign,
            Token::Plus,
            Token::Lparen,
            Token::Rparen,
            Token::Lbrace,
            Token::Rbrace,
            Token::Comma,
            Token::Semicolon,
        ];

        let mut lexer = Lexer::new(INPUT);

        for (i, token) in types.iter().enumerate() {
            let tok = lexer.next_token();

            if tok != *token {
                panic!(
                    "types[{}] - token wrong. expected={:?}, got={:?}",
                    i, tok, token
                );
            }
        }
    }
}
