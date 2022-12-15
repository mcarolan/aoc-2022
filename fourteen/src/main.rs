use std::{collections::HashMap, borrow::BorrowMut, fs};

use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    combinator::map,
    multi::{separated_list1, separated_list0},
    sequence::{terminated, tuple},
    IResult,
};

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq)]
struct Path {
    points: Vec<Point>,
}

#[derive(Clone)]
enum Entity {
    Sand,
    Rock
}

impl Path {
    fn iter(&self) -> PathIter {
        PathIter {
            current: None,
            current_target: None,
            remaining_targets: self.points.clone(),
        }
    }
}

struct PathIter {
    current: Option<Point>,
    current_target: Option<Point>,
    remaining_targets: Vec<Point>,
}

impl Iterator for PathIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        fn delta(current: usize, target: usize) -> i64 {
            if current < target {
                1
            } else if current > target {
                -1
            } else {
                0
            }
        }

        fn next_point(from: Point, to: Point) -> Point {
            let dx: i64 = delta(from.x, to.x);
            let dy: i64 = delta(from.y, to.y);
            Point {
                x: ((from.x as i64) + dx) as usize,
                y: ((from.y as i64) + dy) as usize,
            }
        }

        match (self.current, self.current_target) {
            (None, None) => self.remaining_targets.clone().split_first().and_then(|(x, xs)| {
                self.current = Some(*x);
                self.current_target = xs.first().copied();
                self.remaining_targets = xs.to_vec();
                self.current
            }),
            (Some(current), Some(target)) if current == target => self.remaining_targets.clone().split_first().and_then(|(x, xs)| {
                self.current_target = xs.first().copied();
                self.current = self.current_target.map(|target| next_point(current, target));
                self.remaining_targets = xs.to_vec();
                self.current
            }),
            (Some(current), Some(target)) if current != target => {
                self.current = Some(next_point(current, target));
                self.current
            },
            (Some(_), None) => {
                self.current = None;
                self.current
            },
            _ => None
        }
    }
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    let point_tupled = tuple((
        terminated(complete::u64, complete::char(',')),
        complete::u64,
    ));
    map(point_tupled, |(x, y)| Point {
        x: x as usize,
        y: y as usize,
    })(input)
}

fn parse_path(input: &str) -> IResult<&str, Path> {
    let points = separated_list1(tag(" -> "), parse_point);
    map(points, |points| Path { points })(input)
}

fn parse_paths(input: &str) -> IResult<&str, Vec<Path>> {
    separated_list0(newline, parse_path)(input)
}

fn next_sand_position(current_sand_position: Point, map: &HashMap<Point, Entity>) -> Option<Point> {
    let below = Point { x: current_sand_position.x, y: current_sand_position.y + 1 };
    let left = Point { x: current_sand_position.x - 1, y: current_sand_position.y + 1  };
    let right = Point { x: current_sand_position.x + 1, y: current_sand_position.y + 1  };

    if map.get(&below).is_none() {
        Some(below)
    }
    else if map.get(&left).is_none() {
        Some(left)  
    } else if map.get(&right).is_none() {
        Some(right)
    } else {
        None
    }
}

fn sand_units_before_abyss(input_map: &HashMap<Point, Entity>) -> usize {
    let sand_start = Point { x: 500, y: 0 };
    let abyss_after = input_map.keys().map(|k| k.y).max().unwrap();
    let mut map: HashMap<Point, Entity> = HashMap::new();
    map.clone_from(&input_map);
    let mut current_sand: Point = sand_start;
    let mut sand_counter  = 0;

    while current_sand.y < abyss_after + 1  {
        let next_sand_opt = next_sand_position(current_sand, &map);

        match next_sand_opt {
            Some(next_sand) => {
                map.remove(&current_sand);
                current_sand = next_sand;
                map.insert(current_sand, Entity::Sand);
            },
            None => {
                sand_counter += 1;
                current_sand = sand_start;
            }
        }
    }

    sand_counter
}

fn build_map(paths: &Vec<Path>) -> HashMap<Point, Entity> {
    let mut res: HashMap<Point, Entity> = HashMap::new();

    for path in paths {
        for point in path.iter() {
            res.insert(point, Entity::Rock);
        }
    }

    res
}

fn main() {
    let input = fs::read_to_string("./input").unwrap();
    let paths = parse_paths(input.as_str()).unwrap();
    let map = build_map(&paths.1);
    let answer = sand_units_before_abyss(&map);

    println!("the answer is {}", answer);
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_parse_path() {
        let input = "498,4 -> 498,6 -> 496,6";
        let expected = Path {
            points: vec![
                Point { x: 498, y: 4 },
                Point { x: 498, y: 6 },
                Point { x: 496, y: 6 },
            ],
        };
        assert_eq!(parse_path(input).ok(), Some(("", expected)));
    }

    #[test]
    fn test_path_iterator1() {
        let path = Path {
            points: vec![
                Point { x: 498, y: 4 },
                Point { x: 498, y: 6 },
                Point { x: 496, y: 6 },
            ],
        };

        let expected = vec![
            Point { x: 498, y: 4 },
            Point { x: 498, y: 5 },
            Point { x: 498, y: 6 },
            Point { x: 497, y: 6 },
            Point { x: 496, y: 6 },
        ];

        assert_eq!(Vec::from_iter(path.iter()), expected);
    }


    // fn print_diff(v1: Vec<Point>, v2: Vec<Point>, counter: u32) {
    //     fn equal_sign(b: bool) -> &'static str {
    //         if b {
    //             "=="
    //         } else {
    //             "!="
    //         }
    //     }

    //     match (v1.split_first(), v2.split_first()) {
    //         (Some((h1, t1)), Some((h2, t2))) => {
    //             println!(
    //                 "{}: {:?}     {:?}     {}",
    //                 counter,
    //                 h1,
    //                 h2,
    //                 equal_sign(h1 == h2)
    //             );
    //             print_diff(t1.to_vec(), t2.to_vec(), counter + 1);
    //         }
    //         (None, Some((h2, t2))) => {
    //             println!("{}: <empty>     {:?}      !=", counter, h2);
    //             print_diff(Vec::new(), t2.to_vec(), counter + 1);
    //         }
    //         (Some((h1, t1)), None) => {
    //             println!("{}: {:?}     <empty>      !=", counter, h1);
    //             print_diff(t1.to_vec(), Vec::new(), counter + 1);
    //         }
    //         (None, None) => {
    //             println!("done. {} items.", counter);
    //         }
    //     }
    // }

    #[test]
    fn test_path_iterator2() {
        let path = Path {
            points: vec![
                Point { x: 503, y: 4 },
                Point { x: 502, y: 4 },
                Point { x: 502, y: 9 },
                Point { x: 494, y: 9 },
            ],
        };

        let expected = vec![
            Point { x: 503, y: 4 },
            Point { x: 502, y: 4 },
            Point { x: 502, y: 5 },
            Point { x: 502, y: 6 },
            Point { x: 502, y: 7 },
            Point { x: 502, y: 8 },
            Point { x: 502, y: 9 },
            Point { x: 501, y: 9 },
            Point { x: 500, y: 9 },
            Point { x: 499, y: 9 },
            Point { x: 498, y: 9 },
            Point { x: 497, y: 9 },
            Point { x: 496, y: 9 },
            Point { x: 495, y: 9 },
            Point { x: 494, y: 9 },
        ];

        let actual = Vec::from_iter(path.iter());
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_next_sand_position_air_below() {
        let current_sand_position = Point { x: 10, y: 1};
        let map: HashMap<Point, Entity> = HashMap::new();
        let expected_sand_position = Some(Point { x: 10, y: 2});
        assert_eq!(next_sand_position(current_sand_position, &map), expected_sand_position);
    }

    #[test]
    fn test_next_sand_position_rock_below_nothing_left() {
        let current_sand_position = Point { x: 10, y: 1};
        let below = Point { x: 10, y: 2};
        let map: HashMap<Point, Entity> = HashMap::from([(below, Entity::Rock)]);
        let expected_sand_position = Some(Point { x: 9, y: 2});
        assert_eq!(next_sand_position(current_sand_position, &map), expected_sand_position);
    }

    #[test]
    fn test_next_sand_position_rock_below_something_left() {
        let current_sand_position = Point { x: 10, y: 1};
        let below = Point { x: 10, y: 2};
        let left = Point { x: 9, y: 2 };
        let map: HashMap<Point, Entity> = HashMap::from([(below, Entity::Rock), (left, Entity::Sand)]);
        let expected_sand_position = Some(Point { x: 11, y: 2});
        assert_eq!(next_sand_position(current_sand_position, &map), expected_sand_position);
    }

    #[test]
    fn test_example() {
        let input = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9";
        let paths = parse_paths(input).unwrap();
        let map = build_map(&paths.1);
        assert_eq!(sand_units_before_abyss(&map), 24);
    }
}
