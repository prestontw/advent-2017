use std::collections::HashSet;

fn already_seen_config(seen: &HashSet<Vec<isize>>, new: Vec<isize>) -> bool {
    false
}
#[test]
fn test_given_example() {
    let mut hs = HashSet::new();
    // don't do this since we are doing block redistribution cycles
    // hs.insert(vec![0, 2, 7, 0]);
    hs.insert(vec![2, 4, 1, 2]);
    hs.insert(vec![3, 1, 2, 3]);
    hs.insert(vec![0, 2, 3, 4]);
    assert!(!already_seen_config(&hs, vec![1, 3, 4, 1]));
    hs.insert(vec![1, 3, 4, 1]);
    assert!(already_seen_config(&hs, vec![2, 4, 1, 2]));
}

// if we take ownership over param, will that borrow out from hashset?
fn next_blocks(cur: Vec<usize>) -> Vec<usize> {
    cur
}
#[test]
fn test_next_blocks() {
    let first = vec![0, 2, 7, 0];
    let secon = vec![2, 4, 1, 2];
    let third = vec![3, 1, 2, 3];
    let fourt = vec![0, 2, 3, 4];
    let fifth = vec![1, 3, 4, 1];
    let sixth = vec![2, 4, 1, 2];

    assert_eq!(next_blocks(first.clone()), secon);
    assert_eq!(next_blocks(secon.clone()), third);
    assert_eq!(next_blocks(third.clone()), fourt);
    assert_eq!(next_blocks(fourt.clone()), fifth);
    assert_eq!(next_blocks(fifth.clone()), sixth);

    let mut hs = HashSet::new();
    hs.insert(first);
    let new = next_blocks(first);
}
