pub fn input_to_list(i: &str) -> Vec<usize> {
  i.split(",").map(|w| w.parse::<usize>().unwrap()).collect()
}
#[test]
fn test_input_to_list() {
  assert_eq!(input_to_list("3,4"), vec![3, 4]);
}

/// Call this with n=256 to get [0, 255]
pub fn initial_list(n: usize) -> Vec<usize> {
  (0..n).collect()
}

fn start_plus_length_indices(start: usize, length: usize) -> Vec<usize> {
  (start..length).collect()
}
fn wrap_indices(is: &[usize], list_length: usize) -> Vec<usize> {
  is.iter().map(|&c| c % list_length).collect()
}
#[test]
fn test_wrapping() {
  assert_eq!(wrap_indices(&vec![3, 4, 5, 6], 5), vec![3, 4, 0, 1]);
}

fn reverse_values_at_indices(values: &[usize], indices: &[usize]) -> Vec<usize> {
  // could just do this as last half and first half, then reverse
  // instead of reversing, zipping, then halfing
  let mut values: Vec<usize> = values.iter().cloned().collect();
  // don't swap twice
  let mut reversed: Vec<usize> = indices.iter().cloned().collect();
  reversed.reverse();
  let to_swap: Vec<(&usize, usize)> = indices.iter().zip(reversed).collect();
  let (to_swap, _redundant) = to_swap.split_at(to_swap.len() / 2);
  for &(&i_1, i_2) in to_swap {
    values.swap(i_1, i_2);
  }
  values
}
#[test]
fn testReverseValues() {
  assert_eq!(reverse_values_at_indices(&mut vec![10, 11, 12, 13, 14], &vec![0, 1, 2]), vec![12, 11, 10, 13, 14]);
  assert_eq!(reverse_values_at_indices(&mut vec![10, 11, 12, 13, 14], &vec![3, 4, 0, 1]), vec![14, 13, 12, 11, 10]);
}