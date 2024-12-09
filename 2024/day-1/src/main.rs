use std::fs::read_to_string;

fn parse_input() -> [Vec<u32>; 2] {
    let mut result = [vec![], vec![]];

    for line in read_to_string("./input.txt").unwrap().lines() {
        let matches: Vec<&str> = line.split_whitespace().collect();
        result[0].push(matches[0].parse::<u32>().unwrap());
        result[1].push(matches[1].parse::<u32>().unwrap());
    }

    return result;
}

fn total_distance() -> u32 {
    let mut sum = 0;
    let [ mut left, mut right ] = parse_input(); left.sort(); right.sort();
    for idx in 0..left.len() { sum += (right[idx] as i32 - left[idx] as i32).abs() };

    return sum as u32;
}

fn similarity_score() -> u32 {
    let mut similarity_score = 0;
    let [ mut left, mut right ] = parse_input();
    for num in left { similarity_score += num * right.iter().filter(|&n| *n == num).count() as u32 }

    return similarity_score;
}

fn main() {
    println!("{}", total_distance());
    println!("{}", similarity_score());
}
