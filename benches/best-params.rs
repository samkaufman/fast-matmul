#![allow(non_snake_case)]

use itertools::iproduct;

use fast_matmul::{fast::Param, *};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn fast_matmul(size: usize, param: Param) {
    let A = Matrix::serial_new(size, size);
    let B = Matrix::serial_new(size, size);
    let mut C = Matrix::new(size, size);
    fast::matmul(size, size, size, &A, &B, &mut C, param);
}

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Matmul");
    group.sample_size(10); // 10 is minimum required; default is 100

    let range = 0..=11; // 2^(0..=11)
    for (nc_exp, mc_exp, kc_exp) in iproduct!(range.clone(), range.clone(), range) {
        let param = Param::new(
            2_usize.pow(nc_exp),
            2_usize.pow(mc_exp),
            2_usize.pow(kc_exp),
        );
        // group.throughput(Throughput::Elements(parameter as u64));
        group.bench_with_input(BenchmarkId::new("fast", param), &param, |b, par| {
            b.iter(|| fast_matmul(2048, *par))
        });
        // group.bench_with_input(BenchmarkId::new("naive", parameter), parameter, |b, par| {
        //     b.iter(|| naive_matmul(*par))
        // });
    }
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
