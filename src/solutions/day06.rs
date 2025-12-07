// day 6 solution

use std::collections::HashMap;

#[allow(dead_code)]
pub fn part_one(input: &str) -> u64 {
    println!("Solving part 1!");

    // parse input
    let lines = input.lines().collect::<Vec<&str>>();

    // create hashmap of vectors with numbers and math operator
    let mut maths: HashMap<usize, Vec<&str>> = HashMap::new();

    for line in lines {
        for (idx, item) in line.split(" ").filter(|x| !x.is_empty()).enumerate() {
            if let Some(vec) = maths.get_mut(&idx) {
                vec.push(item);
            } else {
                let vec = vec![item];
                maths.insert(idx, vec);
            }
        }
    }

    // take a peek at the hashamp
    // println!("{:?}", maths);

    // loop through hashmap and count
    let mut total = 0;
    for (_, v) in maths {
        if v[v.len()-1] == "*" {
            // get product
            let prod = v.iter().filter_map(|x| x.parse::<u64>().ok());
            total += prod.product::<u64>();
        } else if v[v.len()-1] == "+" {
            // get sum
            let sum = v.iter().filter_map(|x| x.parse::<u64>().ok());
            total += sum.sum::<u64>();
        }
    }


    total
}

#[allow(dead_code)]
pub fn part_two(input: &str) -> u64 {
    println!("Solving part 2!");

    // parse input
    let input_lines: Vec<&str> = input.lines().collect();

    // get a vector of tuples containing 
    // (operator, whitespace_between_operators)
    let ops_and_spaces: Vec<char> = input_lines[input_lines.len() -1]
        .chars()
        .collect();

    let mut config_vec: Vec<(char, u64)> = Vec::new();
    let mut op_counter = 0;
    for ch in ops_and_spaces {
         if ch == ' ' {
            // let last_idx = config_vec.len() -1;
            config_vec[op_counter-1].1 += 1;
         } else {
            config_vec.push((ch, 0));
            op_counter += 1;
         }
    }
    // add one space for the last element
    let last_idx = config_vec.len() -1;
    config_vec[last_idx].1 += 1;
    // println!("{:?}", config_vec);

    // so this will let us preserve white space for creating 
    // vector of strings held by the hashmap
    // whitespace needed for correct parsing of vector into numbers
    let mut maths: HashMap<usize, Vec<&str>> = HashMap::new();
    let number_lines = &input_lines[..(input_lines.len()-1)];
    for line in number_lines {
        let mut char_count: u64 = 0;
        for (idx, (_, num)) in config_vec.iter().enumerate() {
            // use number to define the string slice to take
            if let Some(vec) = maths.get_mut(&idx) {
                let start_idx: usize = char_count as usize;
                let end_idx = (char_count + num) as usize;
                // println!("start: {}, end: {}", start_idx, end_idx);
                let substring = &line[start_idx..end_idx];
                vec.push(substring);
                char_count += num + 1;
            } else {
                let start_idx = char_count as usize;
                let end_idx = (char_count + num) as usize;
                // println!("start: {}, end: {}", start_idx, end_idx);
                let substring = &line[start_idx..end_idx];
                let vec = vec![substring];
                maths.insert(idx, vec);
                char_count += num + 1;
            }
        }
    }

    // take a peek at our numbers with correct spaces
    // println!("{:?}\n", maths.get(&0));

    // lets reverse order of the number vectors for easier parsing
    let mut reverse_maths: HashMap<usize, Vec<String>> = HashMap::new();
    for (i, v) in maths {
        reverse_maths.insert(i, Vec::new());
        for el in v {
            let reversed = el.chars().rev().collect::<String>();
            let vec = reverse_maths.get_mut(&i).unwrap();
            vec.push(reversed);
        }
    }

    // take a peek at our numbers with correct spaces
    // println!("{:?}\n", reverse_maths.get(&0));

    // loop through config vec and number vectors
    let mut count: u64 = 0;
    for (idx,(ch, _) ) in config_vec.iter().enumerate() {
        // get rearranged numbers
        let nums = convert_numbers(&reverse_maths[&idx]);
        if ch == &'*' {
            // product 
            count += nums.iter().product::<u64>();
        } else if ch == &'+' {
            // sum
            count += nums.iter().sum::<u64>();
        }
    }
    
   count
}

#[allow(dead_code)]
fn convert_numbers(input: &Vec<String>) -> Vec<u64> {
    // println!("Vec: {:?}", input);
    let num_digits = input[0].len();
    // create new vec to hold converted numbers
    let mut nums: Vec<u64> = Vec::new();
    for i in 0..num_digits {
        let mut new_number: String = String::new();
        for j in input {
            let chr = String::from(&j[i..i+1]);
            new_number.push_str(&chr);
        }
        let err_message = format!("\nPray this --{}-- is a number\n", new_number);
        nums.push(new_number.trim().parse().expect(&err_message));
    }

    nums
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
    fn part_two_example_input() {
        let input = fs::read_to_string("data/examples/06.txt")
        .expect("Should have been able to read the file");
        assert_eq!(part_two(&input), 3263827);
    }

    #[test]
    fn test_convert_numbers() {
        let input = vec![String::from("321"), String::from("54 "), String::from("6  ")];
        assert_eq!(convert_numbers(&input), vec![356, 24, 1]);
    }


}