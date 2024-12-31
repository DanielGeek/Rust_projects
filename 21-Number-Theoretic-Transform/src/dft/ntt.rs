use crate::dft::DFT; // Import the DFT trait for compatibility
use std::simd::u64x4;

/// Represents a table for NTT operations.
pub struct Table<O> {
    q: O,                   // NTT-friendly prime modulus
    psi: O,                 // n-th root of unity
    psi_powers: Vec<O>,     // Precomputed powers of psi for the forward transform
    psi_inv_powers: Vec<O>, // Precomputed powers of psi^-1 for the inverse transform
    n_inv: O,               // Modular inverse of N (size of the array)
}

impl Table<u64> {
    /// Initialize a new NTT table with a given size.
    pub fn new(n: usize) -> Self {
        let q = 0x1fffffffffe00001u64; // 61-bit prime modulus
        let psi = 0x15eb043c7aa2b01fu64; // 2^17-th root of unity
        let psi_inv = mod_inverse(psi, q); // Modular inverse of psi
        let n_inv = mod_inverse(n as u64, q); // Modular inverse of n

        // Precompute powers of psi and psi^-1
        let psi_powers = compute_powers(psi, n, q);
        let psi_inv_powers = compute_powers(psi_inv, n, q);

        // Return the initialized table
        Self {
            q,
            psi,
            psi_powers,
            psi_inv_powers,
            n_inv,
        }
    }

    fn vectorized_ntt(&self, a: &mut [u64], powers: &[u64], q: u64) {
        let n = a.len();
        let mut len = 1;

        while len < n {
            let step = len * 2;
            for start in (0..n).step_by(step) {
                for i in 0..len {
                    let u = a[start + i]; // First term
                    let v = (a[start + i + len] * powers[n / step * i]) % q; // Second term

                    // Check if indexes are within limits
                    if start + i + 3 < n {
                        // Use SIMD to update values ​​in parallel
                        let u_simd = u64x4::from([u, a[start + i + 1], a[start + i + 2], a[start + i + 3]]);
                        let v_simd = u64x4::from([v, v, v, v]);

                        let u_plus_v = u_simd + v_simd;
                        let u_minus_v = u_simd - v_simd;

                        // We save the processed results in parallel
                        a[start + i] = u_plus_v[0] % q;
                        a[start + i + len] = (u_minus_v[0] + q) % q;
                    } else {
                        // In case it is not possible to use SIMD, proceed without SIMD
                        a[start + i] = (u + v) % q;
                        a[start + i + len] = (u + q - v) % q;
                    }
                }
            }
            len = step; // We double the length for the next level
        }
    }
}

impl DFT<u64> for Table<u64> {
    fn forward_inplace(&self, a: &mut [u64]) {
        self.vectorized_ntt(a, &self.psi_powers, self.q); // Optimization with SIMD
    }

    /// Perform the forward NTT in place
    // fn forward_inplace(&self, a: &mut [u64]) {
    //     ntt(a, &self.psi_powers, self.q); // Call NTT with forward powers
    // }

    /// Perform the lazy forward NTT (for optimization)
    fn forward_inplace_lazy(&self, a: &mut [u64]) {
        ntt(a, &self.psi_powers, self.q); // Reuse forward logic
    }

    fn backward_inplace(&self, a: &mut [u64]) {
        self.vectorized_ntt(a, &self.psi_inv_powers, self.q);
        for val in a.iter_mut() {
            *val = (*val * self.n_inv) % self.q;
        }
    }

    /// Perform the backward (inverse) NTT in place
    // fn backward_inplace(&self, a: &mut [u64]) {
    //     ntt(a, &self.psi_inv_powers, self.q); // Call NTT with inverse powers
    //     for val in a.iter_mut() {
    //         *val = (*val * self.n_inv) % self.q; // Normalize results
    //     }
    // }

    /// Perform the lazy backward NTT (for optimization)
    fn backward_inplace_lazy(&self, a: &mut [u64]) {
        self.backward_inplace(a); // Reuse inverse logic
    }
}

/// Compute the modular inverse of `a` modulo `q`.
fn mod_inverse(a: u64, q: u64) -> u64 {
    let mut t: u64 = 0;
    let mut new_t = 1;
    let mut r = q;
    let mut new_r = a;

    while new_r != 0 {
        let quotient = r / new_r;
        t = t.wrapping_sub(quotient.wrapping_mul(new_t));
        r = r.wrapping_sub(quotient.wrapping_mul(new_r));
        std::mem::swap(&mut t, &mut new_t);
        std::mem::swap(&mut r, &mut new_r);
    }

    if r > 1 {
        panic!("Number is not invertible");
    }
    if t > q {
        t = t.wrapping_sub(q);
    }
    t
}

/// Precompute powers of a base modulo q.
fn compute_powers(base: u64, n: usize, q: u64) -> Vec<u64> {
    let mut powers = vec![1u64; n];
    for i in 1..n {
        powers[i] = (powers[i - 1] * base) % q;
    }
    powers
}

/// Perform the NTT algorithm in place.
fn ntt(a: &mut [u64], powers: &[u64], q: u64) {
    let n = a.len();
    let mut len = 1;

    while len < n {
        let step = len * 2;
        for start in (0..n).step_by(step) {
            for i in 0..len {
                let u = a[start + i]; // First term
                let v = (a[start + i + len] * powers[n / step * i]) % q; // Second term with powers
                a[start + i] = (u + v) % q; // Update first term
                a[start + i + len] = (u + q - v) % q; // Update second term
            }
        }
        len = step; // Double the length for the next level
    }
}
