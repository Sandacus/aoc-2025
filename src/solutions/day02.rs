// day 2 solution

pub fn part_one(input: &str) -> u64 {
    println!("Solving part 1!");

    // split input on ','
    let id_rngs: Vec<&str> = input
        .split(",")
        .collect();

    // for each id_rng run the count fn
    let mut count: u64 = 0;
    for id_rng in id_rngs {
        count += count_doubles(id_rng);
    }
    
    count
}

fn count_doubles(rng: &str) -> u64 {
    let rng: Vec<&str> = rng.split("-").collect();
    let start = rng[0].parse::<u64>().unwrap();
    let end = rng[1].parse::<u64>().unwrap();

    let mut count: u64 = 0;
    for id in start..=end {
        let x = id.to_string().chars().count();
        
        if x % 2 != 0 {
            continue;
        } 

        let first = &id.to_string()[..x/2];
        let last = &id.to_string()[x/2..];
        
        if first == last {
            count += id;
        }
    }
    count
}

pub fn part_two(_input: &str) -> u32 {
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
    fn counts_doubles() {
        assert_eq!(count_doubles("95-115"), 99);
    }

    #[test]
    fn sums_from_input() {
        assert_eq!(part_one("11-22,95-115,998-1012"), 11 + 22 + 99 + 1010)
    }

    #[test]
    fn part_one_example_input() {
        let input = fs::read_to_string("data/examples/02.txt")
        .expect("Should have been able to read the file");
        assert_eq!(part_one(&input), 1227775554);
    }
}