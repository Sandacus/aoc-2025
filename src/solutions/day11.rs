// day 11 solution

use std::collections::HashMap;

#[allow(dead_code)]
pub fn part_one(input: &str) -> u64 {
    println!("Solving part 1!");

    let lines: Vec<&str> = input.lines().collect();

    let map = get_device_outputs(&lines);


   count_outs("you", &map, 0)
}

fn get_device_outputs<'a>(lines: &[&'a str]) -> HashMap<&'a str, Vec<&'a str>> {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in lines {
        let k = line.split(":").next().unwrap();
        let v = line.split(" ").skip(1).collect();
        map.insert(k, v);
    }

    map
}

fn count_outs(key: &str, map: &HashMap<&str, Vec<&str>>, mut count: u64) -> u64 {
    let out = vec!["out"];
    
    if let Some(v) = map.get(key) {
        if v == &out {
            // todo
            return count + 1;
        } else {
            //todo
            // count += count_outs(v.iter().next().unwrap(), map, count);
            for new_key in v.iter() {
                count = count_outs(new_key, map, count);
            }
        }
    }

    count
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

    const EXAMPLE_FILE: &str = "data/examples/11.txt";
    const INPUT_FILE: &str = "data/inputs/11.txt";

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
        assert_eq!(part_one(&input), 5);
    }

    #[test]
    fn test_parse_input() {
        let input: String = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff".to_string();
        let lines: Vec<&str> = input.lines().collect();
        let map = get_device_outputs(&lines);

        let mut expected_map = HashMap::new();
        expected_map.insert("aaa", vec!["you", "hhh"]);
        expected_map.insert("you", vec!["bbb", "ccc"]);
        expected_map.insert("bbb", vec!["ddd", "eee"]);
        expected_map.insert("ccc", vec!["ddd", "eee", "fff"]);

        assert_eq!(map, expected_map);
    }

    #[test]
    fn test_out_count() {
        let mut map = HashMap::new();
        map.insert("aaa", vec!["out"]);
        map.insert("you", vec!["bbb", "ccc"]);
        map.insert("bbb", vec!["ddd", "aaa"]);
        map.insert("ccc", vec!["fff"]);
        map.insert("ddd", vec!["aaa", "eee", "fff"]);
        map.insert("eee", vec!["out"]);
        map.insert("fff", vec!["out"]);

        let out_count = count_outs("you", &map, 0);
        let expected_out_count: u64 = 5;

        assert_eq!(out_count, expected_out_count);
    }

    #[test]
    #[ignore]
    fn part_two_example_input() {
        let input = fs::read_to_string(EXAMPLE_FILE)
        .expect("Should have been able to read the file");
        assert_eq!(part_two(&input), 24);
    }
}