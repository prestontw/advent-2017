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
