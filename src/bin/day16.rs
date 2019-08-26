extern crate advent_2017;
use advent_2017::day16;
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
    let res1 = day16::part1(&input);
    println!("{}", res1.into_iter().collect::<String>());
    let res2 = day16::part2(&input);
    println!("{}", res2.into_iter().collect::<String>());
    // pkgnhomelfdibjac
}
