#[derive(Debug, PartialEq)]
struct SpinLock {
    position: usize,
    buffer: Vec<usize>,
}

impl SpinLock {
    fn new(capacity: usize) -> SpinLock {
        let mut v = Vec::with_capacity(capacity);
        v.push(0);
        SpinLock {
            position: 0,
            buffer: v,
        }
    }
    fn step(&mut self, num_steps: usize) {
        let len = self.buffer.len();
        self.position = (self.position + num_steps) % len;
    }
    fn insert(&mut self, value: usize) {
        if self.position + 1 >= self.buffer.len() {
            self.buffer.push(value);
        } else {
            self.buffer.insert(self.position + 1, value);
        }
        self.position += 1;
    }
    fn buffer(&self) -> Vec<usize> {
        self.buffer.clone()
    }
}

#[test]
fn test_inserts() {
    let mut spin = SpinLock::new(10);
    let steps = 3;
    spin.step(steps);
    spin.insert(1);
    assert_eq!(spin.buffer(), vec![0, 1]);
    spin.step(steps);
    spin.insert(2);
    assert_eq!(spin.buffer(), vec![0, 2, 1]);
    spin.step(steps);
    spin.insert(3);
    assert_eq!(spin.buffer(), vec![0, 2, 3, 1]);
    spin.step(steps);
    spin.insert(4);
    assert_eq!(spin.buffer(), vec![0, 2, 4, 3, 1]);
    spin.step(steps);
    spin.insert(5);
    assert_eq!(spin.buffer(), vec![0, 5, 2, 4, 3, 1]);
    spin.step(steps);
    spin.insert(6);
    assert_eq!(spin.buffer(), vec![0, 5, 2, 4, 3, 6, 1]);
    spin.step(steps);
    spin.insert(7);
    assert_eq!(spin.buffer(), vec![0, 5, 7, 2, 4, 3, 6, 1]);
    spin.step(steps);
    spin.insert(8);
    assert_eq!(spin.buffer(), vec![0, 5, 7, 2, 4, 3, 8, 6, 1]);
    spin.step(steps);
    spin.insert(9);
    assert_eq!(spin.buffer(), vec![0, 9, 5, 7, 2, 4, 3, 8, 6, 1]);
}

fn naive_approach(steps: usize, rounds: usize, interest: usize) -> usize {
    // adding on assumed zero
    let rounds = rounds + 1;
    let mut spin = SpinLock::new(rounds);
    for i in 1..rounds {
        spin.step(steps);
        spin.insert(i);
    }
    let index = spin.buffer().into_iter().position(|c| c == interest).unwrap();
    spin.buffer()[(index + 1) % rounds]
}

pub fn part1(steps: usize) -> usize {
    naive_approach(steps, 2017, 2017)
}

pub fn part2(steps: usize) -> usize {
    let size = 50_000_000;
    let mut spin = SpinLock::new(size);
    for i in 1..=size {
        spin.step(steps);
        spin.insert(i);
    }
    let index = spin.buffer().into_iter().position(|c| c == 0).unwrap();
    spin.buffer()[(index + 1) % size]
}

#[test]
fn test_part1() {
    assert_eq!(part1(3), 638);
    assert_eq!(part1(348), 417);
}