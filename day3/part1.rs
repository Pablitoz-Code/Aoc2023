use std::fs::read_to_string;

fn valid_squares(matrix: &Vec<Vec<char>>) -> Vec<Vec<bool>> {
    let mut squares = vec![vec![false; matrix[0].len()]; matrix.len()];
    for (i, row) in matrix.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c != '.' && !c.is_numeric() {
                squares[i][j] = true;
                for k in -1..=1 {
                    for l in -1..=1 {
                        if let Some(val) = squares.get_mut((i as i32 + k) as usize) {
                            if let Some(val2) = val.get_mut((j as i32 + l) as usize) {
                                *val2 = true;
                            }
                        }
                    }
                }
            }
        }
    }
    squares
}

fn get_numbers(matrix: &Vec<Vec<char>>, squares: &Vec<Vec<bool>>) -> Vec<u32> {
    let mut numbers = Vec::new();
    for (i, row) in matrix.iter().enumerate() {
        let mut num_end = 0;
        for (j, c) in row.iter().enumerate() {
            if squares[i][j] && c.is_numeric() && j >= num_end {
                let mut num_start: i32 = j as i32;
                let mut parsed_num = 0;
                while num_start >= 0 && row[num_start as usize].is_numeric() {
                    num_start -= 1;
                }
                num_start += 1;
                while num_start < row.len() as i32 && row[num_start as usize].is_numeric() {
                    parsed_num *= 10;
                    parsed_num += row[num_start as usize].to_digit(10).unwrap();
                    num_start += 1;
                }
                num_end = num_start as usize;
                numbers.push(parsed_num);
            }
        }
    }
    numbers
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let squares = valid_squares(&matrix);
    let numbers = get_numbers(&matrix, &squares);
    println!("{}", numbers.iter().sum::<u32>());
}
