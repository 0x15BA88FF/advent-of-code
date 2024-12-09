use std::fs::read_to_string;

fn get_guard_dir(guard: char) -> [i16; 2] {
    if guard == '^' { return [0, -1] }
    if guard == '>' { return [1, 0] }
    if guard == 'v' { return [0, 1] }
    if guard == '<' { return [-1, 0] }
    else { return [0, 0] }
}

fn print_mat(mat: &mut Vec<Vec<char>>) {
    println!("");
    for vect in mat {
        println!("{:?}", vect);
    }
    println!("");
}

fn predict_path(mut pos_matrix: Vec<Vec<char>>, mut guard_pos: [i16; 2], mut guard_dir: [i16; 2]) -> i32 {
    let mut sum: i32 = 0;
    let mut breadcrumb_trail: [[[i16; 2]; 130]; 130] = [[[0; 2]; 130]; 130];

    loop {
        let next_pos = [guard_pos[0] + guard_dir[0], guard_pos[1] + guard_dir[1]];

        if next_pos[0] < 0 || next_pos[1] < 0 { return sum }
        if next_pos[1] >= pos_matrix.len() as i16 || next_pos[0] >= pos_matrix[next_pos[1] as usize].len() as i16 { return sum }
        if pos_matrix[next_pos[1] as usize][next_pos[0] as usize] != '#' {
            guard_pos = next_pos;
            if breadcrumb_trail[guard_pos[1] as usize][guard_pos[0] as usize] == guard_dir { return -1 }
            breadcrumb_trail[guard_pos[1] as usize][guard_pos[0] as usize] = guard_dir;
            if pos_matrix[next_pos[1] as usize][next_pos[0] as usize] != 'x' {
                pos_matrix[next_pos[1] as usize][next_pos[0] as usize] = 'x';
                sum += 1;
            }
        } else {
            guard_dir = [-guard_dir[1], guard_dir[0]];
        }
    }
}

fn main() {
    let mut pos_matrix: Vec<Vec<char>> = vec![];
    let mut guard_pos: [i16; 2] = [0, 0];
    let mut guard_dir: [i16; 2] = [0, 0];

    let binding = read_to_string("./input.txt").unwrap();
    for (line_idx, line) in binding.lines().enumerate() {
        let chars = line.chars().collect::<Vec<char>>();
        let guard_x = chars.iter().position(|&i| get_guard_dir(i) != [0, 0]);

        pos_matrix.push(chars.clone());
        if guard_x.is_some() {
            guard_pos = [guard_x.unwrap() as i16, line_idx as i16];
            guard_dir = get_guard_dir(chars[guard_x.unwrap()]);
        }
    }

    // Part 1
    println!("{}", predict_path(pos_matrix.clone(), guard_pos, guard_dir));

    // Part 2
    let mut loop_count = 0;

    for y in 0..pos_matrix.len() {
        for x in 0..pos_matrix[y].len() {
            if pos_matrix[y][x] == '#' { continue }
            let mut new_pos_martix = pos_matrix.clone();
            new_pos_martix[y][x] = '#';

            if get_guard_dir(pos_matrix[y][x]) != [0, 0]  { continue }
            if predict_path(new_pos_martix, guard_pos, guard_dir) == -1 { loop_count += 1 };
        }
    }

    println!(loop_count);
}
