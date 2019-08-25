struct Generator {
    prev: usize,
    factor: usize,
}

impl Generator {
    fn new(start: usize, factor: usize) -> Generator {
        Generator {
            prev: start,
            factor,
        }
    }
}

impl Iterator for Generator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.prev = (self.prev * self.factor) % 2147483647;
        Some(self.prev)
    }
}

const SIXTEEN_BITS: usize = 0b1111111111111111;

fn sixteen_bits(u: usize) -> usize {
    u & SIXTEEN_BITS
}

pub fn part1(a: usize, b: usize, len: usize) -> usize {
    let a = Generator::new(a, 16807);
    let b = Generator::new(b, 48271);

    a.into_iter().zip(b.into_iter()).take(len).filter(|&(a, b)| sixteen_bits(a) == sixteen_bits(b)).count()
}

#[test]
fn test_part1() {
    assert_eq!(part1(65, 8921, 5), 1);
    assert_eq!(part1(65, 8921, 40000000), 588);
}

#[test]
fn test_sequences() {
    let a = Generator::new(65, 16807);
    assert_eq!(
        a.into_iter().take(5).collect::<Vec<_>>(),
        vec![1092455, 1181022009, 245556042, 1744312007, 1352636452]
    );
    let b = Generator::new(8921, 48271);
    assert_eq!(
        b.into_iter().take(5).collect::<Vec<_>>(),
        vec![430625591, 1233683848, 1431495498, 137874439, 285222916]
    )
}
