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
    let mut sum = 0;
    for line in input.lines() {
        let cubes = parse_cubes(line);
        let mut mins = [0, 0, 0];
        for curr_cubes in cubes {
            let mut curr_sums = [0, 0, 0];
            for cube in curr_cubes {
                match cube.1 {
                    "red" => {curr_sums[0] += cube.0},
                    "green" => {curr_sums[1] += cube.0},
                    "blue" => {curr_sums[2] += cube.0},
                    _ => {unreachable!()}
                }
            }
            for i in 0..3 {
                mins[i] = mins[i].max(curr_sums[i])
            }
        }
        sum += mins[0] * mins[1] * mins[2]
    }

    println!("The sum of the power of the sets is {sum}")
}
