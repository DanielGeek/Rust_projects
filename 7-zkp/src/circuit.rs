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

}
