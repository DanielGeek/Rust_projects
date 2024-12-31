use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion}; // Import Criterion for benchmarking
use app::dft::ntt::Table; // Import the NTT table implementation
use app::dft::DFT; // Import the DFT trait

/// Benchmark the `forward_inplace` method of the NTT implementation.
fn forward_inplace(c: &mut Criterion) {
    // Create a benchmarking group named "forward_inplace"
    let mut group = c.benchmark_group("forward_inplace");

    // Test for sizes 2^10 (1024) to 2^16 (65536)
    for log_n in 10..=16 {
        let n = 1 << log_n; // Calculate the size of the array as 2^log_n
        let table = Table::new(n); // Initialize the NTT table with the size
        let mut data: Vec<u64> = (0..n as u64).collect(); // Create an array with sequential numbers

        // Benchmark the `forward_inplace` function with the input size `n`
        group.bench_with_input(BenchmarkId::from_parameter(log_n), &log_n, |b, _| {
            b.iter(|| table.forward_inplace(&mut data)) // Perform NTT on the data
        });
    }

    // Finalize the benchmarking group
    group.finish();
}

// Criterion boilerplate: Define the benchmark group and entry point
criterion_group!(benches, forward_inplace);
criterion_main!(benches);
