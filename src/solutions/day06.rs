// day 6 solution

use std::collections::VecDeque;

pub fn part_one(input: &str) -> u64 {
    println!("Solving part 1!");

    // parse input
    let lines = input.lines().collect::<Vec<&str>>();

    let available_ids_str: Vec<&&str> = lines
        .iter()
        .filter(|x| !x.contains("-") && !x.is_empty())
        .collect::<Vec<&&str>>();

    let available_ids: Vec<u64> = available_ids_str
        .iter()
        .map(|x| x.parse().unwrap())
        .collect();

    42
}


pub fn part_two(input: &str) -> u64 {
    println!("Solving part 2!");

    // parse input
    let lines = input.lines().collect::<Vec<&str>>();

    let mut fresh_id_rngs: Vec<(u64, u64)> = lines
        .clone()
        .iter()
        .filter(|x| x.contains("-"))
        .map(|x| x.split_once("-").unwrap())
        .collect::<Vec<(&str, &str)>>()
        .iter()
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect();

   42
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn read_input() {
        assert!(fs::exists("data/inputs/06.txt").expect("couldn't find input"));
    }

    #[test]
    fn read_example() {
        assert!(fs::exists("data/examples/06.txt").expect("couldn't find example"));
    }

    #[test]
    fn part_one_example_input() {
        let input = fs::read_to_string("data/examples/06.txt")
        .expect("Should have been able to read the file");
        assert_eq!(part_one(&input), 4277556);
    }


    #[test]
    #[ignore]
    fn part_two_example_input() {
        let input = fs::read_to_string("data/examples/06.txt")
        .expect("Should have been able to read the file");
        assert_eq!(part_two(&input), 0);
    }


}