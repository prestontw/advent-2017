#[derive(Debug, PartialEq)]
enum DanceMoves {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

trait Dance {
    fn spin(&mut self, _: usize) {}
    fn exchange(&mut self, _: usize, _: usize) {}
    fn partner(&mut self, _: char, _: char) {}
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
    let original = line.clone();
    let instructions = parse_instructions(i);
    // can't permutate because have partner! not just index based permutation
    // permutate(&mut line, &permutation, 1_000_000_000);
    let mut repeat = None;
    const TIMES: usize = 1_000_000_000;
    for i in 0..TIMES {
        execute_dance(&mut line, &instructions);
        if line == original {
            println!("repeat: {}", i);
            repeat = Some(i + 1);
            break;
        }
    }
    match repeat {
        None => line,
        Some(v) => {
            // remainder of 1_000_000_000 and v, then apply that many times
            let actual = TIMES % v;
            for _i in 0..actual {
                execute_dance(&mut line, &instructions);
            }
            line
        }
    }
}

#[allow(dead_code)]
fn permutate_in_place<A>(l: &mut [A], switches: &[usize], times: usize)
where
    A: Copy + std::fmt::Debug,
{
    let mut seen: Vec<usize> = (0..l.len()).collect();
    while !seen.is_empty() {
        let start = seen.pop().unwrap();
        let cycle = get_cycle(switches, start);
        println!("{:?}", cycle);
        let count = times % cycle.len();
        for _ in 0..count {
            apply_permutation(l, &cycle);
            println!("done: {:?}", l);
        }
        for index in cycle {
            // can switch this to be binary search/remove
            seen.remove_item(&index);
        }
    }
}

#[allow(dead_code)]
fn permute_once<A>(l: &[A], switches: &[usize]) -> Vec<A>
where
A: Copy
{
    let mut ret = l.clone().to_vec();
    for (index, value) in l.iter().enumerate() {
        ret[switches[index]] = *value;
    }
    ret
}

#[test]
fn test_permutate() {
    let mut arr = vec!['a', 'b', 'c', 'd', 'e'];
    let cycle = vec![1, 0, 4, 2, 3];

    permutate_in_place(&mut arr, &cycle, 1);
    assert_eq!(arr, vec!['b', 'a', 'd', 'e', 'c']);

    permutate_in_place(&mut arr, &cycle, 1);
    assert_eq!(arr, vec!['a', 'b', 'e', 'c', 'd']);

    permutate_in_place(&mut arr, &cycle, 1);
    assert_eq!(arr, vec!['b', 'a', 'c', 'd', 'e']);

    permutate_in_place(&mut arr, &cycle, 1);
    assert_eq!(arr, vec!['a', 'b', 'd', 'e', 'c']);

    permutate_in_place(&mut arr, &cycle, 1);
    assert_eq!(arr, vec!['b', 'a', 'e', 'c', 'd']);

    permutate_in_place(&mut arr, &cycle, 1);
    assert_eq!(arr, vec!['a', 'b', 'c', 'd', 'e']);

    permutate_in_place(&mut arr, &cycle, 6);
    assert_eq!(arr, vec!['a', 'b', 'c', 'd', 'e']);
}

fn apply_permutation<A>(l: &mut [A], cycle: &[usize])
where
    A: Copy + std::fmt::Debug,
{
    if cycle.len() > 1 {
        let temp = l[cycle[cycle.len() - 1]];
        // shift everything to the right
        for pair in cycle.windows(2).rev() {
            l[pair[1]] = l[pair[0]];
            println!("{:?}", l);
        }
        l[cycle[0]] = temp;
        println!("{:?}", l);
    }
}

#[test]
fn test_apply_permutation() {
    let mut arr = vec!['a', 'b'];
    apply_permutation(&mut arr, &vec![1, 0]);
    assert_eq!(arr, vec!['b', 'a']);

    let mut arr = vec!['a', 'b', 'c', 'd'];
    apply_permutation(&mut arr, &vec![0, 3, 2, 1]);
    assert_eq!(arr, vec!['b', 'c', 'd', 'a']);

    let mut arr = vec!['a', 'b', 'c', 'd', 'e', 'f'];
    let cycle = vec![0, 5, 2, 4, 3, 1];
    apply_permutation(&mut arr, &cycle);
    assert_eq!(arr, vec!['b', 'd', 'f', 'e', 'c', 'a']);

    apply_permutation(&mut arr, &cycle);
    assert_eq!(arr, vec!['d', 'e', 'a', 'c', 'f', 'b']);

    apply_permutation(&mut arr, &cycle);
    assert_eq!(arr, vec!['e', 'c', 'b', 'f', 'a', 'd']);

    apply_permutation(&mut arr, &cycle);
    assert_eq!(arr, vec!['c', 'f', 'd', 'a', 'b', 'e']);

    apply_permutation(&mut arr, &cycle);
    assert_eq!(arr, vec!['f', 'a', 'e', 'b', 'd', 'c']);

    apply_permutation(&mut arr, &cycle);
    assert_eq!(arr, vec!['a', 'b', 'c', 'd', 'e', 'f']);

    let mut arr = vec!['a', 'b', 'c', 'd'];
    apply_permutation(&mut arr, &vec![1, 0]);
    assert_eq!(arr, vec!['b', 'a', 'c', 'd'])
}

fn get_cycle(indices: &[usize], start: usize) -> Vec<usize> {
    let mut current_index = start;
    let mut ret = Vec::with_capacity(indices.len());
    ret.push(current_index);
    while indices[current_index] != start {
        current_index = indices[current_index];
        ret.push(current_index);
    }

    ret
}

#[test]
fn test_get_cycle() {
    //              0   1   2   3  4  5  6  7   8   9 10 11 12 13 14 15
    let arr = vec![14, 12, 15, 10, 7, 9, 2, 4, 11, 13, 1, 8, 6, 3, 5, 0];
    assert_eq!(
        get_cycle(&arr, 0),
        vec![0, 14, 5, 9, 13, 3, 10, 1, 12, 6, 2, 15]
    );
    assert_eq!(get_cycle(&arr, 4), vec![4, 7]);
    assert_eq!(get_cycle(&arr, 8), vec![8, 11]);

    let arr = vec![0, 2, 1];
    assert_eq!(get_cycle(&arr, 0), vec![0]);
    assert_eq!(get_cycle(&arr, 1), vec![1, 2]);
}

fn parse_instructions(i: &str) -> Vec<DanceMoves> {
    i.split(',').map(|segment| parse_segment(segment)).collect()
}

fn parse_segment(i: &str) -> DanceMoves {
    let remainder: String = i.chars().skip(1).collect();
    match i.chars().nth(0).unwrap() {
        's' => DanceMoves::Spin(remainder.parse::<usize>().unwrap()),
        'x' => {
            let split: Vec<&str> = remainder.split('/').collect();
            let first = split[0];
            let second = split[1];
            DanceMoves::Exchange(
                first.parse::<usize>().unwrap(),
                second.parse::<usize>().unwrap(),
            )
        }
        'p' => {
            let split: Vec<&str> = remainder.split('/').collect();
            let first = split[0].chars().nth(0).unwrap();
            let second = split[1].chars().nth(0).unwrap();
            DanceMoves::Partner(first, second)
        }
        _ => unimplemented!(),
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
    let instructions = vec![
        DanceMoves::Spin(1),
        DanceMoves::Exchange(3, 4),
        DanceMoves::Partner('e', 'b'),
    ];
    execute_dance(&mut line, &instructions);
    assert_eq!(line, vec!['b', 'a', 'e', 'd', 'c']);
}

#[test]
fn test_parse() {
    let instructions = "s1,x3/4,pe/b";
    assert_eq!(
        parse_instructions(instructions),
        vec![
            DanceMoves::Spin(1),
            DanceMoves::Exchange(3, 4),
            DanceMoves::Partner('e', 'b')
        ]
    );
}
