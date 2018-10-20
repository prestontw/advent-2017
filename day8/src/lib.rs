#[macro_use]
extern crate nom;
use std::collections::HashMap;
mod parse;

#[derive(PartialEq, Debug)]
enum Operation {
  Increase,
  Decrease,
}
#[derive(PartialEq, Debug)]
enum Question {
  gt,
  lt,
  eq,
  ne,
  ge,
  le,
}

#[derive(PartialEq, Debug)]
struct Condition {
  register: String,
  q: Question,
  amount: isize, // this can't be another register, can it?
}

#[derive(PartialEq, Debug)]
struct Instruction {
  register: String,
  op: Operation,
  amount: isize,
  cond: Condition,
}

fn get_value_in_register(r: &str, store: &HashMap<String, isize>) -> isize {
  *store.get(r).unwrap_or(&0)
}

fn function_for_operation(op: Operation) -> Box<Fn(isize, isize) -> isize> {
  match op {
    Operation::Increase => Box::new(|a, b| a + b),
    Operation::Decrease => Box::new(|a, b| a - b),
  }
}
// condition is true for this
fn set_value_in_register(
  r: &str,
  mut store: HashMap<String, isize>,
  op: Operation,
  amount: isize,
) -> HashMap<String, isize> {
  let starting_value = get_value_in_register(r, &store);
  let new_value = function_for_operation(op)(starting_value, amount);
  store.insert(r.to_string(), new_value);
  store
}

#[test]
fn val_not_in() {
  let store = HashMap::new();
  assert_eq!(
    set_value_in_register("a", store, Operation::Increase, 10),
    vec![("a".to_string(), 10)].iter().cloned().collect()
  );
}

#[test]
fn val_in() {
  let store = vec![("a".to_string(), 5)].iter().cloned().collect();
  assert_eq!(
    set_value_in_register("a", store, Operation::Decrease, -10),
    vec![("a".to_string(), 15)].iter().cloned().collect()
  );
}
