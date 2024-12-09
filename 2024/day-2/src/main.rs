use std::fs::read_to_string;

fn parse_input() -> Vec<Vec<u8>> {
    let mut result: Vec<Vec<u8>> = vec![];

    for line in read_to_string("./input.txt").unwrap().lines() {
        result.push(line.split_whitespace().collect::<Vec<&str>>()
               .iter().map(|n| n.parse::<u8>().unwrap()).collect());
    }

    return result;
}

fn is_safe(report: Vec<u8>) -> bool {
    for idx in 0..report.len() - 1 {
        let absolute_diff = (report[idx] as i32 - report[idx + 1] as i32).abs();
        if absolute_diff < 1 || absolute_diff > 3  { return false }
    }

    return report.is_sorted_by(|a, b| a <= b) || report.is_sorted_by(|a, b| a >= b);
}

fn is_damp_safe(report: Vec<u8>) -> bool {
    for damp_idx in 0..report.len() {
        let mut is_stable = true;
        let mut damped_report = report.clone(); damped_report.remove(damp_idx);

        if !(damped_report.is_sorted_by(|a, b| a <= b) || damped_report.is_sorted_by(|a, b| a >= b)) { continue };

        for idx in 0..damped_report.len() - 1 {
            let absolute_diff = (damped_report[idx] as i16 - damped_report[idx + 1] as i16).abs();
            if absolute_diff < 1 || absolute_diff > 3  { is_stable = false }
        }
        if is_stable { return true }
    }

    return false;
}

fn main() {
    let mut safe_count: u16 = 0;
    let mut damp_safe_count: u16 = 0;

    for report in parse_input() { if is_safe(report) { safe_count += 1 } }
    for report in parse_input() { if is_damp_safe(report) { damp_safe_count += 1 } }

    println!("{}", safe_count);
    println!("{}", damp_safe_count);
}
