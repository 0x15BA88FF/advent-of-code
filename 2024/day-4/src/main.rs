use std::fs::read_to_string;

fn offset_recursion(word_vect: &Vec<&str>, pos: [i16; 2], offset: [i16; 2], target_chars: Vec<char>, target_idx: usize) -> u8 {
    let mut score = 0;

    if target_idx >= target_chars.len() { return score };
    if pos[1] < 0 || pos[1] >= word_vect.len() as i16 { return score }
    if pos[0] < 0 || pos[0] >= word_vect[pos[1] as usize].len() as i16 { return score }
    if word_vect[pos[0] as usize].chars().nth(pos[1] as usize).unwrap() != target_chars[target_idx] { return score }

    if target_idx < target_chars.len() {
        score += offset_recursion(word_vect, [pos[0] + offset[0], pos[1] + offset[1]], offset, target_chars, target_idx + 1)
    }

    return score + 1;
}

fn xmas_iteration(word_vect: &Vec<&str>, pos: [i16; 2]) -> u8 {
    let mut sum = 0;
    for y in -1..1 { for x in -1..1 { sum += offset_recursion(word_vect, pos, [ x, y], vec!['X', 'M', 'A', 'S'], 0) / 4 }}
    return sum;
}

fn x_mas_iteration(word_vect: &Vec<&str>, pos: [usize; 2]) -> u8 {
    let mut score = 0;
    let edge_patterns = [['M', 'S', 'M', 'S'], ['M', 'M', 'S', 'S'], ['S', 'M', 'S', 'M'], ['S', 'S', 'M', 'M']];

    if word_vect[pos[1]].chars().nth(pos[0]).unwrap() == 'A' {
        if pos[1] < 1 || pos[1] >= word_vect.len() - 1 { return score }
        if pos[0] < 1 || pos[0] >= word_vect[pos[1]].len() - 1 { return score }
        let edges = [
            word_vect[pos[1] - 1].chars().nth(pos[0] - 1).unwrap(),
            word_vect[pos[1] - 1].chars().nth(pos[0] + 1).unwrap(),
            word_vect[pos[1] + 1].chars().nth(pos[0] - 1).unwrap(),
            word_vect[pos[1] + 1].chars().nth(pos[0] + 1).unwrap()
        ];
        println!("Edge Pattern: {:?}", edges);
        for pattern in edge_patterns { if edges == pattern { score += 1; break }}
    }

    return score;
}

fn main() {
    let mut sum: u16 = 0;
    let mut word_vect: Vec<&str> = vec![];

    let binding = read_to_string("./input.txt").unwrap();
    for line in binding.lines() { word_vect.push(line) }

    for line_idx in 0..word_vect.len() {
        for char_idx in 0..word_vect[line_idx].len() {
            //sum += xmas_iteration(&word_vect, [char_idx as i16, line_idx as i16]) as u16;
            sum += x_mas_iteration(&word_vect, [char_idx, line_idx]) as u16;
        }
    }

    println!("{:?}", sum);
}
