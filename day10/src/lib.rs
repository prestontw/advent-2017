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