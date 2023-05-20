use big_int_algo::uint::Uint;
use criterion::{criterion_group, Criterion, criterion_main};

fn factorial_benchmark(c: &mut Criterion) {
    const COUNT: u64 = 30;

    c.bench_function("factorial", |b| b.iter(|| {
        let mut factorial = Uint::from([1]);
        for i in 1..COUNT+1 {
            factorial *= i;
        }
    }));
}

criterion_group!(benches, factorial_benchmark);
criterion_main!(benches);