use std::io::{BufRead};
use std::{fs::File, io};

fn main() {
    let file = File::open("./input").unwrap();
    let lines = io::BufReader::new(file).lines();

    let strings = lines.flat_map(|l| l.ok());
    let grouped_strings = group(strings.collect());
    let elves: Vec<Vec<i32>> = grouped_strings
        .iter()
        .map(|g| g.iter().flat_map(|s| i32::from_str_radix(s, 10).ok()).collect())
        .collect();

    let mut carrying: Vec<i32> = elves.iter().map(|elf| elf.iter().sum()).collect();

    carrying.sort_by(|a, b| b.cmp(a));

    let answer: i32 = carrying.iter().take(3).sum();

    println!("the answer is {}", answer);
}

fn group(vector: Vec<String>) -> Vec<Vec<String>> {
    vector.into_iter().fold(vec![vec![]], | mut acc, item| {
        if item.is_empty() {
            acc.push(Vec::new());
            acc
        }
        else {
            acc.last_mut().unwrap().push(item);
            acc
        }
    })
}
