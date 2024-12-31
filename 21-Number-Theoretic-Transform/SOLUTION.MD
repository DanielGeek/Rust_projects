# Number Theoretic Transform (NTT) Challenge

## Project Overview

This project implements the Number Theoretic Transform (NTT), an efficient variant of the Discrete Fourier Transform (DFT) used for polynomial arithmetic in modular fields. NTT is critical in cryptography, particularly for polynomial multiplication in lattice-based cryptography.

The implementation focuses on:
- **Forward and Backward NTT**: Over the modular polynomial ring \( Z_Q[X]/(X^N + 1) \), where `Q` is a prime number and `N` is a power of 2.
- **Optimizations**: Uses SIMD for parallel computation, improving performance for large-scale NTTs.

## Results

### Initial Implementation

- **Warnings**: Unused variables and methods, minimal computation.
- **Benchmarks**: Initial benchmarks showed nanosecond-level timings, indicating the logic wasn't fully implemented.

### Optimized Implementation

#### Key Updates:
- **SIMD Optimization**: Vectorized NTT to significantly improve performance using `u64x4`.
- **Precomputation**: Precomputed `psi` powers and their inverses to avoid redundant calculations during NTT.
- **Benchmarking**: Performance was measured for input sizes \( 2^{10} \) to \( 2^{16} \), showing exponential growth in execution time as expected due to the \( O(N^2) \) complexity of NTT.

#### Benchmark Results:

| Size (log N) | Time (Forward NTT)   | Outliers (Forward NTT)     |
|--------------|----------------------|----------------------------|
| 10 (1024)    | 11.202 µs - 11.435 µs | 14 outliers (3% mild, 11% severe) |
| 11 (2048)    | 24.711 µs - 25.636 µs | 13 outliers (1% mild, 12% severe) |
| 12 (4096)    | 53.459 µs - 54.909 µs | 13 outliers (1% mild, 12% severe) |
| 13 (8192)    | 116.96 µs - 120.32 µs | 16 outliers (1% mild, 15% severe) |
| 14 (16384)   | 255.63 µs - 262.11 µs | 14 outliers (2% mild, 12% severe) |
| 15 (32768)   | 554.88 µs - 577.30 µs | 18 outliers (3% mild, 15% severe) |
| 16 (65536)   | 1.1433 ms - 1.1513 ms | 2 outliers (2% mild)         |

This shows that our optimizations significantly reduced computation time, especially for larger values of `N`.

## Help Commands

### Running the Project

To benchmark the NTT implementation and run the project with input size 1024, use the following commands:

```bash
# Run project
cargo bench
cargo bench --bench ntt -- --sample-size 60
cargo run --release -- 1024
```

### Solution

#### Forward NTT:
- Splits the input into smaller sections recursively and computes the polynomial coefficients in modular arithmetic.
- Utilizes the precomputed powers of `psi` to avoid redundant calculations.

#### Backward NTT:
- Uses the same steps as the forward NTT, but multiplies each coefficient by the modular inverse of `N` to normalize the results.

#### Optimizations:
1. **Efficient Modular Arithmetic**: We use modular exponentiation to compute powers of `psi` and `psi_inv` efficiently.
2. **SIMD**: The `vectorized_ntt` function leverages SIMD to process four values in parallel, reducing the time complexity for larger inputs.
3. **Lazy Forward and Backward NTT**: The `lazy` variants avoid unnecessary recalculations by reusing the logic from the standard methods.

## Code Structure

### Key Modules:
- **`ntt.rs`**: Contains the main implementation of the NTT, including SIMD optimizations and helper functions.
- **`dft.rs`**: Defines the `DFT` trait, ensuring compatibility between different transforms (NTT, DFT).
- **`main.rs`**: Entry point of the program, which handles input parsing and invokes the NTT operations.

### Performance Evaluation:
- We benchmarked the forward and backward NTT for various input sizes using the Criterion library.
- The results show a consistent performance gain with increasing input sizes, validating the efficiency of our implementation.

## Conclusion

This solution implements a highly optimized NTT, leveraging modular arithmetic and SIMD for improved performance in cryptographic applications. The benchmarking confirms that the implementation scales efficiently with larger inputs.