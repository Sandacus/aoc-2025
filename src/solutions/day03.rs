// day 3 solution

pub fn part_one(input: &str) -> u64 {
    println!("Solving part 1!");
    
    // todo parse input to get battery banks
    42
}

fn find_joltage(input: &str) -> u64 {
    // Create 2 variables and loop through numbers
    // if number is higher than variable reassign
    let mut b1: u8 = 0;
    let mut b2: u8 = 0;

    // turn number into digits
    let digits = input
        .chars()
        .map(|x| 
            x as u8
        )
        .collect::<Vec<u8>>();

    for x in digits {
        if x > b1 {
            (b1, b2) = (x, b1);
        } else if x > b2 {
            b2 = x;
        }
    }

    // convert b1 and b2 back to string to get number
    let b1_char = b1 as char;
    let b2_char = b2 as char;
    let num = [b1_char, b2_char].iter().collect::<String>();

    num.parse::<u64>().unwrap()
}


pub fn part_two(_input: &str) -> u64 {
    println!("Solving part 2!");

    42
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn read_input() {
        assert!(fs::exists("data/inputs/03.txt").expect("couldn't find input"));
    }

    #[test]
    fn read_example() {
        assert!(fs::exists("data/examples/03.txt").expect("couldn't find example"));
    }

    #[test]
    fn part_one_example_input() {
        let input = fs::read_to_string("data/examples/03.txt")
        .expect("Should have been able to read the file");
        assert_eq!(part_one(&input), 357);
    }

    #[test]
    fn test_find_joltage() {
        assert_eq!(find_joltage("987654321111111"), 98);
    }

    #[test]
    #[ignore]
    fn part_two_example_input() {
        let input = fs::read_to_string("data/examples/03.txt")
        .expect("Should have been able to read the file");
        assert_eq!(part_two(&input), 4174379265);
    }
}