use super::{Instruction, Question, Condition, Operation::*};
use nom::types::CompleteStr;
use nom::{alpha, digit, space, be_i64, le_i64};
use std::str::FromStr;

named!(parse_by<CompleteStr, isize>,
  map_res!(recognize!(pair!(opt!(tag!("-")), digit)),
    |cs: CompleteStr| isize::from_str(&cs)));
#[test]
fn test_numbers() {
  assert_eq!(parse_by(CompleteStr("-10")), Ok((CompleteStr(""), -10)));
  assert_eq!(parse_by(CompleteStr("10")), Ok((CompleteStr(""), 10)));
}

// todo: figure out how to include struct from lib
#[test]
fn test_line() {
  let ideal = Instruction {
    register: "a".to_string(),
    op: Increase,
    amount: 10,
    cond: Condition {
      register: "b".to_string(),
      q: Question::le,
      amount: 1
    },
  };
}