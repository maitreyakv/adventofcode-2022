use std::fs;
use std::ops::RangeInclusive;

type Coord = (isize, isize);

fn distance_L1(c1: &Coord, c2: &Coord) -> isize {
    (c1.0 - c2.0).abs() + (c1.1 - c2.1).abs()
}

struct Pair {
    sensor: Coord,
    beacon: Coord,
    dist: isize
}

fn parse_coordinate(coord_str: &str) -> Coord {
    let (x_str, y_str) = coord_str.split_once(", ").unwrap();
    (
        x_str.split_once("=").unwrap().1.parse::<isize>().unwrap(),
        y_str.split_once("=").unwrap().1.parse::<isize>().unwrap()
    )
}

impl Pair {
    fn new(sensor: Coord, beacon: Coord) -> Self {
        Self {
            sensor: sensor,
            beacon: beacon,
            dist: distance_L1(&sensor, &beacon)
        }
    }

    // fn coverage_band(&self, row: isize) -> RangeInclusive<isize> {
    //     let dy = (self.sensor.1 - row).abs();
    //     let start = self.sensor.0 - (self.dist - dy);
    //     let end = self.sensor.0 + (self.dist - dy);
    //     start..=end
    // }

    fn from_string(line: &str) -> Self {
        let (mut sensor_str, mut beacon_str) = line.split_once(":").unwrap();
        sensor_str = sensor_str.strip_prefix("Sensor at ").unwrap();
        beacon_str = beacon_str.strip_prefix(" closest beacon is at ").unwrap();

        let sensor = parse_coordinate(sensor_str);
        let beacon = parse_coordinate(beacon_str);

        Self::new(sensor, beacon)
    }
}

fn read_sensor_beacon_pairs() -> Vec<Pair> {
    fs::read_to_string("test_input.txt")
            .unwrap()
            .lines()
            .map(|s| Pair::from_string(s))
            .collect()
}

fn combine_ranges(
    r1: RangeInclusive<isize>,
    r2: RangeInclusive<isize>
) -> Option<RangeInclusive<isize>> {

    
    None
}

struct RangeSet {
    ranges: Vec<RangeInclusive<isize>>
}

fn main() {
    let pairs = read_sensor_beacon_pairs();
    
}
