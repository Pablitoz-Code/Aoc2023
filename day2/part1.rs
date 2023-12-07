use std::fs::read_to_string;

fn parse_cubes(s: &str) -> Vec<Vec<(usize, &str)>> {
    let mut cubes = Vec::new();
    for i in s
        .split_once(':')
        .unwrap()
        .1
        .split([';']) {
            let nums = i.split(',');
            let mut curr_cubes = Vec::new();
            for j in nums {
                let (num, color) = j.trim().split_once(' ').unwrap();
                curr_cubes.push((num.parse().unwrap(), color.trim()))
            }
            cubes.push(curr_cubes);
    }
    cubes
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let mut id_sum = 0;
    for (id, line) in input.lines().enumerate() {
        let cubes = parse_cubes(line);
        let mut valid = true;
        for curr_cubes in cubes {
            let mut totals = [0, 0, 0];
            for cube in curr_cubes {
                match cube.1 {
                    "red" => {totals[0] += cube.0},
                    "green" => {totals[1] += cube.0},
                    "blue" => {totals[2] += cube.0},
                    _ => {unreachable!()}
                }
            }
            if !(totals[0] <= 12 && totals[1] <= 13 && totals[2] <= 14) {
                valid = false;
                break;
            }
        }   
        if valid {
            id_sum += id + 1;
        }
    }

    println!("The sum of the IDs of the games is {id_sum}")
}
