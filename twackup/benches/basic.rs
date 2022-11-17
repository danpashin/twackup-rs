use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use twackup::{package::Package, Parser};

fn bench(c: &mut Criterion) {
    let database = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/databases/real-system-100"
    );
    let parser = Parser::new(database).unwrap();
    let rt = tokio::runtime::Runtime::new().unwrap();

    let b_input = (parser, rt);
    c.bench_with_input(
        BenchmarkId::new("100 packages", "tokio runtime"),
        &b_input,
        |b, (parser, rt)| b.to_async(rt).iter(|| parser.parse::<Package>()),
    );
}

criterion_group!(benches, bench);
criterion_main!(benches);
