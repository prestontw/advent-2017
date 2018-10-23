extern crate day9;
use std::io::{self, Read};
use nom::types::CompleteStr;

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
    }
    else {
        println!("could not parse input");
    }
}
