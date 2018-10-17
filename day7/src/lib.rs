// include nom
#[macro_use]
extern crate nom;
use nom::types::CompleteStr;
use nom::{alpha, digit, space};

#[derive(PartialEq, Debug)]
struct Program {
  name: String,
  weight: usize,
  children: Option<Vec<String>>,
}

named!(weight<CompleteStr, usize>, delimited!(char!('('), number, char!(')')));

#[test]
fn test_weight() {
  assert_eq!(weight(CompleteStr("(77)")), Ok((CompleteStr(""), 77_usize)));
}

use std::str::FromStr;

named!(
  number<CompleteStr, usize>,
  map_res!(recognize!(digit),
  |cs: CompleteStr| usize::from_str(&cs))
);

named!(arrow<CompleteStr, CompleteStr>, tag!("->"));

// use opt! for parsing children
// opt!(arrow, space, many0(children), chars)

named!(
  children<CompleteStr, Vec<String>>,
  do_parse!(
    ws: separated_list!(tag!(", "), alpha)
      >> (ws
        .iter()
        .map(|c| String::from_str(c).unwrap())
        .collect())
  )
);

#[test]
fn test_children() {
  assert_eq!(
    children(CompleteStr("abcd, defg, als")),
    Ok((
      CompleteStr(""),
      vec!["abcd", "defg", "als"]
        .iter()
        .map(|c| c.to_string())
        .collect()
    ))
  );
}

named!(
  line<CompleteStr, Program>,
  do_parse!(
    n: alpha
      >> space
      >> w: weight
      >> c: opt!(do_parse!(space >> arrow >> space >> c: children >> (c)))
      >> (Program {
        name: String::from_str(&n).unwrap(),
        weight: w,
        children: c,
      })
  )
);

#[test]
fn test_lines() {
  assert_eq!(
    line(CompleteStr("pbga (66)")),
    Ok((
      CompleteStr(""),
      Program {
        name: "pbga".to_string(),
        weight: 66,
        children: None
      }
    ))
  );

  assert_eq!(
    line(CompleteStr("fwft (72) -> ktlj, cntj, xhth")),
    Ok((
      CompleteStr(""),
      Program {
        name: "fwft".to_string(),
        weight: 72,
        children: Some(
          vec!["ktlj", "cntj", "xhth"]
            .iter()
            .map(|c| c.to_string())
            .collect()
        )
      }
    ))
  );
}

fn parse_lines(i: &str) -> Vec<Program> {
  i.lines()
    .map(|l| {
      let res = line(CompleteStr(l));
      res.unwrap().1
    }).collect()
}
#[test]
fn test_parse_lines() {
  assert_eq!(
    parse_lines(
      "ktlj (57)
fwft (72) -> ktlj, cntj, xhth"
    ),
    vec![
      Program {
        name: "ktlj".to_string(),
        weight: 57,
        children: None,
      },
      Program {
        name: "fwft".to_string(),
        weight: 72,
        children: Some(
          vec!["ktlj", "cntj", "xhth"]
            .iter()
            .map(|c| c.to_string())
            .collect()
        )
      }
    ]
  )
}
