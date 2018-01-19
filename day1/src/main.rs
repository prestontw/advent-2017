extern crate day1;
use day1::{captcha, string_to_vec};

fn main() {
    let input = "11";
    let result = captcha(&string_to_vec(input)[..]);
    println!("{}", result);
}
