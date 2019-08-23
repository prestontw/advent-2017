
// use string to hash
use crate::day10;

pub fn gridify_hash(i: &str) -> Vec<String> {
  let i = i.to_string();
  let rows = (0..128).into_iter().map(|n| i.clone() + "-" + &n.to_string());
  let knots = rows.map(|r| day10::string_to_hash_string(&r));
  // format each character as binary
  // then map each 1 to #, 0 to .
  knots.map(|s| s.to_string()).collect()
}

#[test]
fn test_gridify_hash() {
  let result = gridify_hash("flqrgnkx"); 
  assert_eq!(&result[0][0..8], "##.#.#..");
  assert_eq!(&result[1][0..8], ".#.#.#.#");
  assert_eq!(&result[2][0..8], "....#.#.");
  assert_eq!(&result[3][0..8], "#.#.##.#");
  assert_eq!(&result[4][0..8], ".##.#...");
  assert_eq!(&result[5][0..8], "##..#..#");
  assert_eq!(&result[6][0..8], ".#...#..");
  assert_eq!(&result[7][0..8], "##.#.##.");
}