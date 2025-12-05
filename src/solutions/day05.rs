// day 5 solution

pub fn part_one(input: &str) -> u64 {
    println!("Solving part 1!");

    // parse input
    // split into two strings on line break
    let lines = input.lines().collect::<Vec<&str>>();

    let fresh_ids_str: Vec<(&str, &str)> = lines
        .clone()
        .iter()
        .filter(|x| x.contains("-"))
        .map(|x| x.split_once("-").unwrap())
        .collect::<Vec<(&str, &str)>>();

    let fresh_ids: Vec<(u64, u64)> = fresh_ids_str
        .iter()
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect();

    let available_ids_str: Vec<&&str> = lines
        .iter()
        .filter(|x| !x.contains("-") && !x.is_empty())
        .collect::<Vec<&&str>>();

    let available_ids: Vec<u64> = available_ids_str
        .iter()
        .map(|x| x.parse().unwrap())
        .collect();
    
    // println!("{:?}", fresh_ids);
    // println!("{:?}", available_ids);

    let mut fresh_count = 0;
    for id in available_ids {
        for fresh_rng in &fresh_ids {
            if id >= fresh_rng.0 && id <= fresh_rng.1 {
                fresh_count += 1;
                break;
            }
        }
    }


    fresh_count
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
        assert!(fs::exists("data/inputs/05.txt").expect("couldn't find input"));
    }

    #[test]
    fn read_example() {
        assert!(fs::exists("data/examples/05.txt").expect("couldn't find example"));
    }

    #[test]
    fn part_one_example_input() {
        let input = fs::read_to_string("data/examples/05.txt")
        .expect("Should have been able to read the file");
        assert_eq!(part_one(&input), 3);
    }


    #[test]
    #[ignore]
    fn part_two_example_input() {
        let input = fs::read_to_string("data/examples/05.txt")
        .expect("Should have been able to read the file");
        assert_eq!(part_two(&input), 43);
    }


}