use std::fs;
use crate::solutions::*;

mod solutions;

fn main() {
    let input = fs::read_to_string("data/inputs/08.txt")
        .expect("Should have been able to read the file");
    
    let part1 = day08::part_one(&input);
    println!("part 1 ans: {}", part1);

    let part2 = day08::part_two(&input);
    println!("part 2 ans: {}", part2);
}
