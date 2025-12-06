// day 5 solution

use std::collections::VecDeque;

#[allow(dead_code)]
pub fn part_one(input: &str) -> u64 {
    println!("Solving part 1!");

    // parse input
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

#[allow(dead_code)]
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

    // sort the id ranges in order of lowest to highest range end value
    for i in 0..fresh_id_rngs.len() {
        for j in i..fresh_id_rngs.len() {
            if fresh_id_rngs[i].1 > fresh_id_rngs[j].1 
            || (fresh_id_rngs[i].1 == fresh_id_rngs[j].1 && fresh_id_rngs[i].0 > fresh_id_rngs[j].1) {
                fresh_id_rngs.swap(i, j);
            } 
        }
    }

    // find highest end id value for use as seed
    let seed_rng: (u64, u64) = fresh_id_rngs[fresh_id_rngs.len()-1];
    // for (start, end) in &fresh_id_rngs {
    //     if end > &seed_rng.1 {
    //         seed_rng = (*start, *end);
    //     }
    // }

    // initialise fresh_id vector
    // seed with the highest ending range from ranges input
    let mut fresh_ids: VecDeque<(u64, u64)> = VecDeque::from([seed_rng]);

    for (try_start, try_end) in &fresh_id_rngs {
        // is the rng a subset of a range already in the fresh_ids vector
        // loop through fresh_ids
        for idx in 0..fresh_ids.len() {
            // start and end of range
            // println!("try range: ({:?}, {:?})", try_start, try_end);
            let f_st = fresh_ids[idx].0;
            let f_end = fresh_ids[idx].1;
            // println!("fresh range: ({:?}, {:?})", f_st, f_end);

            // range before
            if try_end < &f_st {
                // add new range before current range
                let insert_point = if idx == 0 {0} else {idx};
                fresh_ids.insert(insert_point, (*try_start, *try_end));
                break;
            }

            // range start overlaps
            if try_start <= &f_st && try_end >= &f_st && try_end <= &f_end {
                // update current fresh range
                fresh_ids[idx].0 = *try_start;
                break;
            }

            // range end overlaps
            if try_start >= &f_st && try_start <= &f_end && try_end >= &f_end {
                // update current fresh range
                fresh_ids[idx].1 = *try_end;
                break;
            }

        }
    }

    // if seed value highest value is equal to the next highest end range 
    // remove seed
    if seed_rng.1 == fresh_ids[fresh_ids.len()-2].1 {
        fresh_ids.pop_back();
    }

    // loop though to check for overlaps
    for i in 0..fresh_ids.len()-1 {
        for j in i+1..fresh_ids.len() {
            // if the end value of current index >= start value of next index
            if fresh_ids[i].1 >= fresh_ids[j].0 {
                fresh_ids[j].0 = fresh_ids[i].0;
                fresh_ids.pop_front();
                break;
            } 
        }
    }

    println!("{:?}", fresh_ids);

    // sum the ranges we have 
    let mut fresh_id_count = 0;
    for (st, end) in fresh_ids {
        println!("Counting ==> ({}, {})", st,end);
        fresh_id_count += (end-st) + 1;
    }

    
    fresh_id_count
}
// 357429003954601
// 382855926026596
// 334949527076567 too high
// 172249655347656
// 316191919545416

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
    fn part_two_example_input() {
        let input = fs::read_to_string("data/examples/05.txt")
        .expect("Should have been able to read the file");
        assert_eq!(part_two(&input), 14);
    }


}