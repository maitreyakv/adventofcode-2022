use std::fs;
use std::collections::HashSet;

type Coord = (isize, isize);
type Pair = (Coord, Coord);

fn parse_coordinate(coord_str: &str) -> Coord {
    let (x_str, y_str) = coord_str.split_once(", ").unwrap();
    (
        x_str.split_once("=").unwrap().1.parse::<isize>().unwrap(),
        y_str.split_once("=").unwrap().1.parse::<isize>().unwrap()
    )
}

fn parse_line(line: &str) -> Pair {
    let (mut sensor_str, mut beacon_str) = line.split_once(":").unwrap();
    sensor_str = sensor_str.strip_prefix("Sensor at ").unwrap();
    beacon_str = beacon_str.strip_prefix(" closest beacon is at ").unwrap();

    (parse_coordinate(sensor_str), parse_coordinate(beacon_str))
} 

fn read_sensor_beacon_pairs() -> Vec<Pair> {
    fs::read_to_string("test_input.txt")
            .unwrap()
            .lines()
            .map(parse_line)
            .collect()
}

fn distance_L1(pair: &Pair) -> isize {
    (pair.0.0 - pair.1.0).abs() + (pair.0.1 - pair.1.1).abs()
}

fn blocked_by_sensor_in_row(
    pair: &Pair,
    row: isize
) -> impl Iterator<Item=Coord> {
    let radius = distance_L1(pair);
    let sensor = pair.0;
    let beacon = pair.0;
    let dy = (row - sensor.1).abs();
    let left = sensor.0 - (radius - dy);
    let right = sensor.0 + (radius - dy);
    (left..=right).map(move |x| (x, row))
}

fn is_blocked_by_sensor(pair: &Pair, coord: Coord) -> bool {
    distance_L1(&(pair.0, coord)) <= distance_L1(pair)
}

fn main() {
    let pairs = read_sensor_beacon_pairs();
    
    let mut blocked = HashSet::new();
    for pair in pairs.iter() {
        blocked.extend(blocked_by_sensor_in_row(pair, 2000000));
    }
    for pair in pairs.iter() {
        blocked.remove(&pair.0);
        blocked.remove(&pair.1);
    }
    let result = blocked.len();
    println!("Number of blocked locations in row: {result}");

}
