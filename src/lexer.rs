#![allow(dead_code)]

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

impl<'a> From<&'a str> for Token<'a> {
    fn from(s: &'a str) -> Self {
        use self::Keyword::*;
        use Token::*;
        match s {
            "let" => Keyword(Let),
            "add" => Keyword(Add),
            "mult" => Keyword(Mult),
            s => match s.parse::<i32>() {
                Ok(n) => Literal(n),
                Err(_) => Ident(s),
            },
        }
    }
}

pub fn eval(expr: &str) -> impl Iterator<Item = Token> {
    use self::Delim::*;
    use Token::*;

    let mut start = 0;
    let mut end = 0;

    let tokenize = move |ch| {
        let s = &expr[start..end];
        start += 1;
        end += 1;

        let mut commit = |token| {
            [
                (!s.is_empty()).then(|| {
                    start = end;
                    Token::from(s)
                }),
                token,
            ]
        };

        match ch {
            '(' => commit(Some(Delim(Open))),
            ')' => commit(Some(Delim(Close))),
            ' ' => commit(None),
            _ => {
                start -= 1;
                [None, None]
            }
        }
    };

    expr.chars().flat_map(tokenize).filter_map(|t| t)
}

#[cfg(test)]
mod test {
    use super::{eval, Delim::*, Keyword::*, Token, Token::*};

    #[test]
    fn example3() {
        assert_eq!(
            eval("(let x 2 (mult x (let x 3 y 4 (add x y))))").collect::<Vec<Token>>(),
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
