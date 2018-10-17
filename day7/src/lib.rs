// include nom
#[macro_use]
extern crate nom;
use nom::types::CompleteStr;
use nom::{alpha, digit, space};
use std::collections::HashSet;

#[derive(PartialEq, Debug)]
struct Program {
  name: String,
  weight: usize,
  children: Option<Vec<String>>,
}

// return an either?
pub fn balanced_weight(i: &str) -> usize {
  // start at the bottom, check weights of children
  // if all children's children have balanced weight, check children themselves
  0
}
#[test]
fn test_balanced_weight() {
  let input = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";
  assert_eq!(balanced_weight(input), 60_usize);

  let simpleinput = "r (77) -> a b c
a (6)
b (6)
c (7)";
  assert_eq!(balanced_weight(simpleinput), 6);

  let diffinput = "r (77) -> a b
a (5) -> m n o
b (7)
m (1)
n (1)
o (1)";
  assert_eq!(balanced_weight(diffinput), 4);
}

pub fn bottom_program(i: &str) -> String {
  let programs = parse_lines(i);
  let (parents, _kids) = programs.iter().fold(
    (HashSet::<String>::new(), HashSet::<String>::new()),
    |(parents, children), p: &Program| {
      if let Some(ref c) = p.children {
        // iterate over children and remove from parents
        let mut new_parents = c.iter().fold(parents, |mut acc, kid| {
          acc.remove(kid);
          acc
        });
        // check to see if add p's name to parents
        if !children.contains(&p.name) {
          new_parents.insert(p.name.clone());
        }
        // and add children to children
        let new_children = c.iter().fold(children, |mut acc, kid| {
          acc.insert(kid.clone());
          acc
        });
        (new_parents, new_children)
      } else {
        (parents, children)
      }
    },
  );

  if parents.len() == 1 {
    parents.iter().nth(0).unwrap().to_string()
  } else {
    panic!("more than one bottom program!")
  }
}
#[test]
fn example_bottom_program() {
  let input = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";
  assert_eq!(bottom_program(input), "tknk");
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
