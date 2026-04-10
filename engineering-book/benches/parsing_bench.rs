use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};

/// Data type for parsed GPU information
#[derive(Debug, Clone, PartialEq)]
struct GpuInfo {
    index: u32,
    name: String,
    temp_c: u32,
    power_w: f64,
}

/// The function under test - simulate parsing device-query CSV output
fn parse_gpu_csv(input: &str) -> Vec<GpuInfo> {
    input
        .lines()
        .filter(|line| !line.starts_with('#'))
        .filter_map(|line| {
            let fields: Vec<&str> = line.split(", ").collect();
            if fields.len() >= 4 {
                Some(GpuInfo {
                    index: fields[0].parse().ok()?,
                    name: fields[1].to_string(),
                    temp_c: fields[2].parse().ok()?,
                    power_w: fields[3].parse().ok()?,
                })
            } else {
                None
            }
        })
        .collect()
}

fn bench_parse_gpu_csv(c: &mut Criterion) {
    // Representative test data
    let small_input = "0, Acme Accel-V1-80GB, 32, 65.5\n\
                       1, Acme Accel-V1-80GB, 34, 67.2\n";
    let large_input = (0..64)
        .map(|i| {
            format!(
                "{i}, Acme Accel-V1-80GB, {}, {:.1}\n",
                30 + i % 20,
                60.0 + i as f64
            )
        })
        .collect::<String>();

    c.bench_function("parse_2_gpus", |b| {
        b.iter(|| parse_gpu_csv(black_box(small_input)))
    });

    c.bench_function("parse_64_gpus", |b| {
        b.iter(|| parse_gpu_csv(black_box(large_input.as_str())))
    });
}

criterion_group!(benches, bench_parse_gpu_csv);
criterion_main!(benches);
