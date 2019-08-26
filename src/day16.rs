#[derive(Debug, PartialEq)]
enum DanceMoves {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

trait Dance {
    fn spin(&mut self, s: usize) {}
    fn exchange(&mut self, one: usize, two: usize) {}
    fn partner(&mut self, one: char, two: char){}
}

impl Dance for Vec<char> {
    fn spin(&mut self, s: usize) {
        self.rotate_right(s);
    }
    fn exchange(&mut self, one: usize, two: usize) {
        self.swap(one, two);
    }

    fn partner(&mut self, one: char, two: char) {
        let one: usize = self.iter().position(|&c| c == one).unwrap();
        let two: usize = self.iter().position(|&c| c == two).unwrap();
        self.exchange(one, two);
    }
}

pub fn part1(i: &str) -> Vec<char> {
    let mut line: Vec<char> = (b'a'..=b'p').map(char::from).collect::<Vec<_>>();
    let instructions = parse_instructions(i);
    execute_dance(&mut line, &instructions);
    line
}

pub fn part2(i: &str) -> Vec<char> {
    let mut line: Vec<char> = (b'a'..=b'p').map(char::from).collect::<Vec<_>>();
    // 0p 1k 2g 3n 4h 5o 6m 7e 8l 9f 10d 11i 12b 13j 14a 15c
    // could probably do something with remainder of a billion and apply that?
    let permutate: Vec<usize> = vec![14, 12, 15, 10, 7, 9, 2, 4, 11, 13, 1, 8, 6, 3, 5, 0];
    let instructions = parse_instructions(i);
    for _ in 0..1_000_000_000 {
        execute_dance(&mut line, &instructions);
    }
    line
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
    for instruction in instructions {
        match instruction {
            DanceMoves::Spin(n) => line.spin(*n),
            DanceMoves::Exchange(a, b) => line.exchange(*a, *b),
            DanceMoves::Partner(a, b) => line.partner(*a, *b),
        }
    }
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