use std::fs::read_to_string;

fn try_operations(target: u64, nums: &[u64]) -> u64 {
    let mut count = 0;

    if nums.len() <= 1 {
        if nums[0] == target {
            count += 1;
        }
        return count;
    }

    if target % nums[nums.len() - 1] == 0 {
        count += try_operations(target / nums[nums.len() - 1], &nums[..nums.len() - 1])
    };
    if target >= nums[nums.len() - 1] {
        count += try_operations(target - nums[nums.len() - 1], &nums[..nums.len() - 1])
    }
    if target.to_string().len() > nums[nums.len() - 1].to_string().len() {
        if target.to_string()[target.to_string().len() - nums[nums.len() - 1].to_string().len()..] == nums[nums.len() - 1].to_string() {
            count += try_operations(
                target.to_string()[..target.to_string().len() - nums[nums.len() - 1].to_string().len()].parse::<u64>().unwrap(),
                &nums[..nums.len() - 1]
            )
        };
    }

    return count;
}

fn main() {
    let mut equations: Vec<Vec<u64>> = vec![vec![]];

    for line in read_to_string("./input.txt").unwrap().lines() {
        let line = line.to_string().replace(":", "");
        equations.push(
            line.split(" ")
                .collect::<Vec<&str>>()
                .iter()
                .map(|a| a.parse::<u64>().unwrap())
                .collect::<Vec<u64>>(),
        )
    }

    // Part 1
    let mut sum = 0;

    for equation in &equations[1..] {
        if try_operations(equation[0], &equation[1..]) > 0 {
            sum += equation[0]
        };
    }

    println!("{}", sum);
}
