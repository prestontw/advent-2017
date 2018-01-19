/// junk

pub fn captcha(_: &[u8]) -> usize {
  0
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
}