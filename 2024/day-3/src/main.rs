use regex::Regex;
use std::fs::read_to_string;

fn parse_input_pt1() -> Vec<[u16; 2]> {
    let mut recovered_data = vec![];
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let corrupt_data: String = read_to_string("./input.txt").unwrap();

    for (_, [num1, num2]) in re.captures_iter(&corrupt_data).map(|c| c.extract()) {
        recovered_data.push([
            num1.parse::<u16>().unwrap(),
            num2.parse::<u16>().unwrap()
        ])
    }

    return recovered_data;
}

fn parse_input_pt2() -> Vec<[u16; 2]> {
    let mut is_enabled = true;
    let mut recovered_data = vec![];
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do()()\(\)|don't()()\(\)").unwrap();
    let corrupt_data: String = read_to_string("./input.txt").unwrap();

    for (flag, [num1, num2]) in re.captures_iter(&corrupt_data).map(|c| c.extract()) {
        if flag == "don't()" { is_enabled = false; continue }
        else if flag == "do()" { is_enabled = true; continue }
        if is_enabled { recovered_data.push([
            num1.parse::<u16>().unwrap(),
            num2.parse::<u16>().unwrap()
        ])}
    }

    return recovered_data;
}

fn main() {
    let mut sum1: u32 = 0;
    let mut sum2: u32 = 0;

    for op in parse_input_pt1() { sum1 += op[0] as u32 * op[1] as u32 }
    for op in parse_input_pt2() { sum2 += op[0] as u32 * op[1] as u32 }

    println!("{}", sum1);
    println!("{}", sum2);
}
