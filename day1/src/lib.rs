/// junk

pub fn captcha(i: &[usize]) -> usize {
  let mut circle : Vec<usize> = i.iter().cloned().collect::<Vec<_>>();
  circle.push(i[0]);
  circle.as_slice().windows(2).
    filter(|&c| c[0] == c[1]).
    map(|c| c[0]).
    sum()
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
  fn do_string() {
    use super::string_to_vec;
    assert_eq!(string_to_vec("122342"),
      vec![1, 2, 2, 3, 4, 2]);
  }
}