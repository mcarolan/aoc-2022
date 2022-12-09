use std::collections::HashSet;
use std::io::BufRead;
use std::{fs::File, io};

fn main() {
    let file = File::open("./input").unwrap();
    let lines = io::BufReader::new(file).lines();
    let strings = lines.flat_map(|l| l.ok());

    strings
        .into_iter()
        .map(first_marker_offset)
        .for_each(|offset| println!("{}", offset));
}

fn number_unqiue(xs: &Vec<char>) -> usize {
    let set: HashSet<char> = HashSet::from_iter(xs.iter().map(|c| *c));
    set.len()
}

fn first_marker_offset(s: String) -> usize {
    let mut last_seen: Vec<char> = Vec::new();

    let result = s.char_indices().find(|(_i, c)| {
        if last_seen.len() == 14 {
            last_seen.remove(0);
        }
        last_seen.push(*c);
        number_unqiue(&last_seen) == 14
    });

    result.unwrap().0 + 1
}

#[cfg(test)]
mod tests {
    use crate::first_marker_offset;

    #[test]
    fn test_first_marker_offset() {
        assert_eq!(first_marker_offset("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string()), 19);
        assert_eq!(first_marker_offset("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()), 23);
        assert_eq!(first_marker_offset("nppdvjthqldpwncqszvftbrmjlhg".to_string()), 23);
        assert_eq!(first_marker_offset("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()), 29);
        assert_eq!(first_marker_offset("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()), 26);
    }
}
