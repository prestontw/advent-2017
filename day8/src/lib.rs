#[macro_use]
extern crate nom;
use std::collections::HashMap;
mod parse;

enum Operation {
  Increase,
  Decrease,
}
enum Question {
  gt,
  lt,
  eq,
  ne,
  ge,
  le,
}

struct Condition {
  register: String,
  q: Question,
  amount: isize, // this can't be another register, can it?
}

struct Instruction {
  register: String,
  op: Operation,
  amount: isize,
  cond: Condition,
}