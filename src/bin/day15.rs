extern crate advent_2017;
use advent_2017::day15;

fn main() {
    let input_a = 289;
    let input_b = 629;
    println!("{}", day15::part1(input_a, input_b, 40000000));
    println!("{}", day15::part2(input_a, input_b, 5_000_000));
}
