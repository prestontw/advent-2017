#![feature(nll)]

pub fn input_to_list(i: &str) -> Vec<usize> {
  i.split(",").map(|w| w.parse::<usize>().unwrap()).collect()
}
#[test]
fn test_input_to_list() {
  assert_eq!(input_to_list("3,4"), vec![3, 4]);
}

fn string_to_ascii_codes(i: &str) -> Vec<u8> {
  i.as_bytes().to_vec()
}
#[test]
fn test_part_two_input() {
  assert_eq!(string_to_ascii_codes("1,2,3"), vec![49, 44, 50, 44, 51]);
}

fn append_usual(v: &mut Vec<u8>) {
  let mut other = vec![17, 31, 73, 47, 23];
  v.append(&mut other);
}
#[test]
fn test_append() {
  let mut base = vec![1, 2];
  append_usual(&mut base);
  assert_eq!(base, vec![1, 2, 17, 31, 73, 47, 23]);
}

pub fn get_hash_of_list(i: &[usize]) -> usize {
  i[0] * i[1]
}

pub fn hash_list(n: usize, lengths: &[usize]) -> Vec<usize> {
  let mut start = initial_list(n);

// move this out so can repeat hashlist thingy multiple times? or just loop lengths over and over?
  fn inner<'a>(lst: &'a mut[usize], cur_pos: usize, step_size: usize, length: usize) -> (&'a mut [usize], usize, usize) {
    let len = lst.len();
    let indices = wrap_indices(&start_plus_length_indices(cur_pos, length), len);
    // actually perform switching here, including getting indices
    (reverse_values_at_indices(lst, &indices), (cur_pos + step_size + length) % len, step_size + 1)
  }

  lengths.iter().fold((&mut start[..], 0, 0), |acc, cur| {
    inner(acc.0, acc.1, acc.2, *cur)
  });

  start.to_vec()
}
#[test]
fn test_example() {
  assert_eq!(get_hash_of_list(&hash_list(5, &[3, 4, 1, 5])), 12);
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

/// block of 16 numbers to one number
fn xor_sparse_hash(i: &[u8]) -> u8 {
  i.iter().fold(0, |acc, cur| acc ^ cur)
}
#[test]
fn test_example_xor() {
  assert_eq!(xor_sparse_hash(&vec![65, 27, 9, 1, 4, 3, 40, 50, 91, 7, 6, 0, 2, 5, 68, 22]), 64);
}

fn number_to_hex(i: u8) -> String {
  let result = format!("{:x}", i).to_string();
  if result.len() == 1 {
    // then prepend a 0
    "0".to_owned() + &result
  }
  else {
    result
  }
}
#[test]
fn test_numbers_to_hex() {
    assert_eq!(number_to_hex(64), "40".to_string());
    assert_eq!(number_to_hex(7), "07".to_string());
    assert_eq!(number_to_hex(255), "ff".to_string());
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
fn test_reverse_values() {
  assert_eq!(reverse_values_at_indices(&mut vec![10, 11, 12, 13, 14], &vec![0, 1, 2]), &[12, 11, 10, 13, 14]);
  assert_eq!(reverse_values_at_indices(&mut vec![10, 11, 12, 13, 14], &vec![3, 4, 0, 1]), &[14, 13, 12, 11, 10]);
}