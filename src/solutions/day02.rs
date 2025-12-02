// day 2 solution

pub fn part_one(input: &str) -> u32 {
    println!("Solving part 1!");

    
    42
}

pub fn part_two(input: &str) -> u32 {
    println!("Solving part 2!");

    // TODO

    42
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn read_example() {
        assert!(fs::exists("data/examples/02.txt").expect("couldn't find example"));
    }

    #[test]
    fn part_two_example_input() {
        let input = fs::read_to_string("data/examples/02.txt")
        .expect("Should have been able to read the file");
        assert_eq!(part_two(&input), 1227775554);
    }
}