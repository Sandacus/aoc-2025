// day 8 solution

const NUMBER_OF_PAIRS: usize = 1000;

// point struct
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
struct Point {
    x: f32,
    y: f32,
    z: f32
}

#[allow(dead_code)]
#[derive(Clone)]
#[derive(Debug)]
struct Connection {
    id: (usize, usize),
    p1: Point,
    p2: Point,
    dist: f32
}

#[allow(dead_code)]
impl Connection {
    fn new(box1: (Point, usize), box2: (Point, usize)) -> Self {
        let id: (usize, usize) = (box1.1, box2.1);
        let (p1, p2) = (box1.0, box2.0);
        let dist = calcualte_distance(&p1, &p2);
        Self { id, p1, p2, dist }
    }
}


// calculate distance between two points
#[allow(dead_code)]
fn calcualte_distance(p1: &Point, p2: &Point) -> f32 {
    let dx = (p1.x - p2.x) * (p1.x - p2.x);
    let dy = (p1.y - p2.y) * (p1.y - p2.y);
    let dz = (p1.z - p2.z) * (p1.z - p2.z);

    (dx + dy + dz).sqrt()
}

#[allow(dead_code)]
fn add_connections(mut conns: Vec<Connection>, con: Connection) -> Vec<Connection> {
    if conns.len() < NUMBER_OF_PAIRS {
        conns.push(con.clone());
    } else {
        // find longest distance
        let mut idx_longest = 0;
        for (i, c) in conns.iter().enumerate().skip(1) {
            if c.dist > conns[idx_longest].dist {
                idx_longest = i;
            }
        }

        // update conns vector
        if con.dist < conns[idx_longest].dist {
            conns.remove(idx_longest);
            conns.push(con);
        }
    }

    conns
}

#[allow(dead_code)]
pub fn part_one(input: &str) -> u64 {
    println!("Solving part 1!");

    // parse input
    let lines = input.lines().collect::<Vec<&str>>();
    
    // get vec of points
    let points: Vec<Point> = lines
        .iter()
        .map(|x| {
            x.split(",")
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| x.parse::<f32>().unwrap())
            .collect::<Vec<f32>>()
        })
        .collect::<Vec<Vec<f32>>>()
        .iter()
        .map(|v| Point{x: v[0], y:v[1], z: v[2]})
        .collect::<Vec<Point>>();

    // for the points add connections
    let mut conns: Vec<Connection> = Vec::new();
    for (i, p1) in points.iter().enumerate() {
        for (j, p2) in points.iter().enumerate().skip(i+1) {
            let con: Connection = Connection::new((p1.clone(), i), (p2.clone(), j));
            conns = add_connections(conns, con);
        }
    }

    // sort connections into circuits
    let mut circuits: Vec<Vec<usize>> = Vec::new();
    // start looping through connections
    for i in 0..conns.len() {
        let p1: &usize = &conns[i].id.0;
        let p2: &usize = &conns[i].id.1;
        let mut circuit_found = false;
        for j in 0..circuits.len() {
            // check if p1 or p2 in circuit
            if circuits[j].contains(p1) && !circuits[j].contains(p2) {
                circuits[j].push(*p2);
                circuit_found = true;
                break;
            } else if circuits[j].contains(p2) && !circuits[j].contains(p1) {
                circuits[j].push(*p1);
                circuit_found = true;
                break;
            } else if circuits[j].contains(p2) && circuits[j].contains(p1) {
                circuit_found = true;
                break;
            }
        }
        if !circuit_found {
            circuits.push(vec![*p1, *p2]);
        }
    }

    // check for connected circuits
    for i in 0..circuits.len()-1 {
        for j in i+1..circuits.len() {
            for p in circuits[i].clone() {
                if circuits[j].contains(&p) {
                    println!("\noverlap found!\n");
                    // add all items from i to j, so that they carry forward with overlap search
                    for i_p in circuits[i].clone() {
                        if !circuits[j].contains(&i_p) {
                            circuits[j].push(i_p);
                        }
                    }
                    // set circuit i to zero
                    circuits[i] = Vec::new();
                }
            }
        }
    }

    println!("Print circuits\n");
    let mut first: u64 = 0;
    let mut second: u64 = 0;
    let mut third: u64 = 0;
    for c in circuits {
        if c.len() as u64 >= first {
            third = second;
            second = first;
            first = c.len() as u64;
            println!("first: {:?}, second: {}, third: {}", first, second, third);
        } else if c.len() as u64 >= second {
            third = second;
            second = c.len() as u64;
            println!("first: {:?}, second: {}, third: {}", first, second, third);
        } else if c.len() as u64 > third {
            third = c.len() as u64;
            println!("first: {:?}, second: {}, third: {}", first, second, third);
        }
    }
    

    first * second * third
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

    const EXAMPLE_FILE: &str = "data/examples/08.txt";
    const INPUT_FILE: &str = "data/inputs/08.txt";

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
        assert_eq!(part_one(&input), 40);
    }

    #[test]
    fn test_point_distance() {
        let p1: Point = Point {x: 1.0, y: 2.0, z: 3.0};
        let p2: Point = Point {x: 2.0, y: 1.0, z: 2.0};
        let dist_p1_p2 = calcualte_distance(&p1, &p2);
        assert_eq!(dist_p1_p2, (3.0_f32).sqrt());
    }

    #[test]
    fn test_connection_struct() {
        let p1: Point = Point {x: 1.0, y: 2.0, z: 3.0};
        let p1_id: usize = 1;
        let p2: Point = Point {x: 2.0, y: 1.0, z: 2.0};
        let p2_id: usize = 2;
        let conn = Connection::new((p1, p1_id), (p2, p2_id));
        assert_eq!(conn.dist, (3.0_f32).sqrt());
        assert_eq!(conn.id, (1,2));
        assert_eq!(conn.p1, Point {x: 1.0, y: 2.0, z: 3.0});
        assert_eq!(conn.p2, Point {x: 2.0, y: 1.0, z: 2.0});
    }

    #[test]
    fn test_add_connection() {
        let p1: Point = Point {x: 1.0, y: 2.0, z: 3.0};
        let p1_id: usize = 1;
        let p2: Point = Point {x: 2.0, y: 1.0, z: 2.0};
        let p2_id: usize = 2;
        let con = Connection::new((p1, p1_id), (p2, p2_id));
        let mut conns: Vec<Connection> = Vec::new();
        conns = add_connections(conns, con);
        
        assert_eq!(conns.len(), 1);
    }

    #[test]
    #[ignore]
    fn part_two_example_input() {
        let input = fs::read_to_string(EXAMPLE_FILE)
        .expect("Should have been able to read the file");
        assert_eq!(part_two(&input), 40);
    }

}