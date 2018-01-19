/// functions for day 1

pub fn captcha(i: &[usize]) -> usize {
  let mut circle : Vec<usize> = i.iter().cloned().collect::<Vec<_>>();
  circle.push(i[0]);
  circle.as_slice().windows(2).
    filter(|&c| c[0] == c[1]).
    map(|c| c[0]).
    sum()
}

/// split into two,
/// filter if the same,
/// then sum up
pub fn part_two(i: &[usize]) -> usize {
  let offset = i.len() / 2;
  let (first, second) = i.split_at(offset);
  // zip first and second together for filter
  let result: usize = first.iter().zip(second.iter()).
    filter(|c| c.0 == c.1).
    map(|c| c.1).
    sum();
  result * 2
}

pub fn string_to_vec(i: &str) -> Vec<usize> {
  let ret = i.chars().map(|c| c.to_digit(10).unwrap() as usize).
  collect::<Vec<_>>();
  ret
}

#[cfg(test)]
mod test {
  use super::captcha;
  #[test]
  fn given_examples() {
    assert_eq!(captcha(&vec![1, 1, 2, 2]), 3);
    assert_eq!(captcha(&vec![1, 1, 1, 1]), 4);
    assert_eq!(captcha(&vec![1, 2, 3, 4]), 0);
    assert_eq!(captcha(&vec![9, 1, 2, 1, 2, 1, 2, 9]), 9);
  }

#[test]
  fn part_two() {
    use super::part_two;
    assert_eq!(part_two(&vec![1, 2, 1, 2]), 6);
    assert_eq!(part_two(&vec![1, 2, 2, 1]), 0);
    assert_eq!(part_two(&vec![1, 2, 3, 4, 2, 5]), 4);
    assert_eq!(part_two(&vec![1, 2, 3, 1, 2, 3]), 12);
    assert_eq!(part_two(&vec![1, 2, 1, 3, 1, 4, 1, 5]), 4);
  }

  #[test]
  fn do_string() {
    use super::string_to_vec;
    assert_eq!(string_to_vec("122342"),
      vec![1, 2, 2, 3, 4, 2]);
  }
}