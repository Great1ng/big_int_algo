use big_int_algo::uint::Uint;
use criterion::{criterion_group, Criterion, criterion_main};

fn addition_benchmark(c: &mut Criterion) {
    let iterations = 300;

    c.bench_function("addition", |b| b.iter(|| {
        let mut first = Uint::from([1]);
        let mut second = first.clone();
        for _ in 0..iterations {
            let third = first + &second;
            first = second;
            second = third;
        }
    }));
}

criterion_group!(benches, addition_benchmark);
criterion_main!(benches);