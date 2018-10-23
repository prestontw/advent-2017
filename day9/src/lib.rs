#[macro_use]
extern crate nom;
pub mod parse;
pub use self::parse::group;

#[derive(Debug, PartialEq, Clone)]
pub enum Group {
  Garbage(String),
  Group { children: Vec<Group> },
}

pub fn score_group(g: &Group) -> usize {
  fn score(g: &Group, acc: usize) -> usize {
    match g {
      Group::Garbage(_) => 0,
      Group::Group { children } => {
        children.iter().map(|c| score(c, acc + 1)).sum::<usize>() + acc + 1
      }
    }
  }
  score(g, 0)
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
      children: vec![Group::Group {
        children: vec![Group::Group {
          children: Vec::new()
        }]
      }]
    }),
    6
  );
  assert_eq!(
    score_group(&Group::Group {
      children: vec![
        Group::Group {
          children: Vec::new()
        },
        Group::Group {
          children: Vec::new()
        }
      ]
    }),
    5
  );
  assert_eq!(
    score_group(&Group::Group {
      children: vec![
        Group::Group {
          children: Vec::new()
        },
        Group::Garbage("".to_string()),
        Group::Group {
          children: Vec::new()
        },
        Group::Garbage("".to_string()),
      ]
    }),
    5
  );
  assert_eq!(
    score_group(&Group::Group {
      children: vec![Group::Group {
        children: vec![
          Group::Group {
            children: Vec::new()
          },
          Group::Group {
            children: Vec::new()
          },
          Group::Group {
            children: vec![Group::Group {
              children: Vec::new()
            }]
          }
        ]
      }]
    }),
    16
  );
  assert_eq!(
    score_group(&Group::Group {
      children: vec![
        Group::Garbage("".to_string()),
        Group::Garbage("".to_string()),
        Group::Garbage("".to_string()),
        Group::Garbage("".to_string())
      ]
    }),
    1
  );
  assert_eq!(
    score_group(&Group::Group {
      children: vec![
        Group::Group {
          children: vec![Group::Garbage("".to_string())]
        },
        Group::Group {
          children: vec![Group::Garbage("".to_string())]
        },
        Group::Group {
          children: vec![Group::Garbage("".to_string())]
        },
        Group::Group {
          children: vec![Group::Garbage("".to_string())]
        },
      ]
    }),
    9
  );
  assert_eq!(
    score_group(&Group::Group {
      children: vec![Group::Group {
        children: vec![Group::Garbage("".to_string())]
      }]
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
