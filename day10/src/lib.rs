#![feature(nll)]

pub fn input_to_list(i: &str) -> Vec<usize> {
  i.split(",").map(|w| w.parse::<usize>().unwrap()).collect()
}
#[test]
fn test_input_to_list() {
  assert_eq!(input_to_list("3,4"), vec![3, 4]);
}

pub fn hash_list(n: usize, lengths: &[usize]) -> usize {
  let mut start = initial_list(n);

  fn inner<'a>(lst: &'a mut[usize], cur_pos: usize, step_size: usize, length: usize) -> (&'a mut [usize], usize, usize) {
    let len = lst.len();
    let indices = wrap_indices(&start_plus_length_indices(cur_pos, length), len);
    // actually perform switching here, including getting indices
    (reverse_values_at_indices(lst, &indices), (cur_pos + step_size + length) % len, step_size + 1)
  }

  lengths.iter().fold((&mut start[..], 0, 0), |acc, cur| {
    inner(acc.0, acc.1, acc.2, *cur)
  });

  start[0] * start[1]
}
#[test]
fn test_example() {
  assert_eq!(hash_list(5, &[3, 4, 1, 5]), 12);
}
/// Call this with n=256 to get [0, 255]
pub fn initial_list(n: usize) -> Vec<usize> {
  (0..n).collect()
}

fn start_plus_length_indices(start: usize, length: usize) -> Vec<usize> {
  (start..(start + length)).collect()
}
fn wrap_indices(is: &[usize], list_length: usize) -> Vec<usize> {
  is.iter().map(|&c| c % list_length).collect()
}
#[test]
fn test_wrapping() {
  assert_eq!(wrap_indices(&vec![3, 4, 5, 6], 5), vec![3, 4, 0, 1]);
}

fn reverse_values_at_indices<'a>(values: &'a mut [usize], indices: &[usize]) -> &'a mut [usize] {
  // could produce different first halves and second halves based on parity of length of list
  let (first_half, _second) = indices.split_at(indices.len() / 2);
  let (_first, second_half) = indices.split_at((indices.len() + 1) / 2);
  let mut second_half = second_half.to_vec();
  // don't swap twice
  second_half.reverse();
  let to_swap = first_half.iter().zip(second_half);
  for (&i_1, i_2) in to_swap {
    values.swap(i_1, i_2);
  }
  values
}
#[test]
fn testReverseValues() {
  assert_eq!(reverse_values_at_indices(&mut vec![10, 11, 12, 13, 14], &vec![0, 1, 2]), &[12, 11, 10, 13, 14]);
  assert_eq!(reverse_values_at_indices(&mut vec![10, 11, 12, 13, 14], &vec![3, 4, 0, 1]), &[14, 13, 12, 11, 10]);
}