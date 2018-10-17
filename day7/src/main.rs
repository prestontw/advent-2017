extern crate day7;
use std::io::{self, Read};

fn main() {
    let input_part1 = {
        let mut buffer = String::new();
        let res = io::stdin().read_to_string(&mut buffer);
        if let Err(e) = res {
            panic!("could not read from stdin: {}", e)
        }
        buffer
    };
    println!("{}", day7::bottom_program(&input_part1));
}
