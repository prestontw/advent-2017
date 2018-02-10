// use a hashmap of hashmaps,
// a function to get indices to look up
// map over these, sum up
// function to get next pair of indices based on position

use std::collections::HashMap;

pub fn first_value_bigger(target: usize) -> usize {
    let mut grid: HashMap<isize, HashMap<isize, usize>> = HashMap::new();
    let first: HashMap<isize, usize> = [(0, 1)].into_iter().cloned().collect();
    grid.insert(0, first);
    let mut coord = (1, 0);
    loop {
        let to_insert = sum_around(&grid, coord);
        if to_insert > target {
            return to_insert
        }
        grid.entry(coord.0).or_insert(HashMap::new())
            .entry(coord.1).or_insert(to_insert);
        coord = get_next_index(coord);
    }
}

enum Time {
    Two,
    Five,
    Eight,
    Eleven,
}

fn get_neighboring_indices(cur: (isize, isize)) -> Vec<(isize, isize)> {
    let (x, y) = cur;
    vec![(x - 1, y + 1), (x, y + 1), (x + 1, y + 1),
         (x - 1, y),                 (x + 1, y),
         (x - 1, y - 1), (x, y - 1), (x + 1, y - 1)]
}

fn sum_around(grid: &HashMap<isize, HashMap<isize, usize>>, coord: (isize, isize)) -> usize {
    let neighbors_indices = get_neighboring_indices(coord);
    neighbors_indices.into_iter().map(|(x, y)|
                                      if let Some(ref d) = grid.get(&x) {
                                          *d.get(&y).unwrap_or(&0)
                                      }
                                      else {
                                          0
                                      }
                                      ).sum()
} 

fn produce_quadrant(x: isize, y: isize) -> Time {
    use Time::*;
    if x >= 0 {
        if y >= 0 {
            Two
        }
        else {
            Five
        }
    }
    else {
        if y >= 0 {
            Eleven
        }
        else {
            Eight
        }
    }
}

fn get_next_index(cur: (isize, isize)) -> (isize, isize) {
    let (x, y) = cur;
    if cur == (0, 0) {
        (1, 0)
    }
    else {
        match produce_quadrant(x, y) {
            Time::Two => if x > y {
                (x, y + 1)
            }
            else {
                (x - 1, y)
            },
            Time::Eleven => if -x < y {
                (x - 1, y)
            }
            else {
                (x, y - 1)
            },
            Time::Eight => if y - 1 > x {
                (x, y - 1)
            }
            else {
                (x + 1, y)
            },
            Time::Five => if x <= -y {
                (x + 1, y)
            }
            else {
                (x, y + 1)
            }
        }
    }
}
#[test]
fn test_next_index() {
    assert_eq!(get_next_index((0, 0)), (1, 0));
    assert_eq!(get_next_index((1, 0)), (1, 1));
    assert_eq!(get_next_index((1, 1)), (0, 1));
    assert_eq!(get_next_index((0, 1)), (-1, 1));
    assert_eq!(get_next_index((-1, 1)), (-1, 0));
    assert_eq!(get_next_index((-1, 0)), (-1, -1));
    assert_eq!(get_next_index((-1, -1)), (0, -1));
    assert_eq!(get_next_index((0, -1)), (1, -1));
    assert_eq!(get_next_index((0, -1)), (1, -1));
    assert_eq!(get_next_index((1, -1)), (2, -1));
    assert_eq!(get_next_index((2, -1)), (2, 0));
}

pub fn distance(s: usize) -> usize {
    let (num_square, lower_right_number) = get_nth_square(s);
    let quads = get_quadrants(lower_right_number, num_square);
    num_square + get_closest_quadrant(s, quads) - 1
        // have a get lower left function? then with this and the number of the square,
        // can subtract once to get lower mid, again to get left mid, again to get up mid,
        // and again to get right mid.
        // take absolute value of the difference between mid's and the value,
        // take the min of this, and add this to the num-square
}
#[test]
fn test_distance() {
    assert_eq!(distance(1), 0);
    assert_eq!(distance(12), 3);
    assert_eq!(distance(23), 2);
    assert_eq!(distance(1024), 31);
}

fn get_quadrants(lower_right: usize, nth_square: usize) -> Vec<usize> {
    let diff = nth_square - 1;
    let lower_mid = lower_right - diff;
    vec![lower_mid,
         lower_mid - 2 * diff,
         lower_mid - 4 * diff,
         lower_mid - 6 * diff]
}
#[test]
fn test_get_quadrants() {
    assert_eq!(get_quadrants(9, 2), vec![8, 6, 4, 2]);
    assert_eq!(get_quadrants(25, 3), vec![23, 19, 15, 11]);
}

fn get_closest_quadrant<I>(num: usize, quads: I) -> usize where I: IntoIterator<Item = usize> {
    let num: isize = num as isize;
    quads.into_iter().map(|c| (c as isize - num).abs()).min().unwrap() as usize
}

fn amount_in_this_square(num_inner: usize) -> usize {
    4 * (num_inner - 1) + 8
}

fn get_nth_square(s: usize) -> (usize, usize) {
    if s == 1 {
        (1, 1)
    }
    else {
        let mut num_inner = 1;
        let mut total = 1 + amount_in_this_square(num_inner);
        let mut count = 2;
        while s > total {
            num_inner += 2;
            total += amount_in_this_square(num_inner);
            count += 1;
        }
        (count, total)
    }
}
#[test]
fn test_nth_square() {
    assert_eq!(get_nth_square(1), (1, 1));
    assert_eq!(get_nth_square(2), (2, 9));
    assert_eq!(get_nth_square(4), (2, 9));
    assert_eq!(get_nth_square(9), (2, 9));
    assert_eq!(get_nth_square(10), (3, 25));
    assert_eq!(get_nth_square(19), (3, 25));
    assert_eq!(get_nth_square(25), (3, 25));
    assert_eq!(get_nth_square(26), (4, 49));
}
