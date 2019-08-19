#[macro_use]
extern crate nom;
pub use self::parse::group;

mod parse {
  use super::Group;
  use nom::types::CompleteStr;

  named!(garbage<CompleteStr, Group>,
  do_parse!(
    body: delimited!(char!('<'), opt!(is_not!(">")), char!('>')) >>
    (Group::Garbage(match body {
      Some(cs) => cs.to_string(),
      None => "".to_string()
    }))));
  #[test]
  fn test_garbage() {
    assert_eq!(
      garbage(CompleteStr("<<<<>")),
      Ok((CompleteStr(""), Group::Garbage("<<<".to_string())))
    );
  }

  /**
   * parsing a group can either be:
   * empty group: {};
   * garbage: garbage parser for this!;
   * or recursive group: {group};
   *
   * Really, can combine the first and third as many0!(group).
   * And, have to think of it many0!(alt!(group | garbage))
   * since can alternate groups and garbage
   * Need to include commas in here too...
   */
  named!(pub group<CompleteStr, Group>,
  do_parse!(
    tag!("{") >>
    children:
      separated_list!(tag!(","), alt!(group | garbage)) >>
    tag!("}") >>
    (Group::Group { children: children })
  )
);
  #[test]
  fn test_parse_groups() {
    assert_eq!(
      group(CompleteStr("{}")),
      Ok((
        CompleteStr(""),
        Group::Group {
          children: Vec::new()
        }
      ))
    );
    assert_eq!(
      group(CompleteStr("{{{<,},<>}}}")),
      Ok((
        CompleteStr(""),
        Group::Group {
          children: vec![Group::Group {
            children: vec![Group::Group {
              children: vec![Group::Garbage("".to_string())]
            }]
          }]
        }
      ))
    );
    assert_eq!(
      group(CompleteStr("{{}}")),
      Ok((
        CompleteStr(""),
        Group::Group {
          children: vec![Group::Group {
            children: Vec::new()
          }]
        }
      ))
    );
    assert_eq!(
      group(CompleteStr("{{},{}}")),
      Ok((
        CompleteStr(""),
        Group::Group {
          children: vec![
            Group::Group {
              children: Vec::new()
            },
            Group::Group {
              children: Vec::new()
            }
          ]
        }
      ))
    );
    assert_eq!(
      group(CompleteStr("{{{},{},{{}}}}")),
      Ok((
        CompleteStr(""),
        Group::Group {
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
        }
      ))
    );
    assert_eq!(
      group(CompleteStr("{<a>,<a>,<a>,<a>}")),
      Ok((
        CompleteStr(""),
        Group::Group {
          children: vec![
            Group::Garbage("".to_string()),
            Group::Garbage("".to_string()),
            Group::Garbage("".to_string()),
            Group::Garbage("".to_string())
          ]
        }
      ))
    );
    assert_eq!(
      group(CompleteStr("{{<ab>},{<ab>},{<ab>},{<ab>}}")),
      Ok((
        CompleteStr(""),
        Group::Group {
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
        }
      ))
    );
    assert_eq!(
      group(CompleteStr("{{<a},{<a},{<a},{<ab>}}")),
      Ok((
        CompleteStr(""),
        Group::Group {
          children: vec![Group::Group {
            children: vec![Group::Garbage("".to_string())]
          }]
        }
      ))
    );
    assert_eq!(
      group(CompleteStr("{{{{<>}}}}")),
      Ok((
        CompleteStr(""),
        Group::Group {
          children: vec![Group::Group {
            children: vec![Group::Group {
              children: vec![Group::Group {
                children: vec![Group::Garbage("".to_string())]
              }]
            }]
          }]
        }
      ))
    );
  }

}

#[derive(Debug, PartialEq, Clone)]
pub enum Group {
  Garbage(String),
  Group { children: Vec<Group> },
}

pub fn count_chars_in_garbage(g: &Group) -> usize {
  match g {
    Group::Garbage(s) => s.len(),
    Group::Group { children } => children.iter().map(count_chars_in_garbage).sum(),
  }
}
#[test]
fn test_counting() {
  assert_eq!(
    count_chars_in_garbage(&Group::Group {
      children: vec![
        Group::Garbage("123".to_string()),
        Group::Garbage("12".to_string()),
      ]
    }),
    5
  );
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
