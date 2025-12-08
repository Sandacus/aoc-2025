// day 8 solution

pub fn part_one(input: &str) -> u64 {
    println!("Solving part 1!");

    // parse input
    let lines = input.lines().collect::<Vec<&str>>();
    let mut arr = lines
        .iter()
        .map(|x| 
            x.chars().collect::<Vec<char>>()
        )
        .collect::<Vec<Vec<char>>>();

    42
}


pub fn part_two(input: &str) -> u64 {
    println!("Solving part 2!");

    // parse input
    let lines: Vec<&str> = input.lines().collect();
    let mut arr = lines
       .iter()
       .map(|x| 
           x.chars().collect::<Vec<char>>()
       )
       .collect::<Vec<Vec<char>>>();

    42
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const EXAMPLE_FILE: &str = "data/examples/08.txt";
    const INPUT_FILE: &str = "data/inputs/08.txt";

    #[test]
    fn read_input() {
        assert!(fs::exists(INPUT_FILE).expect("couldn't find input"));
    }

    #[test]
    fn read_example() {
        assert!(fs::exists(EXAMPLE_FILE).expect("couldn't find example"));
    }

    #[test]
    #[ignore]
    fn part_one_example_input() {
        let input = fs::read_to_string(EXAMPLE_FILE)
        .expect("Should have been able to read the file");
        assert_eq!(part_one(&input), 21);
    }

    #[test]
    #[ignore]
    fn part_two_example_input() {
        let input = fs::read_to_string(EXAMPLE_FILE)
        .expect("Should have been able to read the file");
        assert_eq!(part_two(&input), 40);
    }

}