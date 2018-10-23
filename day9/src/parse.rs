use super::Group;
use nom::types::CompleteStr;

named!(garbage<CompleteStr, Group>,
  do_parse!(
    _body: delimited!(char!('<'), opt!(is_not!(">")), char!('>')) >>
    (Group::Garbage)));
#[test]
fn test_garbage() {
  assert_eq!(
    garbage(CompleteStr("<<<<>")),
    Ok((CompleteStr(""), Group::Garbage))
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
            children: vec![Group::Garbage]
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
        children: vec![
          Group::Group {
            children: Vec::new()
          }
        ]
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
          Group::Garbage,
          Group::Garbage,
          Group::Garbage,
          Group::Garbage
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
            children: vec![Group::Garbage]
          },
          Group::Group {
            children: vec![Group::Garbage]
          },
          Group::Group {
            children: vec![Group::Garbage]
          },
          Group::Group {
            children: vec![Group::Garbage]
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
          children: vec![Group::Garbage]
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
              children: vec![Group::Garbage]
            }]
          }]
        }]
      }
    ))
  );
}
