use std::{collections::HashMap, fs};
use pathfinding::prelude::bfs;

#[derive(Debug)]
enum NodeValue {
    Start,
    End,
    Height(u32),
}

pub fn elevation(c: char) -> u32 {
    let elev: HashMap<char, u32> = 
        ('a'..='z').collect::<Vec<char>>()
        .into_iter()
        .zip((1..=26).collect::<Vec<u32>>()
        .into_iter()).collect();
    elev[&c]
}

fn parse(ch: char) -> Option<NodeValue> {
    if ch == 'S' {
        Some(NodeValue::Start)
    } else if ch == 'E' {
        Some(NodeValue::End)
    } else {
        Some(NodeValue::Height(elevation(ch)))
    }
}

fn index_for(x: usize, y: usize, width: usize) -> usize {
    (y * width) + x
}

fn neighbours(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    heights: &HashMap<(usize, usize), u32>,
) -> Vec<(usize, usize)> {
    let xi = x as i64;
    let yi = y as i64;
    let all = vec![(xi - 1, yi), (xi + 1, yi), (xi, yi - 1), (xi, yi + 1)];

    let illegal_removed = all
        .into_iter()
        .filter(|(x, y)| *x >= 0 && *x < width as i64 && *y >= 0 && *y < height as i64)
        .map(|(x, y)| (x as usize, y as usize));

    let current_height = *heights.get(&(x, y)).unwrap();
    let max_height = current_height + 1;

    let too_high_removed = illegal_removed.into_iter().filter(|(x, y)| {
        let neighbour_height = *heights.get(&(*x, *y)).unwrap();
        neighbour_height <= max_height
    });

    Vec::from_iter(too_high_removed)
}

fn main() {
    let s = fs::read_to_string("./input").unwrap();
    let lines = Vec::from_iter(s.split('\n'));

    let width = lines.first().unwrap().len();
    let height = lines.len();

    let parsed = Vec::from_iter(lines.iter().flat_map(|line| line.chars().flat_map(parse)));

    let mut heights: HashMap<(usize, usize), u32> = HashMap::new();
    let mut start_opt: Option<(usize, usize)> = None;
    let mut end_opt: Option<(usize, usize)> = None;

    for x in 0..width {
        for y in 0..height {
            let value = parsed.get(index_for(x, y, width)).unwrap();
            match value {
                NodeValue::Start => {
                    start_opt = Some((x, y));
                    heights.insert((x, y), elevation('a'));
                }
                NodeValue::End => {
                    end_opt = Some((x, y));
                    heights.insert((x, y), elevation('z'));
                }
                NodeValue::Height(n) => {
                    heights.insert((x, y), *n);
                }
            }
        }
    }

    let start = start_opt.unwrap();
    let end = end_opt.unwrap();

    let path = 
        bfs(&start, |(x, y)| neighbours(*x, *y, width, height, &heights), |(x, y)| *x == end.0 && *y == end.1);
    println!("the answer is {}", path.unwrap().len() - 1);
}
