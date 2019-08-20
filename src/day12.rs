use nom::types::CompleteStr;
use nom::{digit, separated_list};
use std::collections::{HashMap, HashSet};

// include nom
/**
 * examples include 0 <-> 2
 * 3 <-> 2, 4
 */
named!(parse_line<CompleteStr, (usize, Vec<usize>)>,
  do_parse!(
    start: digit >>
    tag!(" <-> ") >>
    l: separated_list!(tag!(", "), digit) >>
    (start.parse::<usize>().unwrap(), l.iter().map(|s| s.parse::<usize>().unwrap()).collect())
  )
);
#[test]
fn test_parse_line() {
  assert_eq!(
    parse_line(CompleteStr("0 <-> 2")),
    Ok((CompleteStr(""), (0, vec![2])))
  );
  assert_eq!(
    parse_line(CompleteStr("32 <-> 2, 4")),
    Ok((CompleteStr(""), (32, vec![2, 4])))
  );
}

fn lines_to_adj_map<'a, I>(lns: I) -> HashMap<usize, Vec<usize>>
where
  I: IntoIterator<Item = &'a (usize, Vec<usize>)>,
{
  lns.into_iter().cloned().collect()
}

fn encompassing_group(e: usize, adj_map: &HashMap<usize, Vec<usize>>) -> HashSet<usize> {
  // inner function to take current element, seen, and current list of things
  fn add_adj_elements(
    adj_map: &HashMap<usize, Vec<usize>>,
    mut seen: HashSet<usize>,
    mut queue: Vec<usize>,
  ) -> HashSet<usize> {
    if let Some(cur) = queue.pop() {
      if seen.contains(&cur) {
        add_adj_elements(adj_map, seen, queue)
      } else {
        // add next's elements to queue
        queue.extend_from_slice(&adj_map[&cur]);
        seen.insert(cur);
        add_adj_elements(adj_map, seen, queue)
      }
    } else {
      seen
    }
  }
  add_adj_elements(adj_map, HashSet::with_capacity(adj_map.len() / 2), vec![e])
}

pub fn part1(i: &str) -> usize {
  let adj_map = i
    .lines()
    .map(|s| parse_line(CompleteStr(s)).unwrap().1)
    .collect::<HashMap<usize, Vec<usize>>>();
  let zero_group = encompassing_group(0, &adj_map);
  zero_group.len()
}
#[test]
fn test_part1() {
  assert_eq!(
    part1(
      "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5"
    ),
    6
  );
}

// part2: total number of groups
// list of every number, remove them as we see them in a group?
pub fn num_groups(i: &str) -> usize {
  let adj_map = i
    .lines()
    .map(|s| parse_line(CompleteStr(s)).unwrap().1)
    .collect::<HashMap<usize, Vec<usize>>>();
  let mut keys: Vec<usize> = adj_map.keys().cloned().collect();
  let mut count = 0;
  while keys.len() > 0 {
    let curElement = keys.pop();
    let curGroup = encompassing_group(curElement.unwrap(), &adj_map);
    // remove all elements of curGroup from keys
    for elem in curGroup {
      keys.remove_item(&elem);
    }
    count += 1;
  }
  count
}
#[test]
fn test_part2() {
  assert_eq!(
    num_groups(
      "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5"
    ),
    2
  );
}
