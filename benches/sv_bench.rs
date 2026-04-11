#![warn(clippy::pedantic)]
#![warn(unused_results)]

use criterion::{BatchSize, Criterion, Throughput, criterion_group, criterion_main};
use rand::{RngExt, rng};
use std::hint::black_box;

use stabilized_vehicle::{FilterAccGyro, ImuFilterBank};
use vector_quaternion_matrix::Vector3df32;

// see target/criterion/SV/update/report/index.html

fn bench_sv(c: &mut Criterion) {
    let mut group = c.benchmark_group("SV");
    let mut imu_filter_bank = ImuFilterBank::new();
    let delta_t = 0.000_1f32;

    _ = group.throughput(Throughput::Elements(1));

    _ = group.bench_function("update", |b| {
        b.iter_batched(
            || {
                // Setup: Generate two random vectors
                let a1: [f32; 3] = rng().random();
                let a2: [f32; 3] = rng().random();
                let acc = Vector3df32::from(a1);
                let gyro_rps = Vector3df32::from(a2);
                (acc, gyro_rps)
            },
            |(acc, gyro_rps)| {
                black_box(imu_filter_bank.update(black_box(acc), black_box(gyro_rps), black_box(delta_t)))
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

criterion_group!(benches, bench_sv);
criterion_main!(benches);
