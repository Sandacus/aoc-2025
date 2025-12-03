// day 3 solution

pub fn part_one(input: &str) -> u64 {
    println!("Solving part 1!");
    
    // parse input to get battery banks
    let banks = input.lines().collect::<Vec<&str>>();

    // find joltage for each bank
    let mut total_joltage: u64 = 0;
    for bank in banks {
        total_joltage += find_2_battery_joltage(bank);
    }

    total_joltage
}

fn find_2_battery_joltage(input: &str) -> u64 {
    // Create battery variables
    let mut b1: u8 = 0;
    let mut b2: u8 = 0;

    // turn number into digits
    let digits = input
        .chars()
        .map(|x| 
            x as u8
        )
        .collect::<Vec<u8>>();
    
    // one pass of digits comparing curent index and index+1
    // if current index greater than b1 then update 
    // ==> b1 = current index
    // ==> b2 current index + 1
    for idx in 1..digits.len() {
        if digits[idx-1] > b1 {
            b1 = digits[idx-1];
            b2 = digits[idx];
        } else if digits[idx] > b2 {
            b2 = digits[idx]
        }
    }

    // convert b1 and b2 back to string to get number
    let b1_char = b1 as char;
    let b2_char = b2 as char;
    let num = [b1_char, b2_char].iter().collect::<String>();

    num.parse::<u64>().unwrap()
}


pub fn part_two(input: &str) -> u64 {
    println!("Solving part 2!");

    // parse input to get battery banks
    let banks = input.lines().collect::<Vec<&str>>();

    // find joltage for each bank
    let mut total_joltage: u64 = 0;
    for bank in banks {
        total_joltage += find_12_battery_joltage(bank);
    }

    total_joltage
}

fn find_12_battery_joltage(input: &str) -> u64 {
    // Create battery variables as array
    let mut batteries = [0; 12];

    // turn number into digits
    let digits = input
        .chars()
        .map(|x| 
            x as u8
        )
        .collect::<Vec<u8>>();
    
    // one pass of digits comparing curent index and index+1... index + 11
    // if current index greater than b1 then update all b's
    // ==> b1 = current index
    // ==> b2 current index + 1
    // ...
    // ==> b12 current index + 11
    for idx in 0..=digits.len()-12 {

        // for loop over batterries array
        for b_idx in 0..batteries.len() {
            if digits[idx + b_idx] > batteries[b_idx] {
                // update batteries vector
                batteries[b_idx..].iter_mut()
                    .enumerate()
                    .for_each(|(i, b)| *b = digits[i+idx+b_idx]);
            }
        }
    }

    // convert batteries array to str then parse and return number
    let num = std::str::from_utf8(&batteries).expect("Not UTF-8");

    num.parse::<u64>().unwrap()
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
        assert_eq!(find_2_battery_joltage("987654321111111"), 98);
    }

    #[test]
    fn part_two_example_input() {
        let input = fs::read_to_string("data/examples/03.txt")
        .expect("Should have been able to read the file");
        assert_eq!(part_two(&input), 3121910778619);
    }

    #[test]
    fn test_find_12_battery_joltage() {
        assert_eq!(find_12_battery_joltage("234234234234278"), 434234234278);
    }
}