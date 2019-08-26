#[derive(Debug, PartialEq)]
enum DanceMoves {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

fn parse_instructions(i: &str) -> Vec<DanceMoves> {
    i.split(',').map(|segment| parse_segment(segment)).collect()
}

fn parse_segment(i: &str) -> DanceMoves {
    let remainder: String = i.chars().skip(1).collect();
    match i.chars().nth(0).unwrap() {
        's' => {
            DanceMoves::Spin(remainder.parse::<usize>().unwrap())
        }
        'x' => {
            let split: Vec<&str> = remainder.split('/').collect();
            let first = split[0];
            let second = split[1];
            DanceMoves::Exchange(first.parse::<usize>().unwrap(), second.parse::<usize>().unwrap())
        }
        'p' => {
            let split: Vec<&str> = remainder.split('/').collect();
            let first = split[0].chars().nth(0).unwrap();
            let second = split[1].chars().nth(0).unwrap();
            DanceMoves::Partner(first, second)
        }
        _ => unimplemented!()
    }
}

fn execute_dance(line: &mut Vec<char>, instructions: &[DanceMoves]) {

}

#[test]
fn test_dance_move_sequence() {
    let mut line = vec!['a', 'b', 'c', 'd', 'e'];
    let instructions = vec![DanceMoves::Spin(1),
    DanceMoves::Exchange(3, 4), DanceMoves::Partner('e', 'b')];
    execute_dance(&mut line, &instructions);
    assert_eq!(line, vec!['b', 'a', 'e', 'd', 'c']);
}

#[test]
fn test_parse() {
    let instructions = "s1,x3/4,pe/b";
    assert_eq!(parse_instructions(instructions), vec![DanceMoves::Spin(1),
    DanceMoves::Exchange(3, 4), DanceMoves::Partner('e', 'b')]);
}