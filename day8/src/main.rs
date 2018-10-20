extern crate day8;
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
    let instructions = day8::get_instructions(&input);
    let registers = day8::eval_instructions(&instructions);
    let biggest = biggest_register(&registers);
    println!("{}", biggest);
}