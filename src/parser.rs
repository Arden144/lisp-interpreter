#![allow(dead_code)]

use crate::lexer::Token;

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
struct Assign {
    var: String,
    expr: Expr,
}

#[derive(Debug, PartialEq, Eq)]
enum Op {
    Add,
    Mult,
}

#[derive(Debug, PartialEq, Eq)]
enum Expr {
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

type AST = Box<Expr>;

impl<'a> From<Vec<Token<'a>>> for AST {
    fn from(tokens: Vec<Token<'a>>) -> Self {
        Self::from_iter(tokens)
    }
}

impl<'a> FromIterator<Token<'a>> for AST {
    fn from_iter<T: IntoIterator<Item = Token<'a>>>(iter: T) -> Self {
        for token in iter {
            println!("{token:?}")
        }
        Box::new(Expr::Literal(0))
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::Lexer;

    use super::{Assign, Expr, Op, AST};

    #[test]
    fn example3() {
        assert_eq!(
            Lexer::from("(let x 2 (mult x (let x 3 y 4 (add x y))))").collect::<AST>(),
            Box::new(Expr::Let {
                assignments: vec![Assign {
                    var: "x".to_string(),
                    expr: Expr::Literal(2)
                }],
                expr: Box::new(Expr::Math {
                    op: Op::Mult,
                    left: Box::new(Expr::Var("x".to_string())),
                    right: Box::new(Expr::Let {
                        assignments: vec![
                            Assign {
                                var: "x".to_string(),
                                expr: Expr::Literal(3)
                            },
                            Assign {
                                var: "y".to_string(),
                                expr: Expr::Literal(4)
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
