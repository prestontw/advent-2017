use std::cmp::max;

enum Direction {
  North,
  Northeast,
  Southeast,
  South,
  Southwest,
  Northwest,
}

pub fn fewest_number_steps(i: &str) -> usize {
  let positions: Vec<Direction> = i.split(",").map(string_to_direction).collect();
  hex_distance(&position(&positions))
}
#[test]
fn test_example_steps() {
    assert_eq!(fewest_number_steps("ne,ne,ne"), 3);
    assert_eq!(fewest_number_steps("ne,ne,sw,sw"), 0);
    assert_eq!(fewest_number_steps("ne,ne,s,s"), 2);
    assert_eq!(fewest_number_steps("se,sw,se,sw,sw"), 3);
}

pub fn max_distance_for_directions(i: &str) -> usize {
  let positions: Vec<Direction> = i.split(",").map(string_to_direction).collect();
  max_distance(&positions).0
}

#[derive(Debug, PartialEq)]
struct Position {
  SN: isize,
  WE: isize,
}

fn string_to_direction(i: &str) -> Direction {
  use self::Direction::*;
  match i {
    "ne" => Northeast,
    "n" => North,
    "nw" => Northwest,
    "s" => South,
    "se" => Southeast,
    "sw" => Southwest,
    _ => panic!("Weird input: {}", i),
  }
}

// really, this should take in a position rather than a stream
fn hex_distance(p: &Position) -> usize {
  let z_diff = p.SN + p.WE;
  max(p.SN.abs(), max(p.WE.abs(), z_diff.abs())) as usize
}
#[test]
fn test_hex_distance() {
  use self::Direction::*;
  assert_eq!(
    hex_distance(&position(&vec![Northeast, Northeast, Northeast])),
    3
  );
  assert_eq!(
    hex_distance(&position(&vec![Southeast, Northeast, Southeast])),
    3
  );
  assert_eq!(hex_distance(&position(&vec![South, North,])), 0);
  assert_eq!(
    hex_distance(&position(&vec![Northeast, Northeast, Southwest, Southwest])),
    0
  );
  assert_eq!(
    hex_distance(&position(&vec![Northeast, Northeast, South, South])),
    2
  );
  assert_eq!(
    hex_distance(&position(&vec![
      Southeast, Southwest, Southeast, Southwest, Southwest
    ])),
    3
  );
}

fn max_distance<'a, I>(ds: I) -> (usize, Position) where I: IntoIterator<Item = &'a Direction> {
  ds.into_iter().fold((0, Position {SN: 0, WE: 0}), |(cur_max, cur_pos), cur_direction| {
    let new_pos = update_position(cur_pos, &cur_direction);
    let cur_distance = hex_distance(&new_pos);

    (max(cur_distance, cur_max), new_pos)
  })
}
fn position<'a, I>(ds: I) -> Position
where
  I: IntoIterator<Item = &'a Direction>,
{
  ds.into_iter().fold(Position { SN: 0, WE: 0 }, |acc, cur| {
    update_position(acc, cur)
  })
}
#[test]
fn test_positions() {
  use self::Direction::*;
  assert_eq!(
    position(&vec![Southeast, Southwest, Southeast, Southwest, Southwest]),
    Position { SN: -2, WE: -1 }
  );
}

fn update_position(mut p: Position, d: &Direction) -> Position {
  use self::Direction::*;
  match d {
    North => {
      p.SN += 1;
      p
    }
    South => {
      p.SN -= 1;
      p
    }
    Northeast => {
      p.WE += 1;
      p
    }
    Northwest => {
      p.WE -= 1;
      p.SN += 1;
      p
    }
    Southwest => {
      p.WE -= 1;
      p
    }
    Southeast => {
      p.WE += 1;
      p.SN -= 1;
      p
    }
  }
}
