// day 1 solution

#[allow(dead_code)]
pub fn part_one(input: &str) -> (u32, u32) {
    println!("Solving part 1!");

    // parse string into multiple instructions
    // vector of &str[]
    let directions: Vec<&str> = input.lines().map(|x| x.trim()).collect();
    // println!("{:?}", directions);

    let mut position = 50;
    let mut count = 0;

    // loop over directions
    for dir in directions.iter() {
        if dir.starts_with("R") {
            let clicks = dir.strip_prefix("R").expect("was expecting R").parse::<u32>().unwrap();
            position = (clicks + position) % 100;
        } else if dir.starts_with("L") {
            let clicks = dir.strip_prefix("L").expect("was expecting L").parse::<u32>().unwrap();
            position = (100 - clicks%100 + position) % 100;
        }
        if position == 0 {count+=1}
    }
    
    (count, position)
}

#[allow(dead_code)]
pub fn part_two(input: &str) -> (u32, u32) {
    println!("Solving part 2!");

    let directions: Vec<&str> = input.lines().map(|x| x.trim()).collect();

    let mut position = 50;
    let mut count = 0;

    // loop over directions
    for dir in directions.iter() {
        if dir.starts_with("R") {
            let clicks = dir.strip_prefix("R").expect("was expecting R").parse::<u32>().unwrap();
            if clicks >= (100 - position) && position != 0 {
                count += (clicks - (100-position)).div_ceil(100);
            } else if position == 0 {
                count += clicks.div_ceil(100);
            }
            position = (clicks + position) % 100;

        } else if dir.starts_with("L") {
            let clicks = dir.strip_prefix("L").expect("was expecting L").parse::<u32>().unwrap();
            if clicks >= position && position != 0 {
                count += (clicks-position).div_ceil(100);
            } else if position == 0 {
                count += clicks.div_ceil(100);
            }
            position = (100 - clicks%100 + position) % 100;
        }
    }
    
    (count, position)
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn read_example() {
        // let path: &Path = Path::new("data/example/01.txt");
        assert!(fs::exists("data/examples/01.txt").expect("couldn't find example"));
    }

    #[test]
    fn part_two_example_input() {
        let input = fs::read_to_string("data/examples/01.txt")
        .expect("Should have been able to read the file");
        assert_eq!(part_two(&input), (6, 32));
    }

    #[test]
    fn part_two_turn_left() {
        // increase the number based on input with prefix R
        assert_eq!(part_two("L68\nL30"), (1, 52));
    }

     #[test]
    fn part_two_turn_left_multiple() {
        // increase the number based on input with prefix R
        assert_eq!(part_two("L551"), (6, 99));
    }

    #[test]
    fn part_two_turn_right_multiple() {
        // increase the number based on input with prefix R
        assert_eq!(part_two("R149"), (1, 99));
    }

    #[test]
    fn part_one_example_input() {
        let input = fs::read_to_string("]data/examples/01.txt")
        .expect("Should have been able to read the file");
        assert_eq!(part_one(&input), (3, 32));
    }

    #[test]
    fn turn_right() {
        // increase the number based on input with prefix R
        assert_eq!(part_one("R1"), (0, 51));
    }

    #[test]
    fn turn_right_twice() {
        // increase the number based on input with prefix R
        let input = "R1\nR1";
        assert_eq!(part_one(input), (0, 52));
    }

    #[test]
    fn turn_left() {
        // increase the number based on input with prefix R
        assert_eq!(part_one("L50"), (1, 0));
    }

    #[test]
    fn turn_left_twice() {
        let input = "L1\nL1";
        assert_eq!(part_one(input), (0, 48));
    }

    #[test]
    fn turn_left_then_right() {
        let input = "L1\nR1";
        assert_eq!(part_one(input), (0,50));
    }

    #[test]
    fn turn_right_past_zero() {
        // increase the number based on input with prefix R
        assert_eq!(part_one("R51"), (0,1));
    }

    #[test]
    fn turn_left_past_zero() {
        // increase the number based on input with prefix R
        assert_eq!(part_one("L51"), (0,99));
    }
}