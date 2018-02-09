/// functions for day 1

fn row_diff<I>(r: I) -> isize where I: Clone + IntoIterator<Item = isize> {
  // need to do this since I don't know an easy way to 
  // into_iter while not moving
  let temp = r.clone();
  r.into_iter().max().unwrap() - temp.into_iter().min().unwrap()
}

pub fn checksum(d: Vec<Vec<isize>>) -> isize {
  d.into_iter().map(|v| row_diff(v)).sum()
}
#[test]
fn test_checksum() {
  assert_eq!(checksum(vec![vec![5, 1, 9, 5],
                           vec![7, 5, 3],
                           vec![2, 4, 6, 8]]), 18);  
}

pub fn to_vec(s: &str) -> Vec<Vec<isize>> {
  s.lines().map(|l| l.split_whitespace().map(|t|
  t.parse::<isize>().unwrap()).collect()).collect()
}
#[test]
fn test_to_vec() {
  assert_eq!(to_vec("5 1 9 5
  7 5 3
  2 4 6 8"), vec![vec![5, 1, 9, 5],
                  vec![7, 5, 3],
                  vec![2, 4, 6, 8]]);
}