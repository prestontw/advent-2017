extern crate advent_2017;
use advent_2017::day3;

fn main() {
    let input = 277678;
    println!("{}", day3::distance(input));
    println!("{}", day3::first_value_bigger(input));
}
