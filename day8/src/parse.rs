use super::{Instruction, Question, Condition, Operation::{self, *}};
use nom::types::CompleteStr;
use nom::{alpha, digit, space, be_i64, le_i64};
use std::str::FromStr;

/* examples
b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10
*/

named!(parse_by<CompleteStr, isize>,
  map_res!(recognize!(pair!(opt!(tag!("-")), digit)),
    |cs: CompleteStr| isize::from_str(&cs)));
#[test]
fn test_numbers() {
  assert_eq!(parse_by(CompleteStr("-10")), Ok((CompleteStr(""), -10)));
  assert_eq!(parse_by(CompleteStr("10")), Ok((CompleteStr(""), 10)));
}

fn op_text_to_enum(s: &str) -> Operation {
  match s {
    "inc" => Operation::Increase,
    "dec" => Operation::Decrease,
    _ => panic!("unknown operation text: {}; options are `inc` or `dec", s),
  }
}

fn question_text_to_enum(s: &str) -> Question {
  match s {
    ">" => Question::gt,
    "<" => Question::lt,
    ">=" => Question::ge,
    "<=" => Question::le,
    "==" => Question::eq,
    "!=" => Question::ne,
    _ => panic!("unknown question in condition: {}", s),
  }
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