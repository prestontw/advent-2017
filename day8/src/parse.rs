use super::{
  Condition, Instruction,
  Operation::{self, *},
  Question,
};
use nom::types::CompleteStr;
use nom::{alpha, be_i64, digit, le_i64, space};
use std::str::FromStr;

/* examples
b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10
*/

named!(parse_line<CompleteStr, Instruction>,
  do_parse!(
    r: alpha >>
    space >>
    op: operation >>
    space >>
    by: parse_amount_by >>
    tag!(" if ") >>
    c_r: alpha >>
    space >>
    q: question >>
    space >>
    base: parse_amount_by >>
    (Instruction {
      register: r.to_string(),
      op: op_text_to_enum(&op),
      amount: by,
      cond: Condition {
        register: c_r.to_string(),
        q: question_text_to_enum(&q),
        amount: base,
      }
    })
  )
);
named!(parse_amount_by<CompleteStr, isize>,
  map_res!(recognize!(pair!(opt!(tag!("-")), digit)),
    |cs: CompleteStr| isize::from_str(&cs)));
#[test]
fn test_numbers() {
  assert_eq!(
    parse_amount_by(CompleteStr("-10")),
    Ok((CompleteStr(""), -10))
  );
  assert_eq!(
    parse_amount_by(CompleteStr("10")),
    Ok((CompleteStr(""), 10))
  );
}

named!(operation<CompleteStr, CompleteStr>,
  alt!(tag!("dec") | tag!("inc"))
);

named!(question<CompleteStr, CompleteStr>,
  alt!(
    tag!("<=") |
    tag!("<=") |
    tag!("<") |
    tag!(">") |
    tag!("==") |
    tag!("!=")
    ));

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
      amount: 1,
    },
  };
  assert_eq!(
    Ok((CompleteStr(""), ideal)),
    parse_line(CompleteStr("a inc 10 if b <= 1"))
  );
}
