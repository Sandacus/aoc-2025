// day 6 solution

use std::collections::HashMap;

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
    #[ignore]
    fn part_two_example_input() {
        let input = fs::read_to_string("data/examples/06.txt")
        .expect("Should have been able to read the file");
        assert_eq!(part_two(&input), 0);
    }


}