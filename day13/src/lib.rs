#![allow(non_snake_case)]
#[macro_use]
extern crate nom;
use nom::digit;
use nom::types::CompleteStr;
use std::collections::HashMap;

named!(parseLine<CompleteStr, (isize, isize)>,
  do_parse!(
    start: digit >>
    tag!(": ") >>
    stop: digit >>
    ((start.parse::<isize>().unwrap(), stop.parse::<isize>().unwrap())))
);

fn updateMap<F>(
  main: &mut HashMap<isize, isize>,
  r: &HashMap<isize, isize>,
  r2: &HashMap<isize, isize>,
  f: F,
) where
  F: Fn(isize, isize, isize) -> isize,
{
  for k in r.keys() {
    let entry = main.entry(*k).or_insert(0);
    *entry = f(*entry, r[k], r2[k]);
  }
}
#[test]
fn testAddRange() {
  let mut hs: HashMap<isize, isize> = vec![(0, 1), (2, 3)].into_iter().collect();
  let r: HashMap<isize, isize> = vec![(0, 1), (2, -1)].into_iter().collect();
  let f = |m: isize, r: isize, _: isize| m + r;
  updateMap(&mut hs, &r, &r, f);
  assert_eq!(hs, vec![(0, 2), (2, 2)].into_iter().collect());
}
#[test]
fn testChangeRange() {
  let mut dirs: HashMap<isize, isize> = vec![(0, 1), (2, 1)].into_iter().collect();
  let positions: HashMap<isize, isize> = vec![(0, 1), (2, 4)].into_iter().collect();
  let ranges: HashMap<isize, isize> = vec![(0, 3), (2, 4)].into_iter().collect();
  let f = |d: isize, p: isize, r: isize| {
    if p >= r {
      -1
    } else if p == 1 {
      1
    } else {
      d
    }
  };
  updateMap(&mut dirs, &positions, &ranges, f);
  assert_eq!(dirs, vec![(0, 1), (2, -1)].into_iter().collect());
}

fn collisions(ranges: &HashMap<isize, isize>) -> Vec<isize> {
  let mut positions: HashMap<isize, isize> = ranges.keys().cloned().map(|k| (k, 1)).collect();
  let mut directions: HashMap<isize, isize> = positions.clone();
  let mut collisions: Vec<isize> = vec![];
  let max = ranges.keys().cloned().max().unwrap();
  // split into own function
  for position in 0..(max + 1) {
    if let Some(_r) = ranges.get(&position) {
      let p = positions[&position];
      if p == 1 {
        collisions.push(position);
      }
    }
    // update positions
    updateMap(&mut positions, &directions, &ranges, |p, d, _r| p + d);
    // update directions
    updateMap(&mut directions, &positions, &ranges, |d, p, r| {
      if p >= r {
        -1
      } else if p == 1 {
        1
      } else {
        d
      }
    });
  }
  collisions
}
#[test]
fn testCollisions() {
  let ranges: HashMap<isize, isize> = vec![(0, 3), (1, 2), (4, 4), (6, 4)].into_iter().collect();
  assert_eq!(collisions(&ranges), vec![0, 6]);
}

pub fn part1(i: &str) -> isize {
  let ranges: HashMap<isize, isize> = i
    .lines()
    .map(|l| parseLine(CompleteStr(l)).unwrap().1)
    .collect();

  let cols = collisions(&ranges);
  cols
    .iter()
    .fold(0, |acc, cur| acc + ranges[cur] * cur)
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
