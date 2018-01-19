/// junk

pub fn captcha(i: &[u8]) -> usize {
  let mut circle : Vec<u8> = i.iter().cloned().collect::<Vec<_>>();
  circle.push(i[0]);
  circle.as_slice().windows(2).
    filter(|&c| c[0] == c[1]).
    map(|c| c[0] as usize).
    sum()
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