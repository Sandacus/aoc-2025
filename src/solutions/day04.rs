// day 4 solution

pub fn part_one(input: &str) -> u64 {
    println!("Solving part 1!");
    
    let arr: Vec<Vec<char>> = input
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.chars().collect())
        .collect::<Vec<Vec<char>>>();

    // println!("{:?}", arr); 

    let mut accessible: u64 = 0;
    // loop through array
    for (i, row) in arr.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if arr[i][j] != '@' {
                continue;
            }

            let mut count: u64 = 0;

            // check left
            if j > 0 {
                count += check_left(i, j, &arr);
            }

            // check right
            if j < arr[i].len()-1 {
                count += check_right(i, j, &arr);
            }

            // check top
            if i > 0 {
                count += check_up(i, j, &arr);
            }

            // check down
            if i < arr.len()-1 {
                count += check_down(i, j, &arr);
            }

            // check top left
            if j > 0 && i > 0 {
                count += check_up_left(i, j, &arr);
            }

            // check top right
            if j < arr[i].len()-1 && i > 0 {
                count += check_up_right(i, j, &arr);
            }

            // check bottom right
            if j < arr[i].len()-1 && i < arr.len()-1 {
                count += check_down_right(i, j, &arr);
            }

            // check bottom left
            if j > 0 && i < arr.len()-1 {
                count += check_down_left(i, j, &arr);
            }
            
            if count < 4 {
                println!("[{}, {}]", i, j);
                accessible += 1;
            }
        }
    }



    accessible
}

fn check_right(i: usize, j: usize, arr: &[Vec<char>]) -> u64 {
    if arr[i][j+1] == '@' {
        // println!("paper roll to the right");
        return 1;
    }  
    0
}

fn check_left(i: usize, j: usize, arr: &[Vec<char>]) -> u64 {
    if arr[i][j-1] == '@' {
        // println!("paper roll to the left");
        return 1;
    }
    0
}

fn check_up(i: usize, j: usize, arr: &[Vec<char>]) -> u64 {
    if arr[i-1][j] == '@' {
        // println!("paper roll above");
        return 1;
    }
    0
}

fn check_down(i: usize, j: usize, arr: &[Vec<char>]) -> u64 {
    if arr[i+1][j] == '@' {
        // println!("paper roll below");
        return 1;
    }
    0
}

fn check_up_right(i: usize, j: usize, arr: &[Vec<char>]) -> u64 {
    if arr[i-1][j+1] == '@' {
        // println!("paper roll top right");
        return 1;
    }
    0
}

fn check_up_left(i: usize, j: usize, arr: &[Vec<char>]) -> u64 {
    if arr[i-1][j-1] == '@' {
        // println!("paper roll top left");
        return 1;
    }
    0
}

fn check_down_right(i: usize, j: usize, arr: &[Vec<char>]) -> u64 {
    if arr[i+1][j+1] == '@' {
        // println!("paper roll bottom right");
        return 1;
    }
    0
}

fn check_down_left(i: usize, j: usize, arr: &[Vec<char>]) -> u64 {
    if arr[i+1][j-1] == '@' {
        // println!("paper roll bottom left");
        return 1;
    }
    0
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
        assert!(fs::exists("data/inputs/04.txt").expect("couldn't find input"));
    }

    #[test]
    fn read_example() {
        assert!(fs::exists("data/examples/04.txt").expect("couldn't find example"));
    }

    #[test]
    fn part_one_example_input() {
        let input = fs::read_to_string("data/examples/04.txt")
        .expect("Should have been able to read the file");
        assert_eq!(part_one(&input), 13);
    }


    #[test]
    #[ignore]
    fn part_two_example_input() {
        let input = fs::read_to_string("data/examples/04.txt")
        .expect("Should have been able to read the file");
        assert_eq!(part_two(&input), 0);
    }


}