extern crate advent_2017;
use advent_2017::day12;
use std::io::{self, Read};

fn main() {
    let input = {
        let mut buffer = String::new();
        let res = io::stdin().read_to_string(&mut buffer);
        if let Err(e) = res {
            panic!("could not read from stdin: {}", e)
        }
        buffer
    };
    println!("{}", day12::part1(&input));
    println!("{}", day12::num_groups(&input));
}