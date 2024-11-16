use num_bigint::BigInt;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Write;

#[derive(Clone, Serialize, Deserialize)]
pub struct Variable {
  pub index: usize,
  pub value: BigInt,
}

#[derive(Serialize, Deserialize)]
pub enum Operation {
  Add,
  Mul,
  Hash,
}

#[derive(Serialize, Deserialize)]
pub struct Constraint {
  pub left: Vec<(Variable, BigInt)>,
  pub right: Vec<(Variable, BigInt)>,
  pub output: Vec<(Variable, BigInt)>,
  pub operation: Operation,
}

#[derive(Serialize, Deserialize)]
pub struct R1CS {
  pub variables: Vec<Variable>,
  pub Constraints: Vec<Constraint>,
}
