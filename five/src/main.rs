use std::collections::HashMap;
use std::io::BufRead;
use std::{fs::File, io};

#[derive(Debug, PartialEq)]
struct Move {
    quantity: u32,
    source: u32,
    destination: u32,
}

fn parse_u32(s: &&str) -> Option<u32> {
    u32::from_str_radix(s, 10).ok()
}

fn parse_move(str: String) -> Option<Move> {
    let split = Vec::from_iter(str.split(' '));

    if split.get(0) == Some(&"move") {
        let quantity = split.get(1).and_then(parse_u32);
        let source = split.get(3).and_then(parse_u32);
        let destination = split.get(5).and_then(parse_u32);

        quantity
            .zip(source.zip(destination))
            .map(|(quantity, (source, destination))| Move {
                quantity,
                source,
                destination,
            })
    } else {
        None
    }
}

fn main() {
    /*
    [G]         [P]         [M]    
    [V]     [M] [W] [S]     [Q]    
    [N]     [N] [G] [H]     [T] [F]
    [J]     [W] [V] [Q] [W] [F] [P]
[C] [H]     [T] [T] [G] [B] [Z] [B]
[S] [W] [S] [L] [F] [B] [P] [C] [H]
[G] [M] [Q] [S] [Z] [T] [J] [D] [S]
[B] [T] [M] [B] [J] [C] [T] [G] [N]
 1   2   3   4   5   6   7   8   9 
    */

    let mut stacks: HashMap<u32, Vec<char>> = HashMap::new();


    stacks.insert(1, vec!['B', 'G', 'S', 'C']);
    stacks.insert(2, vec!['T', 'M', 'W', 'H', 'J', 'N', 'V', 'G']);
    stacks.insert(3, vec!['M', 'Q', 'S']);
    stacks.insert(4, vec!['B', 'S', 'L',  'T', 'W', 'N', 'M']);
    stacks.insert(5, vec!['J', 'Z', 'F', 'T', 'V', 'G',  'W', 'P']);
    stacks.insert(6, vec!['C', 'T', 'B', 'G', 'Q', 'H',  'S']);
    stacks.insert(7, vec!['T', 'J', 'P', 'B', 'W']);
    stacks.insert(8, vec!['G', 'D', 'C', 'Z', 'F', 'T', 'Q', 'M']);
    stacks.insert(9, vec!['N', 'S', 'H', 'B', 'P', 'F']);

    let file = File::open("./input").unwrap();
    let lines = io::BufReader::new(file).lines();

    let strings = lines.flat_map(|l| l.ok());
    let instructions = strings.into_iter().flat_map(parse_move);

    for instruction in instructions {
        let mut intermediate: Vec<char> = Vec::new();
        for _i in 0..instruction.quantity {
            let value = stacks.get_mut(&instruction.source).unwrap().pop().unwrap();
            intermediate.push(value);
        }

        let destination = stacks.get_mut(&instruction.destination).unwrap();
        for _i in 0..instruction.quantity {
            destination.push(intermediate.pop().unwrap());
        }
    }
    
    for i in 1..10 {
        let value = stacks.get_mut(&i).unwrap().pop().unwrap();
        print!("{}", value);
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse_move, Move};

    #[test]
    fn test_parse_move() {
        let input = "move 12 from 5 to 8".to_string();
        assert_eq!(
            parse_move(input),
            Some(Move {
                quantity: 12,
                source: 5,
                destination: 8
            })
        );
        let input2 = "    [G]         [P]         [M]    ".to_string();
        assert_eq!(parse_move(input2), None);
    }
}
