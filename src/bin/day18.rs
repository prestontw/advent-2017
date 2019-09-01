extern crate advent_2017;
use advent_2017::day18;
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
    let res1 = day18::part1(&input);
    println!("{:?}", res1);
}
