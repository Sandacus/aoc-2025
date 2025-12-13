// day 8 solution

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
    pair: (Point, Point),
    dist: f32
}

#[allow(dead_code)]
impl Connection {
    #[warn(dead_code)]
    fn new(p1: Point, p2: Point) -> Self {
        let dist = calcualte_distance(&p1, &p2);
        let p = (p1, p2);
        Self { pair: p, dist }
    }
}

#[allow(dead_code)]
struct Circuit {
    conns: Vec<Connection>,
    len: u64
}

#[allow(dead_code)]
impl Circuit {
    fn new(conns: Vec<Connection>) -> Self {
        let len = conns.len() as u64;
        Self { conns, len}
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
fn update_connections(mut conns: Vec<Connection>, con: &Connection) -> Vec<Connection> {
    for idx in 0..conns.len() {
        if con.dist > conns[idx].dist {
            conns.remove(idx);
            conns.push(con.clone());
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

    // let delta = calcualte_distance(&points[0], &points[1]);
    // println!("{:?}\n{:?} \ndistance: {:.0}", points[0], points[1], delta);

    // for the points add connections
    // only keep the top 10 shortest connections
    let mut conns: Vec<Connection> = Vec::new();
    for i in 0..points.len()-1 {
        for j in i+1..points.len() {
            let p1 = points[i].clone();
            let p2 = points[j].clone();
            let con = Connection::new(p1, p2);
            if conns.len() < 10 {
                conns.push(con);
            } else {
                conns = update_connections(conns, &con);
            }
        }
    }

    println!("conns: {:?}", conns);


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
        let p2: Point = Point {x: 2.0, y: 1.0, z: 2.0};
        let conn = Connection::new(p1, p2);
        assert_eq!(conn.dist, (3.0_f32).sqrt());
        assert_eq!(conn.pair, (Point {x: 1.0, y: 2.0, z: 3.0}, Point {x: 2.0, y: 1.0, z: 2.0}));
    }

    #[test]
    #[ignore]
    fn part_two_example_input() {
        let input = fs::read_to_string(EXAMPLE_FILE)
        .expect("Should have been able to read the file");
        assert_eq!(part_two(&input), 40);
    }

}