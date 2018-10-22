pub enum Group {
  Garbage,
  Group { children: Vec<Group> },
}

pub fn score_group(g: &Group) -> usize {
  0
}
#[test]
fn test_scores() {
  assert_eq!(
    score_group(&Group::Group {
      children: Vec::new()
    }),
    1
  );
  assert_eq!(
    score_group(&Group::Group {
      children: Vec::new()
    }),
    6
  );
  assert_eq!(
    score_group(&Group::Group {
      children: Vec::new()
    }),
    5
  );
  assert_eq!(
    score_group(&Group::Group {
      children: Vec::new()
    }),
    16
  );
  assert_eq!(
    score_group(&Group::Group {
      children: Vec::new()
    }),
    1
  );
  assert_eq!(
    score_group(&Group::Group {
      children: Vec::new()
    }),
    9
  );
  assert_eq!(
    score_group(&Group::Group {
      children: Vec::new()
    }),
    9
  );
  assert_eq!(
    score_group(&Group::Group {
      children: Vec::new()
    }),
    3
  );
}

pub fn escape_bang_in_garbage(i: &str) -> String {
  yield_or_remove_char(i, 0, i.len(), "".to_string())
}

fn yield_or_remove_char(i: &str, index: usize, l: usize, acc: String) -> String {
  if index >= l {
    acc
  } else {
    let cur = unsafe { i.get_unchecked(index..index + 1) };
    match cur {
      "!" => yield_or_remove_char(i, index + 2, l, acc),
      _ => yield_or_remove_char(i, index + 1, l, acc + cur),
    }
  }
}

#[test]
fn test_escaping_bangs() {
  assert_eq!(escape_bang_in_garbage("<!!>"), "<>".to_string());
  assert_eq!(escape_bang_in_garbage("<!>a>"), "<a>".to_string());
  assert_eq!(escape_bang_in_garbage("<!!!>>"), "<>".to_string());
}
