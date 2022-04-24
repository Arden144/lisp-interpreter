#![allow(dead_code)]

use std::{iter::Enumerate, str::Chars};

#[derive(Debug, PartialEq, Eq)]
pub enum Delim {
    Open,
    Close,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Keyword {
    Let,
    Add,
    Mult,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Token<'a> {
    Delim(Delim),
    Keyword(Keyword),
    Ident(&'a str),
    Literal(i32),
}

pub struct Lexer<'a> {
    expr: &'a str,
    chars: Enumerate<Chars<'a>>,
    ch: Option<(usize, char)>,
}

use self::Delim::*;
use self::Keyword::*;
use Token::*;

impl<'a> Lexer<'a> {
    fn skip_whitespace(&mut self) {
        while let Some((_, ch)) = self.ch {
            if !ch.is_whitespace() {
                break;
            }
            self.ch = self.chars.next();
        }
    }

    fn next_delimiter(&mut self) -> Option<Token<'a>> {
        let (_, ch) = self.ch?;
        self.ch = self.chars.next();
        Some(match ch {
            '(' => Delim(Open),
            ')' => Delim(Close),
            _ => panic!("next_delimiter() called on non-delimiter"),
        })
    }

    fn next_literal(&mut self) -> Option<Token<'a>> {
        let (start, _) = self.ch?;
        let mut end = start;
        while let Some((i, ch)) = self.ch {
            if !ch.is_numeric() {
                end = i;
                break;
            }
            self.ch = self.chars.next();
        }
        assert!(end > start, "next_literal() called on non-literal");
        let token = &self.expr[start..end];
        token.parse().ok().map(|n| Literal(n))
    }

    fn next_identifier(&mut self) -> Option<Token<'a>> {
        let (start, _) = self.ch.unwrap();
        let mut end = start;
        while let Some((i, ch)) = self.ch {
            if !ch.is_alphanumeric() {
                end = i;
                break;
            }
            self.ch = self.chars.next();
        }
        assert!(end > start, "next_identifier() called on non-literal");
        let token = &self.expr[start..end];
        Some(match token {
            "let" => Keyword(Let),
            "add" => Keyword(Add),
            "mult" => Keyword(Mult),
            id => Ident(id),
        })
    }
}

impl<'a> From<&'a str> for Lexer<'a> {
    fn from(expr: &'a str) -> Self {
        let mut chars = expr.chars().enumerate();
        let ch = chars.next();
        Self { expr, chars, ch }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.ch.and_then(|(_, ch)| {
            if ch.is_whitespace() {
                self.skip_whitespace();
                return self.next();
            }
            match ch {
                '(' | ')' => self.next_delimiter(),
                ch if ch.is_numeric() => self.next_literal(),
                ch if ch.is_alphabetic() => self.next_identifier(),
                ch => panic!("can't parse '{ch}'"),
            }
        })
    }
}

#[cfg(test)]
mod test {
    use super::{Delim::*, Keyword::*, Lexer, Token, Token::*};

    #[test]
    #[should_panic(expected = "can't parse '!'")]
    fn invalid_language() {
        Lexer::from("(add ! 2)").collect::<Vec<Token>>();
    }

    #[test]
    fn example3() {
        assert_eq!(
            Lexer::from("(let x 2 (mult x (let x 3 y 4 (add x y))))").collect::<Vec<Token>>(),
            vec![
                Delim(Open),
                Keyword(Let),
                Ident("x"),
                Literal(2),
                Delim(Open),
                Keyword(Mult),
                Ident("x"),
                Delim(Open),
                Keyword(Let),
                Ident("x"),
                Literal(3),
                Ident("y"),
                Literal(4),
                Delim(Open),
                Keyword(Add),
                Ident("x"),
                Ident("y"),
                Delim(Close),
                Delim(Close),
                Delim(Close),
                Delim(Close),
            ],
        );
    }
}
