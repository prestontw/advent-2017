extern crate day10;

fn main() {
    let input = "106,16,254,226,55,2,1,166,177,247,93,0,255,228,60,36";
    let lengths = day10::input_to_list(input);
    let lst = day10::hash_list(256, &lengths[..]);
    println!("{}", day10::get_hash_of_list(&lst));
}
