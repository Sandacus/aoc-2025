// day 7 solution

#[allow(dead_code)]
pub fn part_one(input: &str) -> u64 {
    println!("Solving part 1!");

    // parse input
    let lines = input.lines().collect::<Vec<&str>>();
    let mut arr = lines
        .iter()
        .map(|x| 
            x.chars().collect::<Vec<char>>()
        )
        .collect::<Vec<Vec<char>>>();

    // find start S
    let start_idx: usize = find_start(&arr[0]).unwrap();

    // initialise beam down ids
    let mut beam_down_ids: Vec<usize> = vec![start_idx];

    // loop through lines of arr
    // beam down function that modifies the next line with where the beam will be
    for line in arr.iter_mut().skip(1) {
        let updated_vec = beam_down(beam_down_ids.clone(), line.clone());
        *line = updated_vec;

        // recalculate beam_down_ids for next loop
        beam_down_ids = Vec::new();
        for (i, ch) in line.iter().enumerate() {
            if ch == &'|' {
                beam_down_ids.push(i);
            }
        }
    }
    
    // once looped through 
    // count number of converted splitters in arr
    let mut count = 0;
    for line in arr {
        // println!("{:?}", line);
        for ch in line {
            if ch == '+' {
                // println!("{}", ch);
                count += 1;
            }
        }
    }
    
    count
}

#[allow(dead_code)]
fn find_start(line: &[char]) -> Result<usize, String> {
    for (idx, ch) in line.iter().enumerate() {
        if *ch == 'S' {
            return Ok(idx);
        }
    }
    Err("Couldn't find start point".to_string())
}

#[allow(dead_code)]
fn beam_down(ids: Vec<usize>, mut line: Vec<char>) -> Vec<char> {
    // --> beam down updates 
    // ----> free space: '.' to '|' 
    // ----> splitter: '^' to '|','+','|'
    for idx in ids {
        if line[idx] == '.' {
            line[idx] = '|';
        } else if line[idx] == '^' && idx == 0 {
            line[idx] = '+';
            line[idx+1] = '|';
        } else if line[idx] == '^' && idx == line.len()-1 {
            line[idx-1] = '|';
            line[idx] = '+';
        } else if line[idx] == '^' {
            line[idx-1] = '|';
            line[idx] = '+';
            line[idx+1] = '|';
        }
    }
    

    line
}

#[derive(Debug, Clone, Copy)]
struct Point {
    row: usize,
    col: usize
}

fn move_beams_down(beams: Vec<Point>) -> Vec<Point> {
    let mut updated_beams: Vec<Point> = Vec::new();
    for beam in beams {
        let row = beam.row + 1;
        let col = beam.col;
        updated_beams.push(Point { row, col })
    }

    updated_beams
}

fn split_beams(beams: Vec<Point>, row: Vec<char>) -> Vec<Point> {
    let mut split_beams: Vec<Point> = Vec::new();
    for beam in beams {
        if row[beam.col] == '^' {
            // split the beam
            // left beam
            let left_beam = beam_left(beam);
            match left_beam {
                Some(b) => split_beams.push(b),
                None => println!("at the edge, no beam"),
            }
            // right beam
            let right_beam = beam_right(beam, row.len()-1); 
            match right_beam {
                Some(b) => split_beams.push(b),
                None => println!("at the edge, no beam"),
            }

        } else {
            split_beams.push(beam);
        }
    }

    split_beams
}

fn beam_left(beam: Point) -> Option<Point> {
    if beam.col > 0 {
        let col = beam.col - 1;
        return Some(Point { row: beam.row, col })
    }

    None
}

fn beam_right(beam: Point, max: usize) -> Option<Point> {
    if beam.col < max {
        let col = beam.col + 1;
        return Some(Point { row: beam.row, col })
    }

    None
}

#[allow(dead_code)]
pub fn part_two(input: &str) -> u64 {
    println!("Solving part 2!");

    // parse input
    let lines: Vec<&str> = input.lines().collect();
    let arr = lines
       .iter()
       .map(|x| 
           x.chars().collect::<Vec<char>>()
       )
       .collect::<Vec<Vec<char>>>();

    // find start S
    let start_idx: usize = find_start(&arr[0]).unwrap();

    // define a beams vec to hold beam instances
    // initialise if from start postion
    let mut beams: Vec<Point> = vec![Point {row: 0, col: start_idx}];

    // algorithm goes
    // beam down -> update the rows for all beams
    // loop through beams vec
    // --> check corresponding char in array 
    // ---> if splitter update beam in updated_beams vec

    for row in 0..arr.len() {
        // check corresponding arr character for beam position
        if row % 2 == 0 {
            beams = split_beams(beams, arr[row].clone());
        }
        
        // move to next row
        beams = move_beams_down(beams);
        
        // update original beams
        println!("row: {:?}", row);
        println!("number of beams: {:?}", beams.len());
    }


    beams.len() as u64
}

#[allow(dead_code)]
fn beam_down_count(ids: Vec<usize>, mut line: Vec<char>) -> (Vec<char>, u64) {
    // --> beam down updates 
    // ----> free space: '.' to '|' 
    // ----> splitter: '^' to '|','+','|'
    let mut new_beam: u64 = 0;
    for idx in ids {
        if line[idx] == '.' {
            line[idx] = '|';
        } else if line[idx] == '^' && idx == 0 {
            line[idx] = '+';
            line[idx+1] = '|';
        } else if line[idx] == '^' && idx == line.len()-1 {
            line[idx-1] = '|';
            line[idx] = '+';
        } else if line[idx] == '^' {
            new_beam += 1;
            line[idx-1] = '|';
            line[idx] = '+';
            line[idx+1] = '|';
        }
    }
    

    (line, new_beam)
}

#[allow(dead_code)]
fn beam_path_search(idx: usize, arr: &[Vec<char>], mut beam_path_count: u64) -> u64 {
    println!("in beam path count: {}", beam_path_count);
    if arr.len() == 1 {
        return beam_path_count;
    }
    
    // if beam drop down a row
    if arr[0][idx] == '|' && arr.len() > 1 {
        beam_path_count = beam_path_search(idx, &arr[1..], beam_path_count);
    }
    // if splitter
    if arr[0][idx] == '+' && arr.len() > 1 {
        beam_path_count += 1;
        if idx > 0 { // left
            let left_id = idx - 1;
            beam_path_count = beam_path_search(left_id, &arr[1..], beam_path_count);
        }
        if idx < arr[0].len() - 2 { // right
            let right_id = idx + 1;
            beam_path_count = beam_path_search(right_id, &arr[1..], beam_path_count);
        }
    }
    
    beam_path_count
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const EXAMPLE_FILE: &str = "data/examples/07.txt";
    const INPUT_FILE: &str = "data/inputs/07.txt";

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
        assert_eq!(part_one(&input), 21);
    }

    #[test]
    fn test_part_one() {
        let input = fs::read_to_string("data/examples/07.txt")
        .expect("Should have been able to read the file");
        assert_eq!(part_one(&input), 21);
    }

    #[test]
    fn test_beam_down() {
        let input = vec!['.','.','.'];
        let expected_output = vec!['.','|','.'];
        assert_eq!(beam_down(vec![1], input), expected_output);
    }

    #[test]
    fn test_beam_down_with_split() {
        let input = vec!['.','.','^','.','.'];
        let expected_output = vec!['.','|','+','|','.'];
        assert_eq!(beam_down(vec![2], input), expected_output);
    }

    #[test]
    fn test_beam_down_with_split_at_left_edge() {
        let input = vec!['^','.','.','.','.'];
        let expected_output = vec!['+','|','.','.','.'];
        assert_eq!(beam_down(vec![0], input), expected_output);
    }

    #[test]
    fn test_beam_down_with_split_at_right_edge() {
        let input = vec!['.','.','.','.','^'];
        let expected_output = vec!['.','.','.','|','+'];
        assert_eq!(beam_down(vec![4], input), expected_output);
    }

    #[test]
    fn test_can_find_start() {
        let input = vec!['.','.','S','.','.'];
        let expected_output: Result<usize, String> = Ok(2);
        assert_eq!(find_start(&input), expected_output);
    }


    #[test]
    fn part_two_example_input() {
        let input = fs::read_to_string(EXAMPLE_FILE)
        .expect("Should have been able to read the file");
        assert_eq!(part_two(&input), 40);
    }

}