use big_int_algo::uint::Uint;
use criterion::{criterion_group, Criterion, criterion_main};

fn factorial(n: u64) -> Uint {
    let mut factorial = Uint::from([1]);
    for i in 2..=n {
        factorial *= i;
    }
    factorial
}

fn factorial_benchmark(c: &mut Criterion) {
    let f = factorial(50);
    let s = factorial(60);

    c.bench_function("factorial", |b| b.iter(|| {
        let _ = &f * &s;
        let _ = &s * &f;
    }));
}

criterion_group!(benches, factorial_benchmark);
criterion_main!(benches);