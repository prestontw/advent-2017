use std::cmp::min;

enum Directions {
  North,
  Northeast,
  Southeast,
  South,
  Southwest,
  Northwest,
}

#[derive(Debug, PartialEq)]
struct Position {
  SN: isize,
  EW: isize,
}

// really, this should take in a position rather than a stream
fn hex_distance(p: Position) -> usize {
  let abs_sn = p.SN.abs();
  let abs_ew = p.EW.abs();
  let diagonal = min(abs_ew, abs_sn);
  (diagonal + (abs_ew - diagonal) + (abs_sn - diagonal)) as usize
}
#[test]
fn test_hex_distance() {
  use self::Directions::*;
  assert_eq!(
    hex_distance(position(&vec![Northeast, Northeast, Northeast])),
    3
  );
  assert_eq!(
    hex_distance(position(&vec![Southeast, Northeast, Southeast])),
    3
  );
  assert_eq!(hex_distance(position(&vec![South, North,])), 0);
  assert_eq!(
    hex_distance(position(&vec![Northeast, Northeast, Southwest, Southwest])),
    0
  );
  assert_eq!(
    hex_distance(position(&vec![Northeast, Northeast, South, South])),
    2
  );
  assert_eq!(
    hex_distance(position(&vec![
      Southeast, Southwest, Southeast, Southwest, Southwest
    ])),
    3
  );
}

fn position<'a, I>(ds: I) -> Position
where
  I: IntoIterator<Item = &'a Directions>,
{
  ds.into_iter().fold(Position { SN: 0, EW: 0 }, |acc, cur| {
    update_position(acc, cur)
  })
}
#[test]
fn test_positions() {
  use self::Directions::*;
  assert_eq!(position(&vec![Southeast, Southwest, Southeast, Southwest, Southwest]), Position { SN: 3, EW: 1});
}

fn update_position(mut p: Position, d: &Directions) -> Position {
  use self::Directions::*;
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
      p.EW -= 1;
      p.SN += 1;
      p
    }
    Northwest => {
      p.EW += 1;
      p.SN += 1;
      p
    }
    Southwest => {
      p.EW += 1;
      p.SN -= 1;
      p
    }
    Southeast => {
      p.EW -= 1;
      p.SN -= 1;
      p
    }
  }
}