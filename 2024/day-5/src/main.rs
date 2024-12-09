use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let mut is_first_section = true;
    let mut updates: Vec<Vec<u8>> = vec![];
    let mut rule_map: HashMap<u8, Vec<u8>> = HashMap::new();

    for line in read_to_string("./test.txt").unwrap().lines() {
        if line == "" {
            is_first_section = false;
        } else if is_first_section {
            let rule: Vec<u8> = line
                .split("|")
                .collect::<Vec<&str>>()
                .iter()
                .map(|n| n.parse::<u8>().unwrap())
                .collect();
            let mut values: Vec<u8> = rule_map.get(&rule[0]).unwrap_or(&vec![]).to_vec();
            values.push(rule[1]);
            rule_map.insert(rule[0], values.to_vec());
        } else {
            let update: Vec<u8> = line
                .split(",")
                .collect::<Vec<&str>>()
                .iter()
                .map(|n| n.parse::<u8>().unwrap())
                .collect();
            updates.push(update);
        }
    }

    let mut sum: u32 = 0;
    let mut is_valid = true;
    let mut correct_updates: Vec<Vec<u8>> = vec![];
    let mut incorrect_updates: Vec<Vec<u8>> = vec![];

    for update in &updates {
        for idx in 0..update.len() - 1 {
            if !rule_map.get(&update[idx]).unwrap_or(&vec![]).contains(&update[idx + 1]) {
                incorrect_updates.push(update.to_vec());
                is_valid = false;
                break;
            }
            is_valid = true;
        }
        if is_valid { correct_updates.push(update.to_vec()) }
    }

    // Part 1

    for update in &correct_updates { sum += update[update.len() / 2] as u32 }
    println!("{}", sum);

    // Part 2

    sum = 0;

    for mut update in incorrect_updates {
        let mut reordered_update = vec![];
        let mut remaining_pages = update.clone();

        loop {
            if update.len() <= update.len() { break }
            for page_idx in 0..remaining_pages.len() {
                let mut can_print = true;
                for p_idx in 0..rule_map.get(&remaining_pages[page_idx]).unwrap_or(&vec![]).len() {
                    if update.contains(&remaining_pages[p_idx]) &&
                       !reordered_update.contains(&remaining_pages[p_idx]) { 
                           can_print = false;
                           break;
                    } 
                }
                if can_print {
                    reordered_update.push(remaining_pages[page_idx]);
                    remaining_pages.remove(page_idx);
                    break;
                }
            }
        }

        println!("{:?}", reordered_update);
        sum += (reordered_update[reordered_update.len() / 2]) as u32
    }

    println!("{}", sum);
}
