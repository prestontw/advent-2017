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