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


fn split_beams(beams: Vec<u64>, row: Vec<char>) -> Vec<u64> {
    let num_cols = beams.len() - 1;
    let mut updated_beams = beams.clone();
    for (i, el) in beams.iter().enumerate() {
        if row[i] == '^' && *el > 0 {
            // split the beam
            // left beam
            if i > 0 {
                updated_beams[i-1] += *el;
            }
            // right beam
            if i < num_cols {
                updated_beams[i+1] += *el;
            }
            // remove original beam path
            updated_beams[i] = 0;

        } 
    }

    updated_beams
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

    // define a vec to hold the number of beam instances at each column position
    // initialise if from start postion
    let mut beams: Vec<u64> = Vec::new();
    for (i, _) in arr[0].iter().enumerate() {
        if i == start_idx {
            beams.push(1);
        } else {
            beams.push(0);
        }
    }

    // go row by row and update the beams vector with the number of beam 
    // instances at each position
    for (row, _) in arr.iter().enumerate().skip(1) {
        // check corresponding arr character for beam position
        if row % 2 == 0 {
            beams = split_beams(beams, arr[row].clone());
        }
        // println!("row: {:?}", row);
        // println!("{:?}", beams);
    }

    beams.iter().sum()
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

    #[test]
    fn part_two_example_input2() {
        let input = fs::read_to_string("data/examples/07-2.txt")
        .expect("Should have been able to read the file");
        assert_eq!(part_two(&input), 8);
    }

}