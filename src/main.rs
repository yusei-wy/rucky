extern crate rucky;

use std::io::{stdin, stdout, Write};

use rucky::lexer::Lexer;
use rucky::token::Token;

const PROMPT: &str = ">> ";

fn get_input() -> String {
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn main() {
    loop {
        print!("{}", PROMPT);
        let _ = stdout().flush();

        let line: String = get_input();

        if line == "exit" {
            break;
        }

        let mut l = Lexer::new(&line);

        loop {
            let tok = l.next_token();
            if tok == Token::EOF {
                break;
            }
            println!("{:?}", tok);
        }
    }
}
