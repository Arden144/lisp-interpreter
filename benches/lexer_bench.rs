use criterion::{black_box, criterion_group, criterion_main, Criterion};

use rust_playground::lexer::{Lexer, Token};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("lex", |b| {
        b.iter(|| {
            black_box(
                Lexer::from("(let x 2 (mult x (let x 3 y 4 (add x y))))").collect::<Vec<Token>>(),
            )
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
