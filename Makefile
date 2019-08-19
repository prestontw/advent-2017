run=cargo run

day7-part1:
	cat input-problem1.txt | cargo run part1

day7-part2:
	cat input-problem1.txt | cargo run part2

day8:
	cat input.txt | ${run}