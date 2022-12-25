use std::{
    fmt::Debug,
    iter::{Enumerate, Peekable},
    str::Chars,
};

use crate::adapter::PeekWhileExt;

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
    chars: Peekable<Enumerate<Chars<'a>>>,
}

use self::Delim::*;
use self::Keyword::*;
use Token::*;

impl<'a> Lexer<'a> {
    fn take_while(&mut self, mut predicate: impl FnMut(char) -> bool) -> Option<&'a str> {
        let start = self.chars.peek()?.0;
        let len = self
            .chars
            .by_ref()
            .peek_while(|strlen| predicate(strlen.1))
            .count();
        Some(&self.expr[start..(start + len)])
    }

    fn skip_whitespace(&mut self) {
        self.take_while(|ch| ch.is_whitespace());
    }

    fn next_delimiter(&mut self) -> Option<Token<'a>> {
        self.chars.next().map(|ch| match ch.1 {
            '(' => Delim(Open),
            ')' => Delim(Close),
            _ => panic!("next_delimiter() called on non-delimiter"),
        })
    }

    fn next_literal(&mut self) -> Option<Token<'a>> {
        self.take_while(|ch| ch.is_numeric())?
            .parse()
            .ok()
            .map(Literal)
    }

    fn next_identifier(&mut self) -> Option<Token<'a>> {
        let token = self.take_while(|ch| ch.is_alphanumeric())?;
        match token {
            "let" => Some(Keyword(Let)),
            "add" => Some(Keyword(Add)),
            "mult" => Some(Keyword(Mult)),
            id => Some(Ident(id)),
        }
    }
}

impl<'a> From<&'a str> for Lexer<'a> {
    fn from(expr: &'a str) -> Self {
        Self {
            expr,
            chars: expr.chars().enumerate().peekable(),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let ch = self.chars.peek()?.1;
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
    }
}

#[cfg(test)]
mod test {
    use super::{Delim::*, Keyword::*, Lexer, Token, Token::*};

    #[test]
    #[should_panic(expected = "can't parse '!'")]
    fn invalid_language() {
        Lexer::from("(add ! 2)").for_each(drop);
    }

    #[test]
    fn example1() {
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
