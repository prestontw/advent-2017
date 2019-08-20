extern crate advent_2017;
use advent_2017::day9;
use nom::types::CompleteStr;
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
    let cleaned = day9::escape_bang_in_garbage(&input);
    if let Ok((_remainder, group)) = day9::group(CompleteStr(&cleaned)) {
        let score = day9::score_group(&group);
        println!("{}", score);
        let count = day9::count_chars_in_garbage(&group);
        println!("{}", count);
    } else {
        println!("could not parse input");
    }
}
