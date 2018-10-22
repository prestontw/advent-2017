named!(garbage<CompleteStr, CompleteStr>, delimited!(char!('<'), is_not!(">"), char!('>')));
#[test]
fn test_garbage() {
  assert_eq!(garbage(CompleteStr("<<<<>")), 
  Ok((CompleteStr(""), CompleteStr("<<<"))));
}
