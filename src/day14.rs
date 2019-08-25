// use string to hash
use crate::day10;

pub fn oneify_hash(i: &str) -> Vec<Vec<usize>> {
    let i = i.to_string();
    let rows = (0..128)
        .into_iter()
        .map(|n| i.clone() + "-" + &n.to_string());
    let knots: Vec<String> = rows.map(|r| day10::string_to_hash_string(&r)).collect();
    // format each character as binary
    let bins: Vec<Vec<char>> = knots
        .into_iter()
        .map(|k| k.chars().flat_map(|c| string_to_binary(c)).collect())
        .collect();
    let nums = bins
        .into_iter()
        .map(|r| {
            r.into_iter()
                .map(|c| if c == '0' { 0 } else { 1 })
                .collect()
        })
        .collect();
    // then map each 1 to #, 0 to .
    nums
}

pub fn gridify_hash(i: &str) -> Vec<String> {
    let res: Vec<Vec<char>> = oneify_hash(i)
        .into_iter()
        .map(|r| {
            r.into_iter()
                .map(|n| if n == 1 { '#' } else { '.' })
                .collect()
        })
        .collect();
    res.into_iter().map(|r| r.into_iter().collect()).collect()
}

pub fn part1(i: &str) -> usize {
    oneify_hash(i)
        .into_iter()
        .map(|r| r.into_iter().sum::<usize>())
        .sum::<usize>()
}

pub fn part2(i: &str) -> usize {
    let grid = oneify_hash(i);
    regions(grid)
}

fn regions(g: Vec<Vec<usize>>) -> usize {
    let height = g.len();
    let width = g[0].len();
    let mut regions = 0;

    let mut temp = vec![];
    for _i in 0..height {
        let row = (0..width).into_iter().map(|_i| 0).collect::<Vec<usize>>();
        temp.push(row);
    }

    for i in 0..height {
        for j in 0..width {
            // check to see if temp[i][j] is nonzero
            if temp[i][j] == 0 {
                if g[i][j] != 0 {
                    // then we have a new region!
                    regions += 1;
                    spread_region(i, j, regions, &mut temp, &g);
                }
            }
        }
    }

    println!("{:?}", temp);
    regions
}

#[test]
fn test_regions() {
    let grid = vec![vec![1, 0, 0], vec![0, 1, 1], vec![1, 1, 1]];
    assert_eq!(regions(grid), 2);
}

fn spread_region(
    i: usize,
    j: usize,
    region: usize,
    grid: &mut [Vec<usize>],
    reference: &[Vec<usize>],
) {
    let index = grid.get(i).and_then(|row| row.get(j));
    println!("{:?}", index);
    if index == Some(&0) {
        if reference[i][j] != 0 {
            grid[i][j] = region;
            if i > 0 {
                spread_region(i - 1, j, region, grid, reference);
            }
            spread_region(i + 1, j, region, grid, reference);
            if j > 0 {
                spread_region(i, j - 1, region, grid, reference);
            }
            spread_region(i, j + 1, region, grid, reference);
        }
    }
}

#[test]
fn test_spread_region() {
    let three_by_three = vec![vec![1, 0, 0], vec![0, 1, 1], vec![1, 1, 1]];
    let mut grid = vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]];
    spread_region(0, 0, 1, &mut grid, &three_by_three);
    assert_eq!(grid, vec![vec![1, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]);

    spread_region(1, 1, 2, &mut grid, &three_by_three);
    assert_eq!(grid, vec![vec![1, 0, 0], vec![0, 2, 2], vec![2, 2, 2]]);

    let saved_grid = grid.clone();
    spread_region(100, 100, 20, &mut grid, &three_by_three);
    assert_eq!(grid, saved_grid);
}

pub fn string_to_number(i: char) -> Option<u32> {
    i.to_digit(16)
}
pub fn string_to_binary(i: char) -> Vec<char> {
    let num = string_to_number(i).unwrap();
    let binary = format!("{:04b}", num);
    binary.chars().collect()
}
#[test]
fn test_string_to_binary() {
    assert_eq!(
        string_to_binary('0'),
        vec!['0', '0'.into(), '0'.into(), '0'.into()]
    );
    assert_eq!(
        string_to_binary('1'),
        vec!['0', '0'.into(), '0'.into(), '1'.into()]
    );
    assert_eq!(
        string_to_binary('2'),
        vec!['0', '0'.into(), '1'.into(), '0'.into()]
    );
    assert_eq!(
        string_to_binary('3'),
        vec!['0', '0'.into(), '1'.into(), '1'.into()]
    );
    assert_eq!(
        string_to_binary('4'),
        vec!['0', '1'.into(), '0'.into(), '0'.into()]
    );
    assert_eq!(
        string_to_binary('8'),
        vec!['1', '0'.into(), '0'.into(), '0'.into()]
    );
    assert_eq!(
        string_to_binary('a'),
        vec!['1', '0'.into(), '1'.into(), '0'.into()]
    );
    assert_eq!(
        string_to_binary('b'),
        vec!['1', '0'.into(), '1'.into(), '1'.into()]
    );
    assert_eq!(
        string_to_binary('f'),
        vec!['1', '1'.into(), '1'.into(), '1'.into()]
    );
}

#[test]
fn test_gridify_hash() {
    let result = gridify_hash("flqrgnkx");
    assert_eq!(&result[0][0..8], "##.#.#..");
    assert_eq!(&result[1][0..8], ".#.#.#.#");
    assert_eq!(&result[2][0..8], "....#.#.");
    assert_eq!(&result[3][0..8], "#.#.##.#");
    assert_eq!(&result[4][0..8], ".##.#...");
    assert_eq!(&result[5][0..8], "##..#..#");
    assert_eq!(&result[6][0..8], ".#...#..");
    assert_eq!(&result[7][0..8], "##.#.##.");
}

#[test]
fn test_part2() {
    assert_eq!(part2("flqrgnkx"), 1242);
}
