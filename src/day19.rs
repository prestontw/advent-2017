
#[derive(Clone)]
pub struct Position {
  row: usize,
  col: usize,
}

pub enum Direction {
  Down,
  Up,
  Left,
  Right
}

pub struct Packet {
  position: Position,
  dir: Direction
}

pub fn starting_point(i: &str) -> Position {
  let col = i.lines().next().unwrap().find('|').unwrap();
  Position {
    row: 0,
    col
  }
}

pub fn start_down(p: &Position) -> Packet {
  Packet {
    position: p.clone(),
    dir: Direction::Down
  }
}

pub fn part1(i: &str) -> String {
  let start = start_down(&starting_point(i));

  "".into()
}

/*
 * fn main() {
    let mut v = vec![];
    iterate("hello", |c| v.push(c));
    println!("{:?}", v);
}

fn iterate<F>(i: &str, mut f: F) where F: FnMut(char) {
    for c in i.chars() {
        f(c);
    }
}
 */