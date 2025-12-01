use std::fs;

mod day01;

fn main() {
    let input = fs::read_to_string("data/inputs/01.txt")
        .expect("Should have been able to read the file");
    
    let (part1, _) = day01::part_one(&input);
    println!("part 1 ans: {}", part1);

    let (part2, _) = day01::part_two(&input);
    println!("part 2 ans: {}", part2);
}
