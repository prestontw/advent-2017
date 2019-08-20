extern crate advent_2017;
use advent_2017::day7;
use std::env;
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
    let args: Vec<String> = env::args().collect();
    let action = &args[1];
    match &action[..] {
        "part1" => println!("{}", day7::bottom_program(&input)),
        "part2" => println!("{}", day7::balanced_weight(&input)),
        _ => println!("not supported")
    }
}
