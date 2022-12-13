use std::io::BufRead;
use std::{fs::File, io};

fn parse_tree_heights(strings: Vec<String>) -> Vec<u32> {
    Vec::from_iter(strings.iter().flat_map(|s| s.chars().map(|c| char::to_digit(c, 10).unwrap())))
}

fn value_at(x: i32, y: i32, vec: &Vec<u32>) -> u32 {
    let per_row = f64::sqrt(vec.len() as f64) as usize;
    *vec.get((y as usize * per_row) + x as usize).unwrap()
}

fn number_visible(x: i32, y: i32, dx: i32, dy: i32, vec: &Vec<u32>) -> i32 {
    let per_row = f64::sqrt(vec.len() as f64) as i32;

    let value = value_at(x, y, vec);

    let mut current_x = x + dx;
    let mut current_y = y + dy;

    let mut counter = 0;

    while current_x >= 0 && current_x < per_row && current_y  >= 0 && current_y < per_row {
        counter += 1;
        if value_at(current_x, current_y, vec) >= value {
            break;
        }
        current_x += dx;
        current_y += dy;
    }
    
    counter
}

fn main() {

    let file = File::open("./input").unwrap();
    let lines = io::BufReader::new(file).lines();

    let strings = lines.flat_map(|l| l.ok());
    let tree_heights = parse_tree_heights(Vec::from_iter(strings));

    let per_row = f64::sqrt(tree_heights.len() as f64) as i32;

    let mut max_score: i32 = -1;

    for x in 1..per_row - 1 {
        for y in 1..per_row - 1 {
            let left = number_visible(x, y, -1, 0, &tree_heights);
            let right = number_visible(x, y, 1, 0, &tree_heights);
            let top = number_visible(x, y, 0, -1, &tree_heights);
            let bottom = number_visible(x, y, 0, 1, &tree_heights);
            let score = left * right * top * bottom;

            max_score = max_score.max(score);
        }
    }

    println!("The answer is {}", max_score);
}
