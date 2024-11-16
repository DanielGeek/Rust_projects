use std::io::Write;
use num_bigint::BigInt;
use crate::hash_functions::HashFunction;
use crate::r1cs::{Operation, R1CS, Variable};

pub enum Gate {
  Add(usize, usize, usize), // Add: input1, input2, output
  Mul(usize, usize, usize), // Multiply: input1, input2, output
  Hash(usize, usize, usize), // Hash: input1, input2, output
}

pub struct Circuit {
  hash_function: Option<Box<dyn HashFunction>>,
  inputs: Vec<BigInt>,
  gates: Vec<Gate>,
  outputs: Vec<BigInt>,
}

impl Circuit {
  pub fn new(hash_function: Option<Box<dyn HashFunction>>) -> Self {
    Circuit {
      hash_function,
      inputs: Vec::new(),
      gates: Vec::new(),
      outputs: Vec::new(),
    }
  }

  pub fn add_input(&mut self, value: BigInt) -> usize {
    let index = self.inputs.len();
    self.inputs.push(value);
    index
  }

  pub fn add_gate(&mut self, gate: Gate) {
    self.gates.push(gate);
  }

  pub fn set_output(&mut self, value: BigInt) {
    self.outputs.push(value);
  }

  // Apply the hash function, defaulting to a simple operation if none is provided
  pub fn apply_hash(&self, a: &BigInt, b: &BigInt) -> BigInt {
    if let Some(ref hash_function) = self.hash_function {
      hash_function.hash(a, b)
    } else {
      // Default behavior if no hash function is provided
      a + b // Example fallback hash; use a simple operation or return an error as needed
    }
  }

  // Retrieves an input value by index, if it exists
  pub fn get_input(&self, index: usize) -> Option<&BigInt> {
    self.inputs.get(index)
  }

}
