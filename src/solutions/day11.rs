// day 11 solution

#[allow(dead_code)]
pub fn part_one(_input: &str) -> u64 {
    println!("Solving part 1!");

   42
}


#[allow(dead_code)]
pub fn part_two(_input: &str) -> u64 {
    println!("Solving part 2!");

    42
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const EXAMPLE_FILE: &str = "data/examples/11.txt";
    const INPUT_FILE: &str = "data/inputs/11.txt";

    #[test]
    fn read_input() {
        assert!(fs::exists(INPUT_FILE).expect("couldn't find input"));
    }

    #[test]
    fn read_example() {
        assert!(fs::exists(EXAMPLE_FILE).expect("couldn't find example"));
    }

    #[test]
    fn part_one_example_input() {
        let input = fs::read_to_string(EXAMPLE_FILE)
        .expect("Should have been able to read the file");
        assert_eq!(part_one(&input), 5);
    }

    #[test]
    #[ignore]
    fn part_two_example_input() {
        let input = fs::read_to_string(EXAMPLE_FILE)
        .expect("Should have been able to read the file");
        assert_eq!(part_two(&input), 24);
    }
}