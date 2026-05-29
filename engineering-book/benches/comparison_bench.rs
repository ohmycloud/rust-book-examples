use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};

fn generate_gpu_csv(n: usize) -> String {
    (0..n)
        .map(|i| {
            format!(
                "{i}, Acme Accel-X1-80GB, {}, {:.1}\n",
                30 + i % 20,
                60.0 + i as f64
            )
        })
        .collect()
}

fn bench_parsing_strategies(c: &mut Criterion) {
    let mut group = c.benchmark_group("csv_parsing");

    // Test across different input sizes
    for num_gpus in [1, 8, 32, 64, 128] {
        let input = generate_gpu_csv(num_gpus);

        // Set throughput for bytes-per-second reporting
        group.bench_with_input(
            BenchmarkId::new("split_based", num_gpus),
            &input,
            |b, input| b.iter(|| parse_split(input)),
        );

        group.bench_with_input(
            BenchmarkId::new("nom_based", num_gpus),
            &input,
            |b, input| b.iter(|| parse_nom(input)),
        );
    }
    group.finish();
}

criterion_group!(benches, bench_parsing_strategies);
criterion_main!(benches);
