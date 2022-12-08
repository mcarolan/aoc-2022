use std::io::BufRead;
use std::{fs::File, io};

#[derive(PartialEq, Debug)]
struct Assignment {
    lower: u32,
    upper: u32,
}

impl Assignment {
    fn consumes(&self, other: &Assignment) -> bool {
        self.lower <= other.lower && other.lower <= self.upper
            || self.lower <= other.upper && other.upper <= self.upper
            || other.lower <= self.lower && self.lower <= other.upper
            || other.lower <= self.upper && self.upper <= other.upper
    }
}

#[derive(PartialEq, Debug)]
struct AssignmentPair {
    a: Assignment,
    b: Assignment,
}

impl AssignmentPair {
    fn has_overlap(&self) -> bool {
        self.a.consumes(&self.b) || self.b.consumes(&self.a)
    }
}

fn parse_u32(s: &&str) -> Option<u32> {
    u32::from_str_radix(s, 10).ok()
}

fn parse_assignment(s: &&str) -> Option<Assignment> {
    let bounds = Vec::from_iter(s.split('-'));
    let lower = bounds.get(0).and_then(parse_u32);
    let upper = bounds.get(1).and_then(parse_u32);

    lower
        .zip(upper)
        .map(|(lower, upper)| Assignment { lower, upper })
}

fn parse_assignment_pair(line: String) -> Option<AssignmentPair> {
    let pair = Vec::from_iter(line.split(','));
    let a = pair.get(0).and_then(parse_assignment);
    let b = pair.get(1).and_then(parse_assignment);
    a.zip(b).map(|(a, b)| AssignmentPair { a, b })
}

fn main() {
    let file = File::open("./input").unwrap();
    let lines = io::BufReader::new(file).lines();

    let strings = lines.flat_map(|l| l.ok());

    let answer = strings
        .into_iter()
        .flat_map(parse_assignment_pair)
        .filter(|p| p.has_overlap())
        .count();

    println!("The answer is {}", answer);
}

#[cfg(test)]
mod tests {
    use crate::{parse_assignment_pair, Assignment, AssignmentPair};

    #[test]
    fn test_consumes() {
        assert_eq!(
            Assignment { lower: 2, upper: 8 }.consumes(&Assignment { lower: 3, upper: 7 }),
            true
        );
        assert_eq!(
            Assignment { lower: 2, upper: 8 }.consumes(&Assignment { lower: 3, upper: 9 }),
            true
        );
    }

    #[test]
    fn test_parse_assignment_pair() {
        let input = "7-96,6-95".to_string();
        assert_eq!(
            parse_assignment_pair(input),
            Some(AssignmentPair {
                a: Assignment {
                    lower: 7,
                    upper: 96
                },
                b: Assignment {
                    lower: 6,
                    upper: 95
                }
            })
        );
    }
}
