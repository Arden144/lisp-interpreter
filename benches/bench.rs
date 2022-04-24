use criterion::{black_box, criterion_group, criterion_main, Criterion};

use lisp_interpreter::lexer::Lexer;
use lisp_interpreter::parser::AST;

pub fn bench_lexer(c: &mut Criterion) {
    let expr = "(let x 2 (mult x (let x 3 y 4 (add x y))))";

    c.bench_function("lex", |b| {
        b.iter(|| black_box(Lexer::from(expr).for_each(drop)));
    });
}

pub fn bench_both(c: &mut Criterion) {
    let expr = "(let x 2 (mult x (let x 3 y 4 (add x y))))";

    c.bench_function("lex and parse", |b| {
        b.iter(|| black_box(Lexer::from(expr).collect::<AST>()));
    });
}

criterion_group!(benches, bench_lexer, bench_both);
criterion_main!(benches);
