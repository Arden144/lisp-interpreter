use crate::lexer::{Delim, Keyword, Token};

/**
 * var: ID
 * letexpr: (LET (var expr)+ expr)
 * mathexpr: ((ADD | MULT) expr expr)
 * expr: letexpr
 *     | mathexpr
 *     | var
 *     | literal
 */

#[derive(Debug, PartialEq, Eq)]
pub struct Assign {
    pub var: String,
    pub expr: Box<Expr>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Op {
    Add,
    Mult,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    Let {
        assignments: Vec<Assign>,
        expr: Box<Expr>,
    },
    Math {
        op: Op,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Var(String),
    Literal(i32),
}

pub type AST = Box<Expr>;

pub struct Parser<'a, T: Iterator<Item = Token<'a>>> {
    iter: T,
    token: Option<Token<'a>>,
}

impl<'a, T: Iterator<Item = Token<'a>>> Parser<'a, T> {
    pub fn new(mut iter: T) -> Self {
        let token = iter.next();
        Self { iter, token }
    }

    pub fn parse(&mut self) -> Box<Expr> {
        self.expr()
    }

    fn peek(&self) -> &Token {
        self.token
            .as_ref()
            .expect("peek() called with all tokens consumed")
    }

    fn take(&mut self) -> Token {
        let token = self
            .token
            .take()
            .expect("take() called with no tokens left");
        self.token = self.iter.next();
        token
    }

    fn letexpr(&mut self) -> Expr {
        let mut assignments = Vec::new();

        while let Token::Ident(_) = self.peek() {
            let var = self.var();
            if let Token::Delim(Delim::Close) = self.peek() {
                return Expr::Let {
                    assignments,
                    expr: Box::new(Expr::Var(var)),
                };
            }
            assignments.push(Assign {
                var,
                expr: self.expr(),
            })
        }

        Expr::Let {
            assignments,
            expr: self.expr(),
        }
    }

    fn comp_expr(&mut self) -> Box<Expr> {
        assert_eq!(self.take(), Token::Delim(Delim::Open));

        let node = if let Token::Keyword(keyword) = self.take() {
            match keyword {
                Keyword::Let => self.letexpr(),
                Keyword::Add => Expr::Math {
                    op: Op::Add,
                    left: self.expr(),
                    right: self.expr(),
                },
                Keyword::Mult => Expr::Math {
                    op: Op::Mult,
                    left: self.expr(),
                    right: self.expr(),
                },
            }
        } else {
            panic!("expr() expected a keyword token");
        };

        assert_eq!(self.take(), Token::Delim(Delim::Close));

        Box::new(node)
    }

    fn var(&mut self) -> String {
        if let Token::Ident(id) = self.take() {
            id.to_owned()
        } else {
            panic!("var() expected an ident token")
        }
    }

    fn literal(&mut self) -> i32 {
        if let Token::Literal(n) = self.take() {
            n
        } else {
            panic!("literal() expected a literal token")
        }
    }

    fn expr(&mut self) -> Box<Expr> {
        match self.peek() {
            Token::Delim(Delim::Open) => self.comp_expr(),
            Token::Ident(_) => Box::new(Expr::Var(self.var())),
            Token::Literal(_) => Box::new(Expr::Literal(self.literal())),
            _ => panic!(
                "expr() expected an opening delim, an ident, or a literal (reading {:?})",
                self.peek()
            ),
        }
    }
}

impl<'a> From<Vec<Token<'a>>> for AST {
    fn from(tokens: Vec<Token<'a>>) -> Self {
        Self::from_iter(tokens)
    }
}

impl<'a> FromIterator<Token<'a>> for AST {
    fn from_iter<T: IntoIterator<Item = Token<'a>>>(iter: T) -> Self {
        Parser::new(iter.into_iter()).parse()
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::Lexer;

    use super::{Assign, Expr, Op, AST};

    #[test]
    fn example1() {
        assert_eq!(
            Lexer::from("(let x 2 (mult x (let x 3 y 4 (add x y))))").collect::<AST>(),
            Box::new(Expr::Let {
                assignments: vec![Assign {
                    var: "x".to_string(),
                    expr: Box::new(Expr::Literal(2))
                }],
                expr: Box::new(Expr::Math {
                    op: Op::Mult,
                    left: Box::new(Expr::Var("x".to_string())),
                    right: Box::new(Expr::Let {
                        assignments: vec![
                            Assign {
                                var: "x".to_string(),
                                expr: Box::new(Expr::Literal(3))
                            },
                            Assign {
                                var: "y".to_string(),
                                expr: Box::new(Expr::Literal(4))
                            }
                        ],
                        expr: Box::new(Expr::Math {
                            op: Op::Add,
                            left: Box::new(Expr::Var("x".to_string())),
                            right: Box::new(Expr::Var("y".to_string()))
                        })
                    })
                })
            })
        )
    }
}
