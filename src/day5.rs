fn out_of_bounds(a: &[isize], i: isize) -> bool {
    (i < 0) || (i >= a.len() as isize)
}
#[test]
fn test_out_of_bounds() {
    assert!(out_of_bounds(&vec![2, 4, 0, 1, -2][..], 1 + 4));
    assert!(!out_of_bounds(&vec![0, 3, 0, 1, -3][..], 0 + 0));
}

fn get_next_array2(mut v: Vec<isize>, index: usize) -> Vec<isize> {
    if v[index] >= 3 {
        v[index] -= 1;
    } else {
        v[index] += 1;
    }
    v
}

fn get_next_array(mut v: Vec<isize>, index: usize) -> Vec<isize> {
    v[index] += 1;
    v
}
#[test]
fn test_get_next_array() {
    assert_eq!(
        get_next_array(vec![0, 3, 0, 1, -3], 0),
        vec![1, 3, 0, 1, -3]
    );
    assert_eq!(
        get_next_array(vec![2, 4, 0, 1, -3], 4),
        vec![2, 4, 0, 1, -2]
    );
}

pub fn get_num_till_out_of_bounds2(mut v: Vec<isize>) -> usize {
    let mut count = 0;
    let mut index = 0;
    while !out_of_bounds(&v[..], index) {
        let old_index = index as usize;
        index = v[index as usize] + index;
        v = get_next_array2(v, old_index);
        count += 1;
    }
    count
}
#[test]
fn test_part_two() {
    assert_eq!(get_num_till_out_of_bounds2(vec![0, 3, 0, 1, -3]), 10);
}

pub fn get_num_till_out_of_bounds(mut v: Vec<isize>) -> usize {
    let mut count = 0;
    let mut index = 0;
    while !out_of_bounds(&v[..], index) {
        let old_index = index as usize;
        index = v[index as usize] + index;
        v = get_next_array(v, old_index);
        count += 1;
    }
    count
}
#[test]
fn test_example() {
    assert_eq!(get_num_till_out_of_bounds(vec![0, 3, 0, 1, -3]), 5);
}

pub fn string_to_vec(s: &str) -> Vec<isize> {
    s.lines().map(|l| l.parse().unwrap()).collect()
}
