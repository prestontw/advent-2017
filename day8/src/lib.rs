#[macro_use]
extern crate nom;
use nom::types::CompleteStr;
use std::collections::HashMap;
mod parse;

#[derive(PartialEq, Debug)]
pub enum Operation {
  Increase,
  Decrease,
}
#[derive(PartialEq, Debug)]
pub enum Question {
  Gt,
  Lt,
  Eq,
  Ne,
  Ge,
  Le,
}

#[derive(PartialEq, Debug)]
pub struct Condition {
  register: String,
  q: Question,
  amount: isize, // this can't be another register, can it?
}

#[derive(PartialEq, Debug)]
pub struct Instruction {
  register: String,
  op: Operation,
  amount: isize,
  cond: Condition,
}

pub fn biggest_register(h: &HashMap<String, isize>) -> Option<&isize> {
  h.values().max().clone()
}

pub fn biggest_register_ever<'a, I>(is: I) -> isize
where
  I: IntoIterator<Item = &'a Instruction>,
{
  is.into_iter()
    .fold(
      (HashMap::new(), 0),
      |(store, max): (HashMap<String, isize>, isize), cur| {
        let new_store = eval_instruction(cur, store);
        let new_potential = biggest_register(&new_store);
        let new_max = new_potential.map(|nm| std::cmp::max(max, *nm)).unwrap_or(max);
        let new_max = match new_potential {
          Some(&new_max) => std::cmp::max(max, new_max),
          None => max,
        };
        (new_store, new_max)
      },
    ).1
}
#[test]
fn example_biggest_ever() {
  let input = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";
  let instructions = get_instructions(input);
  let be = biggest_register_ever(&instructions);
  assert_eq!(be, 10);
}

pub fn get_instructions(s: &str) -> Vec<Instruction> {
  s.lines()
    .map(|l| {
      let res = parse::parse_line(CompleteStr(l));
      res.unwrap().1
    }).collect()
}

pub fn eval_instructions<'a, I>(is: I) -> HashMap<String, isize>
where
  I: IntoIterator<Item = &'a Instruction>,
{
  is.into_iter()
    .fold(HashMap::new(), |acc, cur| eval_instruction(cur, acc))
}
#[test]
fn example_instructions() {
  let input = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";
  let instructions = get_instructions(input);
  let registers = eval_instructions(&instructions);
  assert_eq!(*biggest_register(&registers).unwrap(), 1);
}

fn eval_instruction(i: &Instruction, store: HashMap<String, isize>) -> HashMap<String, isize> {
  if eval_condition(&store, &i.cond) {
    set_value_in_register(&i.register, store, &i.op, i.amount)
  } else {
    store
  }
}

fn eval_condition(store: &HashMap<String, isize>, c: &Condition) -> bool {
  let val = get_value_in_register(&c.register, store);
  let operation = function_for_question(&c.q);
  operation(val, c.amount)
}

fn get_value_in_register(r: &str, store: &HashMap<String, isize>) -> isize {
  *store.get(r).unwrap_or(&0)
}

fn function_for_operation(op: &Operation) -> Box<Fn(isize, isize) -> isize> {
  match op {
    Operation::Increase => Box::new(|a, b| a + b),
    Operation::Decrease => Box::new(|a, b| a - b),
  }
}

fn function_for_question(q: &Question) -> Box<Fn(isize, isize) -> bool> {
  match q {
    Question::Gt => Box::new(|a, b| a > b),
    Question::Ge => Box::new(|a, b| a >= b),
    Question::Lt => Box::new(|a, b| a < b),
    Question::Le => Box::new(|a, b| a <= b),
    Question::Eq => Box::new(|a, b| a == b),
    Question::Ne => Box::new(|a, b| a != b),
  }
}
// condition is true for this
fn set_value_in_register(
  r: &str,
  mut store: HashMap<String, isize>,
  op: &Operation,
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
    set_value_in_register("a", store, &Operation::Increase, 10),
    vec![("a".to_string(), 10)].iter().cloned().collect()
  );
}

#[test]
fn val_in() {
  let store = vec![("a".to_string(), 5)].iter().cloned().collect();
  assert_eq!(
    set_value_in_register("a", store, &Operation::Decrease, -10),
    vec![("a".to_string(), 15)].iter().cloned().collect()
  );
}
