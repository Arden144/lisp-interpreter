#![allow(dead_code)]

use crate::lexer::Token;

/**
 * ident: [a-zA-z]+
 * bind: ident expr
 * op: add | mult
 * letexpr: (let bind+ expr)
 * mathexpr: (op expr expr)
 * expr: letexpr | mathexpr | ident | literal
 */

#[derive(Debug, PartialEq, Eq)]
struct Binding {
    ident: String,
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
        bindings: Vec<Binding>,
        expr: Box<Expr>,
    },
    Math {
        operator: Op,
        operand1: Box<Expr>,
        operand2: Box<Expr>,
    },
    Ident(String),
    Literal(i32),
}

fn parse<'a>(tokens: impl Iterator<Item = Token<'a>>) -> Box<Expr> {
    unimplemented!()
}

#[cfg(test)]
mod test {
    use crate::lexer::eval;

    use super::{parse, Binding, Expr, Op};

    #[test]
    fn example3() {
        assert_eq!(
            parse(eval("(let x 2 (mult x (let x 3 y 4 (add x y))))")),
            Box::new(Expr::Let {
                bindings: vec![Binding {
                    ident: "x".to_string(),
                    expr: Expr::Literal(2)
                }],
                expr: Box::new(Expr::Math {
                    operator: Op::Mult,
                    operand1: Box::new(Expr::Ident("x".to_string())),
                    operand2: Box::new(Expr::Let {
                        bindings: vec![
                            Binding {
                                ident: "x".to_string(),
                                expr: Expr::Literal(3)
                            },
                            Binding {
                                ident: "y".to_string(),
                                expr: Expr::Literal(4)
                            }
                        ],
                        expr: Box::new(Expr::Math {
                            operator: Op::Add,
                            operand1: Box::new(Expr::Ident("x".to_string())),
                            operand2: Box::new(Expr::Ident("y".to_string()))
                        })
                    })
                })
            })
        )
    }
}
