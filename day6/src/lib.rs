use std::collections::HashSet;

fn num_to_repeat<'a, I>(start: I) -> usize where I: IntoIterator<Item = &'a usize> {
    0
}
#[test]
fn test_num_to_repeat() {
    assert_eq!(num_to_repeat(&vec![0, 2, 7, 0]), 5);
}

fn already_seen_config(seen: &HashSet<Vec<isize>>, new: &Vec<isize>) -> bool {
    seen.contains(new)
}
#[test]
fn test_given_example() {
    let mut hs = HashSet::new();
    // don't do this since we are doing block redistribution cycles
    // hs.insert(vec![0, 2, 7, 0]);
    hs.insert(vec![2, 4, 1, 2]);
    hs.insert(vec![3, 1, 2, 3]);
    hs.insert(vec![0, 2, 3, 4]);
    assert!(!already_seen_config(&hs, &vec![1, 3, 4, 1]));
    hs.insert(vec![1, 3, 4, 1]);
    assert!(already_seen_config(&hs, &vec![2, 4, 1, 2]));
}

// if we take ownership over param, will that borrow out from hashset?
// it will! so need to do & instead
fn next_blocks(cur: &[usize]) -> Vec<usize> {
    let (max_index, max_value) = get_largest_values_index(cur).unwrap();
    let mut ret: Vec<usize> = cur.into_iter().cloned().collect();
    ret[max_index] = 0;
    let distances = distances_from(ret.len(), max_index);
    for (index, distance) in distances.into_iter().enumerate() {
        ret[index]
    }
    ret
}
#[test]
fn test_next_blocks() {
    let first = &[0, 2, 7, 0];
    let secon = &[2, 4, 1, 2];
    let third = &[3, 1, 2, 3];
    let fourt = &[0, 2, 3, 4];
    let fifth = &[1, 3, 4, 1];
    let sixth = &[2, 4, 1, 2];

    assert_eq!(next_blocks(first), secon);
    assert_eq!(next_blocks(secon), third);
    assert_eq!(next_blocks(third), fourt);
    assert_eq!(next_blocks(fourt), fifth);
    assert_eq!(next_blocks(fifth), sixth);
}

fn amount_increase_per_index(distance_from: usize, length: usize, amount: usize) -> usize {
    let base = amount / length;
    let remainder = amount % length;
    let even = remainder == 0;
    if even || distance_from == 0{
        base
    }
    else {
        if distance_from <= remainder {
            base + 1
        }
        else {
            base
        }
    }
}
#[test]
fn test_amount_increase() {
    assert_eq!(amount_increase_per_index(0, 5, 4), 0);
    assert_eq!(amount_increase_per_index(1, 5, 4), 1);
    assert_eq!(amount_increase_per_index(4, 5, 4), 1);
    assert_eq!(amount_increase_per_index(0, 5, 8), 1);
    assert_eq!(amount_increase_per_index(1, 5, 8), 2);
    assert_eq!(amount_increase_per_index(3, 5, 8), 2);
    assert_eq!(amount_increase_per_index(4, 5, 8), 1);
}

fn distances_from(length: usize, position: usize) -> Vec<usize> {
    let increase = length - position;
    (0..length).map(|c| (c + increase) % length).collect()
}
#[test]
fn test_distance_from() {
    assert_eq!(distances_from(5, 3), vec![2, 3, 4, 0, 1]);
    assert_eq!(distances_from(4, 3), vec![1, 2, 3, 0]);
    assert_eq!(distances_from(6, 0), vec![0, 1, 2, 3, 4, 5]);
}

fn get_largest_values_index(l: &[usize]) -> Option<(usize, usize)> {
    l.into_iter().cloned().enumerate()
        .fold(None, |acc, cur|
            match acc {
                None => Some(cur),
                Some((_, val)) => if cur.1 > val { Some(cur) } else { acc }
            })
}
#[test]
fn test_largest_index() {
    assert_eq!(get_largest_values_index(&[0, 3, 5, 2, 1]), Some((2, 5)));
}