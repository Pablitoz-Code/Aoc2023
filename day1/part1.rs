use std::fs::read_to_string;
use std::iter::FromIterator;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let mut sum = 0;
    for line in input.lines() {
        let chars = line.chars();
        let first_digit = chars.clone().find(|c| c.is_ascii_digit()).unwrap();
        let last_digit = chars.rev().find(|c| c.is_ascii_digit()).unwrap();
        sum += String::from_iter([first_digit, last_digit]).parse::<i32>().unwrap()
    }
    println!("The sum of all calibration values is {sum}")
}
