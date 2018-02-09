pub fn distance(s: usize) -> usize {
    let num_square = get_nth_square(s);
    num_square + get_closest_quadrant(s, num_square) - 1
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

fn get_nth_square(s: usize) -> usize {
    let mut total = 1;
    let mut side = 8;
    let mut square = 1;
    let two = 2;
    while s > total {
        total += side;
        square += 1;
        side *= 2;
    }
    square
}
#[test]
fn test_nth_square() {
    assert_eq!(get_nth_square(1), 1);
    assert_eq!(get_nth_square(2), 2);
    assert_eq!(get_nth_square(4), 2);
    assert_eq!(get_nth_square(9), 2);
    assert_eq!(get_nth_square(10), 3);
    assert_eq!(get_nth_square(19), 3);
    assert_eq!(get_nth_square(25), 3);
    assert_eq!(get_nth_square(26), 4);
}
