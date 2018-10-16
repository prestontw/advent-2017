// include nom
#[macro_use]
extern crate nom;
use nom::{alpha, digit, line_ending, space};

#[derive(PartialEq, Debug)]
struct Program {
  name: String,
  weight: usize,
  children: Option<Vec<String>>,
}

named!(weight<usize>, delimited!(char!('('), number, char!(')')));

#[test]
fn test_weight() {
  assert_eq!(weight(&b"(77)"[..]), Ok((&b""[..], 77_usize)));
}

named!(
  number<usize>,
  map_res!(map_res!(digit, std::str::from_utf8), |s: &str| s
    .parse::<usize>())
);

named!(arrow, tag!("->"));

// use opt! for parsing children
// opt!(arrow, space, many0(children), chars)

named!(
  children<Vec<String>>,
  complete!(
  do_parse!(
    ws: separated_list!(tag!(", "), alpha)
      >> (ws
        .iter()
        .map(|c| String::from_utf8(c.to_vec()).unwrap())
        .collect())
  ))
);

#[test]
fn test_children() {
  assert_eq!(
    children(&b"abcd, defg, als"[..]),
    Ok((
      &b""[..],
      vec!["abcd", "defg", "als"]
        .iter()
        .map(|c| c.to_string())
        .collect()
    ))
  );
}

named!(
  line<Program>,
  do_parse!(
    n: alpha
      >> space
      >> w: weight
      >> c: opt!(do_parse!(space >> arrow >> space >> c: children >> (c)))
      >> line_ending
      >> (Program {
        name: String::from_utf8(n.to_vec()).unwrap(),
        weight: w,
        children: c,
      })
  )
);

#[test]
fn test_lines() {
  assert_eq!(
    line(b"pbga (66)\n"),
    Ok((
      &b""[..],
      Program {
        name: "pbga".to_string(),
        weight: 66,
        children: None
      }
    ))
  );

  assert_eq!(
    line(b"fwft (72) -> ktlj, cntj, xhth\n"),
    Ok((
      &b""[..],
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
  i.lines().map(|l| line(&l.as_bytes()[..]).unwrap().1).collect()
}
#[test]
fn test_parse_lines() {
  assert_eq!(parse_lines("ktlj (57)
fwft (72) -> ktlj, cntj, xhth"),
vec![Program {
  name: "ktlj".to_string(),
  weight: 57,
  children: None,
}, Program {
  name: "fwft".to_string(),
  weight: 72,
  children: Some(vec!["ktlj", "cntj", "xhth"].iter().map(|c| c.to_string()).collect())
}])
}