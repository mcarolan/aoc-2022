use std::fs;

use nom::{
    branch::alt,
    character::complete::{self, newline, space0},
    combinator::map,
    multi::{separated_list0, many0, many1},
    sequence::{preceded, terminated, tuple},
    IResult,
};

#[derive(PartialEq, Debug, Clone)]
enum Element {
    Number(i32),
    List(Vec<Element>),
}

fn parse_pairs(input: &str) -> IResult<&str, Vec<(Vec<Element>, Vec<Element>)>> {
    separated_list0(
        complete::char('\n'),
        tuple((
            terminated(parse_element, newline),
            terminated(parse_element, newline),
        )),
    )(input)
}

fn parse_packets(input: &str) -> IResult<&str, Vec<Vec<Element>>> {
    many0(terminated(parse_element, many1(newline)))(input)
}

fn parse_element(input: &str) -> IResult<&str, Vec<Element>> {
    let element = alt((
        map(complete::i32, Element::Number),
        map(parse_element, Element::List),
    ));
    let sep = preceded(space0, terminated(complete::char(','), space0));
    let elements = separated_list0(sep, element);
    preceded(
        complete::char('['),
        terminated(elements, complete::char(']')),
    )(input)
}

#[derive(PartialEq, Debug)]
enum Decision {
    True,
    False,
    None,
}

fn in_order(pair1: &[Element], pair2: &[Element]) -> Decision {
    println!("pair 1 {:?}", pair1);
    println!("pair 2 {:?}", pair2);

    match (pair1.split_first(), pair2.split_first()) {
        (Some((Element::Number(x), _tail1)), Some((Element::Number(y), _tail2))) if x < y => {
            println!("in order as left side is smaller");
            Decision::True
        }
        (Some((Element::Number(x), _tail1)), Some((Element::Number(y), _tail2))) if y < x => {
            println!("not in order as right side is smaller");
            Decision::False
        }
        (Some((Element::Number(x), tail1)), Some((Element::Number(y), tail2))) if x == y => {
            println!("checking further as {} == {}", x, y);
            in_order(tail1, tail2)
        }
        (Some((Element::List(xs), tail1)), Some((Element::List(ys), tail2))) => {
            let list_decision = in_order(xs, ys);
            println!(
                "in order lists {:?} and {:?}, comparing next elements",
                xs, ys
            );

            match list_decision {
                Decision::None => in_order(tail1, tail2),
                other => other,
            }
        }
        (Some((Element::List(xs), _tail1)), Some((Element::Number(n), tail2))) => {
            println!("comparing {:?} and number {}, promoting", xs, n);
            let mut promoted = vec![Element::List(vec![Element::Number(*n)])];
            for elem in tail2 {
                promoted.push(elem.clone());
            }

            in_order(pair1, promoted.as_slice())
        }
        (Some((Element::Number(n), tail1)), Some((Element::List(xs), _tail2))) => {
            println!("comparing number {} and list {:?}, promoting", n, xs);
            let mut promoted = vec![Element::List(vec![Element::Number(*n)])];
            for elem in tail1 {
                promoted.push(elem.clone());
            }

            in_order(promoted.as_slice(), pair2)
        }
        (None, None) => Decision::None,
        (None, _) => {
            println!("Left side ran out of items, inputs are in order");
            Decision::True
        }
        (Some(_), None) => {
            println!("Right side ran out of items, inputs not in order");
            Decision::False
        }
        input => panic!("woah! {:?}", input),
    }
}

fn solution(pairs: &Vec<(Vec<Element>, Vec<Element>)>) -> usize {
    let mut already_in_order: Vec<usize> = Vec::new();

    for ((pair1, pair2), i) in pairs.iter().zip(1..) {
        if in_order(pair1, pair2) == Decision::True {
            already_in_order.push(i);
        }
    }
    already_in_order.into_iter().sum()
}

fn marker1() -> Vec<Element> {
    vec![Element::List(vec![Element::Number(2)])]
}

fn marker2() -> Vec<Element> {
    vec![Element::List(vec![Element::Number(6)])]
}

fn distress_signal(input_packets: &Vec<Vec<Element>>) -> usize {
    let mut packets = input_packets.clone();

    packets.push(marker1());
    packets.push(marker2());

    packets.sort_by(|i, j| match in_order(i, j) {
        Decision::True => std::cmp::Ordering::Less,
        Decision::False => std::cmp::Ordering::Greater,
        Decision::None => std::cmp::Ordering::Equal
    });

    let mut first_marker_index = 0;
    let mut second_marker_index = 0;

    for (packet, i) in packets.iter().zip(1..) {
        print!("{}: {:?}", i, packet);

        if *packet == marker1() {
            first_marker_index = i;
            print!(" (first marker)");
        }

        if *packet == marker2() {
            second_marker_index = i;
            print!(" (second marker)");
        }
        println!()
    }

    first_marker_index * second_marker_index
}

fn main() {
    let input = fs::read_to_string("./input").unwrap();
    let (_, packets) = parse_packets(input.as_str()).unwrap();

    println!("Parsed {} packets,", packets.len());

    let answer: usize = distress_signal(&packets);
    println!("The answer is {}", answer);
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{in_order, parse_element, parse_pairs, solution, Decision, Element, parse_packets, distress_signal};

    #[test]
    fn test_example() {
        let input = fs::read_to_string("./input_example").unwrap();
        let (_, parsed) = parse_pairs(input.as_str()).unwrap();
        let result = solution(&parsed);

        assert_eq!(result, 13);
    }

    #[test]
    fn test_distress_example() {
        let input = fs::read_to_string("./input_example").unwrap();
        let (_, parsed) = parse_packets(input.as_str()).unwrap();
        assert_eq!(distress_signal(&parsed), 140);
    }

    #[test]
    fn test_parse_list_element() {
        assert_eq!(
            parse_element("[1,1,3,1,1]").ok(),
            Some((
                "",
                vec![
                    Element::Number(1),
                    Element::Number(1),
                    Element::Number(3),
                    Element::Number(1),
                    Element::Number(1)
                ]
            ))
        );

        assert_eq!(
            parse_element("[[4,4],4,4]").ok(),
            Some((
                "",
                vec![
                    Element::List(vec![Element::Number(4), Element::Number(4)]),
                    Element::Number(4),
                    Element::Number(4)
                ]
            ))
        )
    }

    #[test]
    fn test_pair_1() {
        let pair1 = parse_element("[1,1,3,1,1]").unwrap().1;
        let pair2 = parse_element("[1,1,5,1,1]").unwrap().1;
        assert_eq!(in_order(&pair1, &pair2), Decision::True);
    }

    #[test]
    fn test_pair_2() {
        let pair1 = parse_element("[[1],[2,3,4]]").unwrap().1;
        let pair2 = parse_element("[[1],4]").unwrap().1;
        assert_eq!(in_order(&pair1, &pair2), Decision::True);
    }

    #[test]
    fn test_pair_3() {
        let pair1 = parse_element("[9]").unwrap().1;
        let pair2 = parse_element("[[8,7,6]]").unwrap().1;
        assert_eq!(in_order(&pair1, &pair2), Decision::False);
    }

    #[test]
    fn test_pair_4() {
        let pair1 = parse_element("[[4,4],4,4]").unwrap().1;
        let pair2 = parse_element("[[4,4],4,4,4]").unwrap().1;
        assert_eq!(in_order(&pair1, &pair2), Decision::True);
    }

    #[test]
    fn test_pair_5() {
        let pair1 = parse_element("[7,7,7,7]").unwrap().1;
        let pair2 = parse_element("[7,7,7]").unwrap().1;
        assert_eq!(in_order(&pair1, &pair2), Decision::False);
    }

    #[test]
    fn test_pair_6() {
        let pair1 = parse_element("[]").unwrap().1;
        let pair2 = parse_element("[3]").unwrap().1;
        assert_eq!(in_order(&pair1, &pair2), Decision::True);
    }

    #[test]
    fn test_pair_7() {
        let pair1 = parse_element("[[[]]]").unwrap().1;
        let pair2 = parse_element("[[]]").unwrap().1;
        assert_eq!(in_order(&pair1, &pair2), Decision::False);
    }

    #[test]
    fn test_pair_8() {
        let pair1 = parse_element("[1,[2,[3,[4,[5,6,7]]]],8,9]").unwrap().1;
        let pair2 = parse_element("[1,[2,[3,[4,[5,6,0]]]],8,9]").unwrap().1;
        assert_eq!(in_order(&pair1, &pair2), Decision::False);
    }
}
