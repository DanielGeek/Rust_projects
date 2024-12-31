
## Complete implementation of the Number Theoretic Transform (NTT) challenge.

## Project Overview

This project focuses on implementing the Number Theoretic Transform (NTT), an algorithmic variant of the Discrete Fourier Transform (DFT) used for polynomial arithmetic in modular arithmetic domains. The NTT is essential in areas like cryptography, particularly lattice-based cryptography, where efficient modular polynomial multiplication is required.

The primary task is to implement a forward and backward NTT over  Z_Q[X]/(X^N+1) , where:
	•	 Q  is a prime number such that \( Q \equiv 1 \pmod{2N} \),
	•	 N  is a power of 2.

The code implements the functions necessary to perform the forward and backward transformation on a prime number of up to 61 bits, meeting the indicated requirements.

### Explanation of Results

### Initial Results

Warnings in the initial implementation:
	1.	Unused variable a: Indicates placeholder methods forward_inplace_core and backward_inplace_core were empty.
	2.	Fields q and psi never read: Suggests unused values in the Table struct, showing unimplemented logic.
	3.	Benchmarks: Minimal computation occurred, so the time taken was in nanoseconds. This reflects that the implementation was incomplete.

### After Adding First Implementation

Changes led to:
	1.	Warnings Reduced:
	•	Only the field psi and a minor comparison warning remained.
	2.	Benchmarks:
	•	Time for computation increased significantly, reflecting the addition of logic.
	•	Benchmarks for input sizes (2^{10} to 2^{16}):
	•	Performance grew exponentially, as expected, given the quadratic complexity of NTT.

## Solution

#### Introduction

The solution implements forward and backward Number Theoretic Transform (NTT) for a modular polynomial ring. The process optimizes polynomial multiplication and is crucial in cryptographic applications.

Steps
	1.	Forward NTT:
	•	Divides input into smaller sections recursively.
	•	Multiplies coefficients using modular arithmetic.
	2.	Backward NTT:
	•	Applies the forward transform.
	•	Multiplies each coefficient by the modular inverse of the array size.

Optimization Techniques
	1.	Efficient Modular Arithmetic:
	•	Modular exponentiation for power computations.
	•	Reduces large modular multiplications.
	2.	Iterative Approach:
	•	Avoids recursion to enhance performance.

### Help Commands
```bash
# Run projec
cargo bench
cargo run --release -- 1024
cargo bench --bench ntt -- --sample-size 60
```
