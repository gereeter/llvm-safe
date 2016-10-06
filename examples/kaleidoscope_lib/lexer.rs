use std::iter::Peekable;
use kaleidoscope_lib::util::ConsumeWhileExt;

#[derive(Clone, PartialEq, Debug)]
pub enum Token {
    // commands
    Def,
    Extern,
    If,
    Then,
    Else,

    // primary
    Identifier(String),
    Number(f64),
    Other(char)
}

pub struct Tokens<I: Iterator> {
    inner: Peekable<I>
}

impl<I: Iterator> Tokens<I> {
    pub fn new(iter: I) -> Tokens<I> {
        Tokens {
            inner: iter.peekable()
        }
    }
}

impl<I: Iterator<Item=char>> Iterator for Tokens<I> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        for _ in self.inner.consume_while(|c| c.is_whitespace()) { }

        let first_char = match self.inner.peek().cloned() {
            Some(c) => c,
            None => return None
        };

        Some(
            if first_char.is_alphabetic() {
                let ident: String = self.inner.consume_while(|c| c.is_alphanumeric()).collect();

                if ident == "def" {
                    Token::Def
                } else if ident == "extern" {
                    Token::Extern
                } else if ident == "if" {
                    Token::If
                } else if ident == "then" {
                    Token::Then
                } else if ident == "else" {
                    Token::Else
                } else {
                    Token::Identifier(ident)
                }
            } else if first_char.is_digit(10) || first_char == '.' {
                let num_str: String = self.inner.consume_while(|c| c.is_digit(10) || *c == '.').collect();

                Token::Number(num_str.parse().unwrap())
            } else {
                Token::Other(self.inner.next().unwrap())
            }
        )
    }
}
