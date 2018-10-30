#![feature(nll)]

pub fn input_to_list(i: &str) -> Vec<u8> {
  i.split(",").map(|w| w.parse::<u8>().unwrap()).collect()
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

pub fn get_hash_of_list(i: &[u8]) -> usize {
  i[0] as usize * i[1] as usize
}

pub fn hash_list(n: usize, lengths: &[u8]) -> Vec<u8> {
  let mut start = initial_list(n);

  // move this out so can repeat hashlist thingy multiple times? or just loop lengths over and over?
  fn inner<'a>(
    lst: &'a mut [u8],
    cur_pos: u8,
    step_size: usize,
    length: u8,
  ) -> (&'a mut [u8], u8, usize) {
    let len = lst.len();
    let indices = wrap_indices(
      &start_plus_length_indices(cur_pos as usize, length as usize)[..],
      len,
    );
    // actually perform switching here, including getting indices
    (
      reverse_values_at_indices(lst, &indices),
      ((cur_pos as usize + step_size + length as usize) % len) as u8,
      step_size + 1,
    )
  }

  lengths.iter().fold((&mut start[..], 0, 0), |acc, cur| {
    inner(acc.0, acc.1, acc.2, *cur)
  });

  start.to_vec()
}
#[test]
fn test_example() {
  assert_eq!(get_hash_of_list(&hash_list(5, &[3, 4, 1, 5])[..]), 12);
}
/// Call this with n=256 to get [0, 255]
pub fn initial_list(n: usize) -> Vec<u8> {
  (0..n).map(|c| c as u8).collect()
}

fn start_plus_length_indices(start: usize, length: usize) -> Vec<usize> {
  (start..(start + length)).collect()
}

pub fn string_to_hash_string(i: &str) -> String {
  let mut lengths = string_to_ascii_codes(i);
  append_usual(&mut lengths);
  // repeat lengths 64 times
  let repeated: Vec<u8> = lengths
    .iter()
    .cloned()
    .cycle()
    .take(64 * lengths.len())
    .collect();
  let hash = hash_list(256, &repeated[..]);
  let dense_hash = sparse_hash_to_dense_hash(&hash[..]);
  let hash_strings = dense_hash.iter().map(number_to_hex);
  hash_strings.fold("".to_string(), |acc, cur| acc + &cur)
}
#[test]
fn test_part_two() {
  assert_eq!(
    string_to_hash_string("AoC 2017"),
    "33efeb34ea91902bb2f59c9920caa6cd"
  );
  assert_eq!(
    string_to_hash_string("1,2,3"),
    "3efbe78a8d82f29979031a4aa0b16a9d"
  );
  assert_eq!(
    string_to_hash_string("1,2,4"),
    "63960835bcdc130f0b66d7ff4f6a5a8e"
  );
  assert_eq!(
    string_to_hash_string(""),
    "a2582a3a0e66e6e86e3812dcb672a272"
  );
}

fn sparse_hash_to_dense_hash(i: &[u8]) -> Vec<u8> {
  i.chunks(16).map(xor_sparse_hash).collect()
}

fn wrap_indices(is: &[usize], list_length: usize) -> Vec<u8> {
  is.iter().map(|&c| (c % list_length) as u8).collect()
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
  assert_eq!(
    xor_sparse_hash(&vec![65, 27, 9, 1, 4, 3, 40, 50, 91, 7, 6, 0, 2, 5, 68, 22]),
    64
  );
}

fn number_to_hex(i: &u8) -> String {
  let result = format!("{:x}", i).to_string();
  if result.len() == 1 {
    // then prepend a 0
    "0".to_owned() + &result
  } else {
    result
  }
}
#[test]
fn test_numbers_to_hex() {
  assert_eq!(number_to_hex(&64), "40".to_string());
  assert_eq!(number_to_hex(&7), "07".to_string());
  assert_eq!(number_to_hex(&255), "ff".to_string());
}

fn reverse_values_at_indices<'a>(values: &'a mut [u8], indices: &[u8]) -> &'a mut [u8] {
  // could produce different first halves and second halves based on parity of length of list
  let (first_half, _second) = indices.split_at(indices.len() / 2);
  let (_first, second_half) = indices.split_at((indices.len() + 1) / 2);
  let mut second_half = second_half.to_vec();
  // don't swap twice
  second_half.reverse();
  let to_swap = first_half.iter().zip(second_half);
  for (&i_1, i_2) in to_swap {
    values.swap(i_1 as usize, i_2 as usize);
  }
  values
}
#[test]
fn test_reverse_values() {
  assert_eq!(
    reverse_values_at_indices(&mut vec![10, 11, 12, 13, 14], &vec![0, 1, 2]),
    &[12, 11, 10, 13, 14]
  );
  assert_eq!(
    reverse_values_at_indices(&mut vec![10, 11, 12, 13, 14], &vec![3, 4, 0, 1]),
    &[14, 13, 12, 11, 10]
  );
}
