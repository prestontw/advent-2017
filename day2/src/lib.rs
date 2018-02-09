/// functions for day 1

fn row_diff<I>(r: I) -> isize where I: Clone + IntoIterator<Item = isize> {
  // need to do this since I don't know an easy way to 
  // into_iter while not moving
  let temp = r.clone();
  r.into_iter().max().unwrap() - temp.into_iter().min().unwrap()
}

fn row_quotient<I>(r: I) -> isize where I: Clone + IntoIterator<Item = isize> {
  let temp = r.clone();
  for el1 in r {
    for el2 in temp.clone().into_iter().filter(|&i| i != el1) {
      if el1 % el2 == 0 {
        return el1 / el2
      }
    }
  }
  0
}
#[test]
fn test_row_quotient() {
  assert_eq!(row_quotient(vec![5, 9, 2, 8]), 4);
  assert_eq!(row_quotient(vec![9, 4, 7, 3]), 3);
  assert_eq!(row_quotient(vec![3, 8, 6, 5]), 2);
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

pub fn checkdiv(d: Vec<Vec<isize>>) -> isize {
  d.into_iter().map(|v| row_quotient(v)).sum()
}
#[test]
fn test_checkdiv() {
  assert_eq!(checkdiv(vec![vec![5, 9, 2, 8],
                           vec![9, 4, 7, 3],
                           vec![3, 8, 6, 5]]), 9);
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