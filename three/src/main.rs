use std::collections::HashSet;
use std::io::BufRead;
use std::{fs::File, io};

fn main() {
    let file = File::open("./input").unwrap();
    let lines = io::BufReader::new(file).lines();

    let strings = lines.flat_map(|l| l.ok());
    let answer: u32 = 
        Vec::from_iter(strings)
            .chunks(3)
            .map(common_group)
            .flat_map(|c|c.into_iter().next())
            .map(priority_of)
            .sum();

    println!("The answer  {}", answer)
}

fn priority_of(c: char) -> u32 {
    if c.is_ascii_uppercase() {
        (c as u32) - ('A' as u32) + 1 + 26
    } else {
        (c as u32) - ('a' as u32) + 1
    }
}


fn common_group(strings: &[String]) -> HashSet<char> {
    let chars: Vec<HashSet<char>> = strings
        .into_iter()
        .map(|s| HashSet::from_iter(s.chars()))
        .collect();

    fn intersect(head: &HashSet<char>, tail: &[HashSet<char>]) -> HashSet<char> {
        let mut intersection = head.clone();

        tail.iter().for_each(|set| {
            intersection = HashSet::from_iter(intersection.intersection(set).map(|c| *c)).clone()
        });

        intersection.clone()
    }

    match chars.split_first() {
        Some((head, tail)) => intersect(head, tail),
        None => HashSet::new(),
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet};

    use crate::{common_group, priority_of};

    #[test]
    fn test_priority_of() {
        assert_eq!(priority_of('a'), 1);
        assert_eq!(priority_of('A'), 27);
        assert_eq!(priority_of('p'), 16);
        assert_eq!(priority_of('L'), 38);
        assert_eq!(priority_of('t'), 20);
    }

    #[test]
    fn test_common_group() {
        let group = [
            "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
            "PmmdzqPrVvPwwTWBwg".to_string(),
        ];

        assert_eq!(common_group(&group), HashSet::from_iter(['r']));
    }
}
