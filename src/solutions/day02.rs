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

pub fn part_two(input: &str) -> u64 {
    println!("Solving part 2!");

    // TODO
    //
    // start with first digit
    // look through rest of digits 1 at a time
    // if all same then count
    //
    // try first 2 digits
    // look at the rest of the pairs
    // if made up of multiple pairs count
    // and repeat
    // 
    // repeat for n/2 digits
    // can stop comparing if the digits % search_digits != 0

    // split input on ','
    let id_rngs: Vec<&str> = input
        .split(",")
        .collect();

    // for each id_rng run the count fn
    let mut count: u64 = 0;
    for id_rng in id_rngs {
        count += count_repeats(id_rng);
    }
    
    count
}

fn count_repeats(rng: &str) -> u64 {
    let rng: Vec<&str> = rng.split("-").collect();
    let start = rng[0].parse::<u64>().unwrap();
    let end = rng[1].parse::<u64>().unwrap();

    let mut count: u64 = 0;
    for id in start..=end {
        let x = id.to_string().chars().count();

        // loop through potential repeating units
        for rpt in 1..=x/2 {
            // skip condition
            if x % rpt != 0 {
            continue;
            }

            let repeating = &id.to_string()[..rpt];
            let rest: Vec<char> = id.to_string()[rpt..].chars().collect();
            let rep_rest: Vec<String> = rest
                .chunks(rpt)
                .map(|x| x.iter().collect::<String>())
                .collect();

            let mut should_count = true;
            for x in rep_rest {
                if repeating != x {
                    should_count = false;
                    break;
                }
            }

            if should_count {
                count += id;
                break;
            }
        }
        
    }
    count
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
    #[ignore]
    fn can_count_doubles() {
        assert_eq!(count_doubles("95-115"), 99);
    }

    #[test]
    #[ignore]
    fn sums_from_input() {
        assert_eq!(part_one("11-22,95-115,998-1012"), 11 + 22 + 99 + 1010)
    }

    #[test]
    #[ignore]
    fn part_one_example_input() {
        let input = fs::read_to_string("data/examples/02.txt")
        .expect("Should have been able to read the file");
        assert_eq!(part_one(&input), 1227775554);
    }

    #[test]
    fn part_two_example_input() {
        let input = fs::read_to_string("data/examples/02.txt")
        .expect("Should have been able to read the file");
        assert_eq!(part_two(&input), 4174379265);
    }

    #[test]
    fn can_counts_repeats() {
        assert_eq!(count_repeats("1188511880-1188511890"), 1188511885);
    }
}