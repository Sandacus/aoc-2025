// day 9 solution

use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i64,
    y: i64
}

#[derive(Debug, PartialEq)]
struct Range {
    min: i64,
    max: i64
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

fn parse_input(input: &str) -> Vec<Point> {
    // parse input
    let lines: Vec<&str> = input.lines().collect::<Vec<&str>>();

    lines
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
        .collect::<Vec<Point>>()
}

#[allow(dead_code)]
pub fn part_one(input: &str) -> u64 {
    println!("Solving part 1!");

    // parse input
    let points = parse_input(input);

    let mut area = 0;

    for i in 0..points.len()-2 {
        for j in i+1..points.len()-1 {
            let area_option = get_area(&points[i], &points[j]);

            match area_option {
                None => continue,
                Some(new_area) => if new_area > area {area = new_area},
            }

        }
    }
    

    area as u64
}


fn get_valid_x_ranges(points: &[Point]) -> HashMap<i64, Range> {
    // get all points on a set y: HashMap{y, [x1, x2, ... xn]}
    let mut x_values: HashMap<i64, Vec<i64>> = HashMap::new();
    for point in points {
        // check if in x_ranges
        match x_values.get_mut(&point.y) {
            Some(x_rng) => {x_rng.push(point.x);},
            None => {x_values.insert(point.y, vec![point.x]);},
        }
    }
    // get the lowest and highest x value
    let mut x_ranges: HashMap<i64, Range> = HashMap::new();
    for (k, v) in &x_values {
        let (min, max): (i64, i64) = (*v.iter().min().unwrap(), *v.iter().max().unwrap());
        x_ranges.insert(*k, Range{min, max});
    }

    x_ranges
}

fn get_valid_y_ranges(points: &[Point]) -> HashMap<i64, Range> {
    // get all points on a set y: HashMap{y, [x1, x2, ... xn]}
    let mut y_values: HashMap<i64, Vec<i64>> = HashMap::new();
    for point in points {
        // check if in x_ranges
        match y_values.get_mut(&point.x) {
            Some(x_rng) => {x_rng.push(point.y);},
            None => {y_values.insert(point.x, vec![point.y]);},
        }
    }
    // get the lowest and highest x value
    let mut y_ranges: HashMap<i64, Range> = HashMap::new();
    for (k, v) in &y_values {
        let (min, max): (i64, i64) = (*v.iter().min().unwrap(), *v.iter().max().unwrap());
        y_ranges.insert(*k, Range{min, max});
    }

    y_ranges
}

fn check_rectangle_valid(p1: &Point, p2: &Point, valid_points: &HashMap<i64, Range>) -> bool {
    // construct array of opposite corners
    let (c1, c2) = (Point{ x: p1.x, y: p2.y }, Point{ x: p2.x, y: p1.y });

    // check c1
    let v1 = match valid_points.get(&c1.y) {                                        
        None => false,                                           // -> is y in the keys for x_range?
        Some(rng) => c1.x >= rng.min && c1.x <= rng.max, // -> is x_min <= x <= x_max?
    };
    // check c2
    let v2 = match valid_points.get(&c2.y) {                                        
        None => false,                                           // -> is y in the keys for x_range?
        Some(rng) => c2.x >= rng.min && c2.x <= rng.max, // -> is x_min <= x <= x_max?
    };
    
    v1 && v2
}

fn get_valid_map(x_ranges: &HashMap<i64, Range>, y_ranges: &HashMap<i64, Range>, points: &[Point]) -> HashMap<i64, Range>{
    // use the x_ranges and y_ranges to find all tiles that are valid and add to the points vector
    // e.g., for y=1: x_min= 1, x_max=5 ==> add points (2,1), (3,1), (4,1) to the points vector
    
    let mut allowed_points: Vec<Point> = points.to_vec();
    // add points from x_ranges
    for (y, x_rng) in x_ranges {
        for x in x_rng.min+1..x_rng.max {
            allowed_points.push(Point{x, y:*y});
        }
    }
    
    // add points from y_ranges
    for (x, y_rng) in y_ranges {
        for y in y_rng.min+1..y_rng.max {
            allowed_points.push(Point{x:*x, y});
        }
    }

    // now, recalculate valid ranges from allowed points
    get_valid_x_ranges(&allowed_points)
}


#[allow(dead_code)]
pub fn part_two(input: &str) -> u64 {
    println!("Solving part 2!");

    // parse input
    let points = parse_input(input);

    // create valid x ranges
    let x_ranges = get_valid_x_ranges(&points);
    // for (k, v) in &x_ranges {
    //     println!("for y: {}, the valid x: {:?}", k, v);
    // }

    // create valid y ranges
    let y_ranges = get_valid_y_ranges(&points);
    // for (k, v) in &y_ranges {
    //     println!("for x: {}, the valid y: {:?}", k, v);
    // }

    // now, recalculate valid ranges from allowed points
    let valid_map = get_valid_map(&x_ranges, &y_ranges, &points);

    // check pairs of points
    let mut area: i64 = 0;
    for i in 0..points.len()-2 {
        for j in i+1..points.len()-1 {
            let (p1, p2) = (&points[i], &points[j]);

            if check_rectangle_valid(p1, p2, &valid_map) 
            && let Some(a) = get_area(p1, p2)  
            && a > area {
                area = a;
            }
            
        }
    }

    
    area as u64
}
//        340 - too low
//     689788 - too low
// 4771508457 - too high
// 4620005060 - incorrect

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
    fn part_two_example_input() {
        let input = fs::read_to_string(EXAMPLE_FILE)
        .expect("Should have been able to read the file");
        assert_eq!(part_two(&input), 24);
    }

    #[test]
    fn test_get_x_min_max() {
        let p1 = Point{x: 0, y: 0};
        let p2 = Point{x: 8, y: 0};    
        let p3 = Point{x: 5, y: 0};
        let p4 = Point{x: 1, y: 3};
        let p5 = Point{x: 23, y: 3};
        let p6 = Point{x: 9, y: 3};

        let points = vec![p1, p2, p3, p4, p5, p6];

        let x_rng = get_valid_x_ranges(&points); 

        // min, max @ y=0
        assert_eq!(x_rng.get(&0), Some(&Range{min: 0, max: 8}));
        assert_eq!(x_rng.get(&3), Some(&Range{min: 1, max: 23}));
    }

    #[test]
    fn test_get_y_min_max() {
        let p1 = Point{x: 0, y: 3};
        let p2 = Point{x: 0, y: 1};    
        let p3 = Point{x: 0, y: 43};
        let p4 = Point{x: 6, y: 21};
        let p5 = Point{x: 6, y: 33};
        let p6 = Point{x: 6, y: 4};

        let points = vec![p1, p2, p3, p4, p5, p6];

        let y_rng = get_valid_y_ranges(&points); 

        // min, max @ y=0
        assert_eq!(y_rng.get(&0), Some(&Range{min: 1, max: 43}));
        assert_eq!(y_rng.get(&6), Some(&Range{min: 4, max: 33}));
    }

    #[test]
    fn test_check_point_is_valid() {
        // valid map based on the example
        let mut valid_map: HashMap<i64, Range> = HashMap::new();
        valid_map.insert(1, Range { min: 7, max:11 });
        valid_map.insert(2, Range { min: 7, max: 11 });
        valid_map.insert(3, Range { min: 2, max: 11 });
        valid_map.insert(4, Range { min: 2, max: 11 });
        valid_map.insert(5, Range { min: 2, max: 11 });
        valid_map.insert(6, Range { min: 9, max: 11 });
        valid_map.insert(7, Range { min: 9, max: 11 });

        // valid pair (p1, p2)
        let p1 = Point{ x: 9, y: 5 };
        let p2 = Point{ x: 2, y: 3 };
        assert!(check_rectangle_valid(&p1, &p2, &valid_map));
        // invalid pair (p1, p3)
        let p3 = Point{ x: 7, y: 1};
        assert!(!check_rectangle_valid(&p2, &p3, &valid_map))
    }

    #[test]
    fn test_making_valid_map() {
        let p1 = Point{x: 7, y: 1};
        let p2 = Point{x: 11, y: 1};    
        let p3 = Point{x: 11, y: 7};
        let p4 = Point{x: 9, y: 7};
        let p5 = Point{x: 9, y: 5};
        let p6 = Point{x: 2, y: 5};
        let p7 = Point{x: 2, y: 3};
        let p8 = Point{x: 7, y: 3};

        let points = vec![p1, p2, p3, p4, p5, p6, p7, p8];

        let x_rng = get_valid_x_ranges(&points); 
        let y_rng = get_valid_y_ranges(&points);

        let test_valid_map = get_valid_map(&x_rng, &y_rng, &points);

        let mut valid_map: HashMap<i64, Range> = HashMap::new();
        valid_map.insert(1, Range { min: 7, max:11 });
        valid_map.insert(2, Range { min: 7, max: 11 });
        valid_map.insert(3, Range { min: 2, max: 11 });
        valid_map.insert(4, Range { min: 2, max: 11 });
        valid_map.insert(5, Range { min: 2, max: 11 });
        valid_map.insert(6, Range { min: 9, max: 11 });
        valid_map.insert(7, Range { min: 9, max: 11 });

        assert_eq!(test_valid_map, valid_map);

        
    }

}