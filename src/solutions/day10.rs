// day 10 solution

#[allow(dead_code)]
pub fn part_one(input: &str) -> u64 {
    println!("Solving part 1!");

    // parse input
    let lines: Vec<&str> = input.lines().collect();

    // get diagrams

    // get wiring

    // get joltage


   42
}

fn get_diagrams(lines: Vec<&str>) -> Vec<Vec<char>> {
    lines
        .iter()
        .map(|x| 
            x.split_whitespace().next().unwrap().chars().filter(|x| *x != '[' && *x != ']').collect()
        )
        .collect::<Vec<Vec<char>>>()
}

fn get_wiring(lines: Vec<&str>) -> Vec<Vec<Vec<u64>>> {
    lines
        .iter()
        .map(|x| 
            x.split_whitespace()
                .filter(|y| y.contains("("))
                .map(|x| 
                    x.chars()
                        .filter_map(|c| c.to_digit(10).map(u64::from))
                        .collect::<Vec<u64>>()
                )
                .collect::<Vec<Vec<u64>>>()
        )
        .collect::<Vec<Vec<Vec<u64>>>>()
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

    const EXAMPLE_FILE: &str = "data/examples/10.txt";
    const INPUT_FILE: &str = "data/inputs/10.txt";

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
        assert_eq!(part_one(&input), 7);
    }

    #[test]
    fn test_parse_diagrams() {
        let lines = vec!["[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}", 
            "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}", 
            "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"];
        
        let diagrams: Vec<Vec<char>> = get_diagrams(lines);

        let expected_diagrams0 = vec!['.', '#', '#', '.'];
        let expected_diagrams1 = vec!['.', '.', '.', '#', '.'];
        let expected_diagrams2 = vec!['.', '#', '#', '#', '.', '#'];

        assert_eq!(diagrams[0], expected_diagrams0);
        assert_eq!(diagrams[1], expected_diagrams1);
        assert_eq!(diagrams[2], expected_diagrams2);
    }

    #[test]
    fn test_parse_wiring() {
        let lines = vec!["[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}", 
            "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}", 
            "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"];
        
        let wiring = get_wiring(lines);

        let expected_wiring0 = vec![vec![3], vec![1,3], vec![2], vec![2,3], vec![0,2], vec![0,1]];
        let expected_wiring1 = vec![vec![0,2,3,4], vec![2,3], vec![0,4], vec![0,1,2], vec![1,2,3,4]];
        let expected_wiring2 = vec![vec![0,1,2,3,4], vec![0,3,4], vec![0,1,2,4,5], vec![1,2]];

        assert_eq!(wiring[0], expected_wiring0);
        assert_eq!(wiring[1], expected_wiring1);
        assert_eq!(wiring[2], expected_wiring2);
    }

    #[test]
    #[ignore]
    fn part_two_example_input() {
        let input = fs::read_to_string(EXAMPLE_FILE)
        .expect("Should have been able to read the file");
        assert_eq!(part_two(&input), 24);
    }
}