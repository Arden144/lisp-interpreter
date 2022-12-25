use std::collections::HashMap;

use crate::parser::Expr::*;
use crate::parser::Op::*;
use crate::parser::AST;

pub struct Interpreter {
    heap: HashMap<String, i32>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            heap: HashMap::new(),
        }
    }

    pub fn evaluate(&mut self, node: AST) -> i32 {
        match *node {
            Let { assignments, expr } => {
                for assignment in assignments {
                    let value = self.evaluate(assignment.expr);
                    self.heap.insert(assignment.var, value);
                }
                self.evaluate(expr)
            }
            Math { op, left, right } => match op {
                Add => self.evaluate(left) + self.evaluate(right),
                Mult => self.evaluate(left) * self.evaluate(right),
            },
            Var(id) => *self.heap.get(&id).expect("undefined variable"),
            Literal(n) => n,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Interpreter;
    use crate::lexer::Lexer;
    use crate::parser::AST;

    #[test]
    fn example1() {
        assert_eq!(
            Interpreter::new().evaluate(
                Lexer::from("(let x 2 (mult x (let x 3 y 4 (add x y))))").collect::<AST>()
            ),
            14
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            Interpreter::new().evaluate(Lexer::from("(let x 3 x 2 x)").collect::<AST>()),
            2
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            Interpreter::new()
                .evaluate(Lexer::from("(let x 1 y 2 x (add x y) (add x y))").collect::<AST>()),
            5
        );
    }
}
