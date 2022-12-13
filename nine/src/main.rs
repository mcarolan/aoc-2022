use std::collections::HashSet;
use std::io::BufRead;
use std::{fs::File, io};

enum MotionDirection {
    Up,
    Down,
    Left,
    Right,
}

struct Motion {
    direction: MotionDirection,
    amount: u32
}

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
struct Position {
    x: i32,
    y: i32,
}

fn are_touching(head: &Position, tail: &Position) -> bool {
    head.x.abs_diff(tail.x) <= 1 && head.y.abs_diff(tail.y) <= 1
}

fn next_head_position(head: &Position, direction: &MotionDirection) -> Position {
    match direction {
        MotionDirection::Up =>
            Position {
                x: head.x,
                y: head.y - 1
            },
        MotionDirection::Down => 
            Position {
                x: head.x,
                y: head.y + 1
            },
        MotionDirection::Left => 
            Position {
                x: head.x - 1,
                y: head.y
            },
        MotionDirection::Right =>
            Position {
                x: head.x + 1,
                y: head.y
            },
    }
}

fn next_tail_position(head: &Position, tail: &Position) -> Position {
    if are_touching(head, tail) {
        Position {
            x: tail.x,
            y: tail.y,
        }
    } else if head.y == tail.y && head.x < tail.x {
        Position {
            x: tail.x - 1,
            y: tail.y,
        }
    } else if head.y == tail.y && head.x > tail.x {
        Position {
            x: tail.x + 1,
            y: tail.y,
        }
    } else if head.x == tail.x && head.y < tail.y {
        Position {
            x: tail.x,
            y: tail.y - 1,
        }
    } else if head.x == tail.x && head.y > tail.y {
        Position {
            x: tail.x,
            y: tail.y + 1,
        }
    } else if head.x > tail.x && head.y > tail.y {
        Position {
            x: tail.x + 1,
            y: tail.y + 1,
        }
    } else if head.x > tail.x && head.y < tail.y {
        Position {
            x: tail.x + 1,
            y: tail.y - 1,
        }
    } else if head.x < tail.x && head.y < tail.y {
        Position {
            x: tail.x - 1,
            y: tail.y - 1,
        }
    } else if head.x < tail.x && head.y > tail.y {
        Position {
            x: tail.x - 1,
            y: tail.y + 1,
        }
    } else {
        todo!()
    }
}

fn parse_line(line: String) -> Option<Motion> {
    let split = Vec::from_iter(line.split(' '));

    let amount_opt = split.get(1).and_then(|s| u32::from_str_radix(s, 10).ok());
    let direction_opt = match split.first() {
        Some(&"U") => Some(MotionDirection::Up),
        Some(&"D") => Some(MotionDirection::Down),
        Some(&"L") => Some(MotionDirection::Left),
        Some(&"R") => Some(MotionDirection::Right),
        _ => None
    };
    
    amount_opt.zip(direction_opt).map(|(amount, direction)| Motion { amount, direction })
}

fn count_tail_positions(motions: Vec<Motion>) -> i32 {
    let mut tail_positions: Vec<Position> = Vec::new();
    let mut head = Position { x: 0, y: 0 };
    let mut tail = Position { x: 0, y: 0};
    
    tail_positions.push(tail.clone());

    for motion in motions {
        for _i in 0..motion.amount {
            head = next_head_position(&head, &motion.direction).clone();
            tail = next_tail_position(&head, &tail).clone();
            tail_positions.push(tail.clone());
        }
    }

    let set : HashSet<Position> = HashSet::from_iter(tail_positions.into_iter());
    set.len() as i32
}

fn main() {
    let file = File::open("./input").unwrap();
    let lines = io::BufReader::new(file).lines();

    let strings = lines.flat_map(|l| l.ok());
    let motions = Vec::from_iter(strings).into_iter().flat_map(parse_line);

    let answer = count_tail_positions(Vec::from_iter(motions));

    println!("The answer is {}", answer);
}

#[cfg(test)]
mod tests {
    use crate::{are_touching, count_tail_positions, next_tail_position, Motion, Position, MotionDirection};

    #[test]
    fn test_are_touching() {
        assert!(are_touching(
            &Position { x: 0, y: 0 },
            &Position { x: 1, y: 0 }
        ));
        assert!(are_touching(
            &Position { x: 0, y: 0 },
            &Position { x: 0, y: -1 }
        ));
        assert!(!are_touching(
            &Position { x: 0, y: 0 },
            &Position { x: 0, y: -2 }
        ));
        assert!(are_touching(
            &Position { x: 0, y: 0 },
            &Position { x: 1, y: 1 }
        ));
        assert!(!are_touching(
            &Position { x: 0, y: 0 },
            &Position { x: 2, y: 1 }
        ));
        assert!(!are_touching(
            &Position { x: 0, y: 0 },
            &Position { x: 2, y: 2 }
        ));
    }

    #[test]
    fn test_next_tail_position_right() {
        let head = Position { x: 2, y: 0 };
        let tail = Position { x: 0, y: 0 };
        let expected = Position { x: 1, y: 0 };

        assert_eq!(next_tail_position(&head, &tail), expected);
    }

    #[test]
    fn test_next_tail_position_left() {
        let head = Position { x: -2, y: 0 };
        let tail = Position { x: 0, y: 0 };
        let expected = Position { x: -1, y: 0 };

        assert_eq!(next_tail_position(&head, &tail), expected);
    }

    #[test]
    fn test_next_tail_position_up() {
        let head = Position { x: 0, y: -2 };
        let tail = Position { x: 0, y: 0 };
        let expected = Position { x: 0, y: -1 };

        assert_eq!(next_tail_position(&head, &tail), expected);
    }

    #[test]
    fn test_next_tail_position_down() {
        let head = Position { x: 0, y: 2 };
        let tail = Position { x: 0, y: 0 };
        let expected = Position { x: 0, y: 1 };

        assert_eq!(next_tail_position(&head, &tail), expected);
    }

    #[test]
    fn test_next_tail_position_ne() {
        let head = Position { x: 2, y: -2 };
        let tail = Position { x: 0, y: 0 };
        let expected = Position { x: 1, y: -1 };

        assert_eq!(next_tail_position(&head, &tail), expected);
    }

    #[test]
    fn test_next_tail_position_se() {
        let head = Position { x: 2, y: 2 };
        let tail = Position { x: 0, y: 0 };
        let expected = Position { x: 1, y: 1 };

        assert_eq!(next_tail_position(&head, &tail), expected);
    }

    #[test]
    fn test_next_tail_position_sw() {
        let head = Position { x: -2, y: 2 };
        let tail = Position { x: 0, y: 0 };
        let expected = Position { x: -1, y: 1 };

        assert_eq!(next_tail_position(&head, &tail), expected);
    }

    #[test]
    fn test_next_tail_position_nw() {
        let head = Position { x: -2, y: -2 };
        let tail = Position { x: 0, y: 0 };
        let expected = Position { x: -1, y: -1 };

        assert_eq!(next_tail_position(&head, &tail), expected);
    }

    #[test]
    fn test_count_tail_positions() {
        let motions = vec![
            Motion { direction: MotionDirection::Right, amount: 4},
            Motion { direction: MotionDirection::Up, amount: 4},
            Motion { direction: MotionDirection::Left, amount: 3},
            Motion { direction: MotionDirection::Down, amount: 1},
            Motion { direction: MotionDirection::Right, amount: 4},
            Motion { direction: MotionDirection::Down, amount: 1},
            Motion { direction: MotionDirection::Left, amount: 5},
            Motion { direction: MotionDirection::Right, amount: 2},
        ];

        assert_eq!(count_tail_positions(motions), 13);
    }
}
