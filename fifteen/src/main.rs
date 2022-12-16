use std::{fs, collections::HashSet};

use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    combinator::map,
    sequence::{preceded, tuple},
    IResult, multi::separated_list0,
};

#[derive(PartialEq, Debug, Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn distance_to(&self, other: &Point) -> u64 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(PartialEq, Debug, Clone)]
struct Entry {
    sensor: Point,
    nearest_beacon: Point
}

impl Entry {

    fn no_closer_at(&self, point: &Point) -> bool {
        let sensor_to_beacon = self.sensor.distance_to(&self.nearest_beacon);
        let point_to_sensor = point.distance_to(&self.sensor);

        point_to_sensor <= sensor_to_beacon
    }
    
}

fn count_cannot_contain_beacon(entries: &Vec<Entry>, y: i64) -> usize {
    let max_beacon_x = entries.iter().map(|e| e.nearest_beacon.x + (e.nearest_beacon.distance_to(&e.sensor) as i64)).max().unwrap();
    let min_beacon_x =  entries.iter().map(|e| e.nearest_beacon.x - (e.nearest_beacon.distance_to(&e.sensor) as i64)).min().unwrap();
    let mut set: HashSet<i64> = HashSet::new();

    for x in min_beacon_x..max_beacon_x + 1 {
        let p = Point { x, y };
        for e in entries {
            if e.no_closer_at(&p) {
                set.insert(x);
            }
        }
    }

    for e in entries {
        if e.nearest_beacon.y == y {
            set.remove(&e.nearest_beacon.x);
        }
    }

    set.len()
}

fn parse_entry(input: &str) -> IResult<&str, Entry> {
    let sensor_x_prefix = "Sensor at x=";
    let sensor_y_prefix = ", y=";

    let beacon_x_prefix = ": closest beacon is at x=";
    let beacon_y_prefix = ", y=";

    let sensor_point = map(
        tuple((
            preceded(tag(sensor_x_prefix), complete::i64),
            preceded(tag(sensor_y_prefix), complete::i64),
        )),
        |(x, y)| Point { x, y },
    );

    let beacon_point = map(
        tuple((
            preceded(tag(beacon_x_prefix), complete::i64),
            preceded(tag(beacon_y_prefix), complete::i64),
        )),
        |(x, y)| Point { x, y },
    );

    map(
        tuple((sensor_point, beacon_point)),
        |(sensor, nearest_beacon)| Entry {
            sensor,
            nearest_beacon,
        },
    )(input)
}

fn parse_entries(input: &str) -> IResult<&str, Vec<Entry>> {
    separated_list0(newline, parse_entry)(input)
}

fn main() {
    let input = fs::read_to_string("./input").unwrap();
    let (_, entries) = parse_entries(input.as_str()).ok().unwrap();

    
    println!("Parsed {} entries", entries.len());
    
    let answer = count_cannot_contain_beacon(&entries, 2000000);

    println!("The answer is {}", answer);
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_parse_entry() {
        let input = "Sensor at x=123637, y=2726215: closest beacon is at x=-886690, y=3416197";
        let expected = Entry {
            sensor: Point {
                x: 123637,
                y: 2726215,
            },
            nearest_beacon: Point {
                x: -886690,
                y: 3416197,
            },
        };

        let actual = parse_entry(input).unwrap();

        assert_eq!(actual, ("", expected));
    }


    #[test]
    fn test_example() {
        let input = fs::read_to_string("./input_example").unwrap();
        let (_, entries) = parse_entries(input.as_str()).ok().unwrap();
        assert_eq!(count_cannot_contain_beacon(&entries, 10), 26);
    }
}
