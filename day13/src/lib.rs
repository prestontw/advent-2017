#![allow(non_snake_case)]
#[macro_use]
extern crate nom;
use nom::types::CompleteStr;
use self::parse::parseLine;
use std::collections::{HashMap, HashSet};

mod parse {
  use nom::digit;
  use nom::types::CompleteStr;
  named!(pub parseLine<CompleteStr, (isize, isize)>,
  do_parse!(
    start: digit >>
    tag!(": ") >>
    stop: digit >>
    ((start.parse::<isize>().unwrap(), (stop.parse::<isize>().unwrap() - 1) * 2)))
);
}

fn intersections(ranges: &HashMap<isize, isize>) -> HashSet<isize> {
  ranges
    .iter()
    .filter(|&(k, v)| k % v == 0)
    .map(|(k, _v)| *k)
    .collect()
}
#[test]
fn testIntersections() {
  let ranges: HashMap<isize, isize> = vec![(0, 4), (1, 2), (4, 6), (6, 6)].into_iter().collect();
  assert_eq!(intersections(&ranges), vec![0, 6].into_iter().collect());
}

fn wait(ranges: &HashMap<isize, isize>) -> usize {
  let mut ret = 0;
  let mut ranges: Vec<(&isize, &isize)> = ranges.iter().collect();
  ranges.sort_unstable_by_key(|(_k, v)| *v);
  loop {
    // maybe move this out of loop?-> don't, slows performance
    let mut caught = false;
    // maybe sort these ranges for depth
    for (&pos, &depth) in &ranges {
      if (pos + ret) % depth == 0 {
        caught = true;
        break;
      }
    }
    if caught == false {
      return ret as usize;
    }
    ret += 1;
  }
}
#[test]
fn testWait() {
  let ranges: HashMap<isize, isize> = vec![(0, 4), (1, 2), (4, 6), (6, 6)].into_iter().collect();
  assert_eq!(wait(&ranges), 10);
}

pub fn part1(i: &str) -> isize {
  let ranges: HashMap<isize, isize> = i
    .lines()
    .map(|l| parseLine(CompleteStr(l)).unwrap().1)
    .collect();

  let inters = intersections(&ranges);
  inters
    .iter()
    .fold(0, |acc, cur| acc + (ranges[cur] / 2 + 1) * cur)
}
#[test]
fn testPart1() {
  assert_eq!(
    part1(
      "0: 3
1: 2
4: 4
6: 4"
    ),
    24
  );
}

pub fn part2(i: &str) -> usize {
  let ranges: HashMap<isize, isize> = i
    .lines()
    .map(|l| parseLine(CompleteStr(l)).unwrap().1)
    .collect();

  wait(&ranges)
}
#[test]
fn testPart2() {
  assert_eq!(
    part2(
      "0: 3
1: 2
4: 4
6: 4"
    ),
    10
  );
}
