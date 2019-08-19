extern crate day6;

fn main() {
    let input = "5	1	10	0	1	7	13	14	3	12	8	10	7	12	0	6";
    println!("{}", day6::num_to_repeat(day6::line_to_vec(input)));
    println!("{}", day6::num_in_cycle(day6::line_to_vec(input)));
}
