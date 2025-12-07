// day 7 solution

pub fn part_one(input: &str) -> u64 {
    println!("Solving part 1!");

    // parse input
    let lines = input.lines().collect::<Vec<&str>>();
    let arr = lines
        .iter()
        .map(|x| 
            x.chars().collect::<Vec<char>>()
        )
        .collect::<Vec<Vec<char>>>();

    // find start S
    // loop through lines of arr
    // beam down function that modifies the next line with where the beam will be
   
    // once looped through 
    // count number of converted spitters in arr
    
    42
}

fn beam_down() {
    // --> beam down updates 
    // ----> free space: '.' to '|' 
    // ----> splitter: '^' to '|','+','|'
}


pub fn part_two(input: &str) -> u64 {
    println!("Solving part 2!");

    // parse input
    let _input_lines: Vec<&str> = input.lines().collect();

    42
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const INPUT_FILE: &str = "data/inputs/07.txt";

    #[test]
    fn read_input() {
        assert!(fs::exists(INPUT_FILE).expect("couldn't find input"));
    }

    #[test]
    fn read_example() {
        assert!(fs::exists(INPUT_FILE).expect("couldn't find example"));
    }

    #[test]
    fn part_one_example_input() {
        let input = fs::read_to_string(INPUT_FILE)
        .expect("Should have been able to read the file");
        assert_eq!(part_one(&input), 21);
    }


    #[test]
    #[ignore]
    fn part_two_example_input() {
        let input = fs::read_to_string(INPUT_FILE)
        .expect("Should have been able to read the file");
        assert_eq!(part_two(&input), 0);
    }

}