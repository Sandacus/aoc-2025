// day 7 solution

#[warn(dead_code)]
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
        println!("{:?}", line);
        for ch in line {
            if ch == '+' {
                // println!("{}", ch);
                count += 1;
            }
        }
    }
    
    count
}

#[warn(dead_code)]
fn find_start(line: &[char]) -> Result<usize, String> {
    for (idx, ch) in line.iter().enumerate() {
        if *ch == 'S' {
            return Ok(idx);
        }
    }
    Err("Couldn't find start point".to_string())
}

#[warn(dead_code)]
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

#[warn(dead_code)]
pub fn part_two(input: &str) -> u64 {
    println!("Solving part 2!");

    // parse input
    let lines: Vec<&str> = input.lines().collect();
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
    let mut beam_count: u64 = 1;
    for line in arr.iter_mut().skip(1) {
        let (updated_vec, new_beam) = beam_down_count(beam_down_ids.clone(), line.clone());
        *line = updated_vec;
        beam_count += new_beam;

        // recalculate beam_down_ids for next loop
        beam_down_ids = Vec::new();
        for (i, ch) in line.iter().enumerate() {
            if ch == &'|' {
                beam_down_ids.push(i);
            }
        }
    }

    // take a peek at arr
    // for l in arr {
    //     println!("{:?}", l);
    // }

    // search through array for all beam paths
    // beam_path_search(start_idx, &arr[1..], 1)
    beam_count
}

#[warn(dead_code)]
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
        let input = fs::read_to_string("data/examples/07-2.txt")
        .expect("Should have been able to read the file");
        assert_eq!(part_one(&input), 6);
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

    #[test]
    fn part_two_example_2() {
        let input = fs::read_to_string("data/examples/07-2.txt")
        .expect("Should have been able to read the file");
        assert_eq!(part_two(&input), 8);
    }

}