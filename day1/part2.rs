use std::fs::read_to_string;

fn find_num(s: &str) -> u8 {
    let nums = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let num_digit_positions = 
        s
            .chars()
            .enumerate()
            .filter(|(_, x)| x.is_ascii_digit())
            .map(|(y, x)| (y, x as u8 - '0' as u8))
            .collect::<Vec<_>>();
    
    let mut num_string_positions = 
        nums
            .iter()
            .map(|num| s.match_indices(num).collect::<Vec<_>>())
            .flatten()
            .map(|(y, x)| (y, nums.iter().position(|n| *n == x).unwrap() as u8))
            .collect::<Vec<_>>();
    
    num_string_positions.sort_by_key(|n| n.0);

    let first_digit_digit = num_digit_positions.iter().next();
    let first_string_digit = num_string_positions.iter().next();

    let first_digit = 
        match (first_digit_digit, first_string_digit) {
            (None, None) => 0,
            (None, Some(val)) => val.1,
            (Some(val), None) => val.1,
            (Some(val1), Some(val2)) => {
                if val1.0 < val2.0 {
                    val1.1
                } else {
                    val2.1
                }
            }
        };

    let last_digit_digit = num_digit_positions.iter().last();
    let last_string_digit = num_string_positions.iter().last();

    let last_digit = 
        match (last_digit_digit, last_string_digit) {
            (None, None) => 0,
            (None, Some(val)) => val.1,
            (Some(val), None) => val.1,
            (Some(val1), Some(val2)) => {
                if val1.0 > val2.0 {
                    val1.1
                } else {
                    val2.1
                }
            }
        };
    
    first_digit * 10 + last_digit
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let mut sum = 0;
    for line in input.lines() {
        sum += find_num(line) as u32;
    }
    println!("The sum of all calibration values is {sum}")
}
