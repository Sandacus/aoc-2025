// day 5 solution

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

#[derive(Debug)]
#[derive(Clone, Copy)]
struct IdRange {
    start: u64,
    end: u64
}

fn filter_ranges(mut fresh_ids: Vec<IdRange>) -> (Vec<IdRange>, bool) {
    let mut overlaps_found = false;
    // filter out overlaps
    for i in 1..fresh_ids.len() {
        if fresh_ids[i].start <= fresh_ids[i-1].end
        && fresh_ids[i].end <= fresh_ids[i-1].end {
            overlaps_found = true;
            // replace current range i with range from i-1
            fresh_ids[i].start = fresh_ids[i-1].start;
            fresh_ids[i].end = fresh_ids[i-1].end;
            fresh_ids[i-1].start = 0;
            fresh_ids[i-1].end = 0;
        } else if fresh_ids[i].start <= fresh_ids[i-1].end
        && fresh_ids[i].end > fresh_ids[i-1].end {
            overlaps_found = true;
            fresh_ids[i].start = fresh_ids[i-1].start;
            fresh_ids[i-1].start = 0;
            fresh_ids[i-1].end = 0;
        }
    }

    // create new vector with out the zeros
    let mut freshies: Vec<IdRange> = Vec::new();
    for rng in fresh_ids {
        if rng.start != 0 && rng.end != 0 {
            freshies.push(rng);
        }
    }

    (freshies, overlaps_found)
}

#[allow(dead_code)]
pub fn part_two(input: &str) -> u64 {
    println!("Solving part 2!");

    // parse input
    let lines = input.lines().collect::<Vec<&str>>();

    let mut fresh_ids: Vec<IdRange> = lines
        .clone()
        .iter()
        .filter(|x| x.contains("-"))
        .map(|x| x.split_once("-").unwrap())
        .collect::<Vec<(&str, &str)>>()
        .iter()
        .map(|(x, y)| 
            IdRange{ start: x.parse().unwrap(), end: y.parse().unwrap()}
        )
        .collect();

    println!("unsorted");
    for rng in &fresh_ids {
        println!("{:?}", rng);
    }
    println!("unsorted\n");


    // sort the id ranges in order of lowest to highest range START value
    for i in 0..fresh_ids.len() {
        for j in i..fresh_ids.len() {
            if fresh_ids[i].start > fresh_ids[j].start {
                fresh_ids.swap(i, j);
            } 
        }
    }

    println!("sorted");
    for rng in &fresh_ids {
        println!("{:?}", rng);
    }
    println!("sorted\n");

    // filter out overlaps
    let mut overlaps_found = true;
    let mut freshies: Vec<IdRange> = fresh_ids.clone();
    while overlaps_found {
        (freshies, overlaps_found) = filter_ranges(freshies);
    }
    
    // check for overlaps
    for i in 1..freshies.len() {
        if freshies[i].start < freshies[i-1].end {
            panic!("overlap found at index {}\n{:?}\n{:?}\n", i, freshies[i-1], freshies[i]);
        }
    }

    // sum ranges
    let mut total: u64 = 0;
    for rng in freshies {
        println!("{:?}", rng);
        total += rng.end - rng.start + 1;
    }
    
    total
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
    fn part_two_example_input() {
        let input = fs::read_to_string("data/examples/05.txt")
        .expect("Should have been able to read the file");
        assert_eq!(part_two(&input), 14);
    }

    #[test]
    fn part_two_example2_input() {
        let input = fs::read_to_string("data/examples/05 copy.txt")
        .expect("Should have been able to read the file");
        assert_eq!(part_two(&input), 24);
    }


}