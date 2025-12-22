// day 9 solution

struct Point {
    x: i64,
    y: i64
}

fn get_area(p1: &Point, p2: &Point) -> Option<i64> {
    // calculate the area of a rectangle made up of two opposing red points
    // check points can be opposing
    if p1.x == p2.x && p1.y == p2.y {
        return None;
    }
    let length = (p1.x - p2.x).abs() + 1;
    let breadth = (p1.y - p2.y).abs() + 1;

    Some(length * breadth)
}

#[allow(dead_code)]
pub fn part_one(input: &str) -> u64 {
    println!("Solving part 1!");

    // parse input
    let lines: Vec<&str> = input.lines().collect::<Vec<&str>>();

    let points = lines
        .iter()
        .map(|x| {
            x.split(",")
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>()
        .iter()
        .map(|p| Point{x: p[0], y: p[1]})
        .collect::<Vec<Point>>();

    let mut area = 0;

    for i in 0..points.len()-2 {
        for j in 0..points.len()-1 {
            let area_option = get_area(&points[i], &points[j]);

            match area_option {
                None => continue,
                Some(new_area) => if new_area > area {area = new_area},
            }

        }
    }
    

    area as u64
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
        let area12 = get_area(&p1, &p2);

        let p3 = Point{x: 3, y: 4};
        let p4 = Point{x: 1, y: 2};
        let area34 = get_area(&p3, &p4);

        assert_eq!(area12, Some(30));
        assert_eq!(area34, Some(9));
    }

    #[test]
    fn test_get_invalid_area() {
        let p1 = Point{x: 0, y: 0};
        let p2 = Point{x: 0, y: 4};
        let area12 = get_area(&p1, &p2);
        
        
        let p3 = Point{x: 5, y: 5};
        let p4 = Point{x: 5, y: 5};
        let area34 = get_area(&p3, &p4);

        let p5 = Point{x: 2, y: 2};
        let p6 = Point{x: 2, y: 2};
        let area56 = get_area(&p5, &p6);

        assert_eq!(area12, Some(5));
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