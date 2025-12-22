// day 9 solution

struct Point {
    x: i32,
    y: i32
}

fn get_area(p1: Point, p2: Point) -> Option<i32> {
    // calculate the area of a rectangle made up of two opposing red points
    // check points can be opposing
    if p1.x == p2.x || p1.y == p2.y {
        return None;
    }

    let area = (p1.x - p2.x).abs() * (p1.y - p2.y).abs();

    Some(area)
}

#[allow(dead_code)]
pub fn part_one(_input: &str) -> u64 {
    println!("Solving part 1!");

    // process input


    

    42
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

    const EXAMPLE_FILE: &str = "data/examples/09.txt";
    const INPUT_FILE: &str = "data/inputs/09.txt";

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
        assert_eq!(part_one(&input), 50);
    }

    #[test]
    fn test_get_valid_area() {
        let p1 = Point{x: 0, y: 0};
        let p2 = Point{x: 5, y: 4};
        let area12 = get_area(p1, p2);

        let p3 = Point{x: 3, y: 4};
        let p4 = Point{x: 1, y: 2};
        let area34 = get_area(p3, p4);

        assert_eq!(area12, Some(20));
        assert_eq!(area34, Some(4));
    }

    #[test]
    fn test_get_invalid_area() {
        let p1 = Point{x: 0, y: 0};
        let p2 = Point{x: 0, y: 4};
        let area12 = get_area(p1, p2);
        
        
        let p3 = Point{x: 0, y: 0};
        let p4 = Point{x: 5, y: 0};
        let area34 = get_area(p3, p4);

        let p5 = Point{x: 2, y: 2};
        let p6 = Point{x: 2, y: 2};
        let area56 = get_area(p5, p6);

        assert_eq!(area12, None);
        assert_eq!(area34, None);
        assert_eq!(area56, None);
    }

    #[test]
    #[ignore]
    fn part_two_example_input() {
        let input = fs::read_to_string(EXAMPLE_FILE)
        .expect("Should have been able to read the file");
        assert_eq!(part_two(&input), 40);
    }

}