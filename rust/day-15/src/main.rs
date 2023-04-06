use std::fs;
use std::ops::RangeInclusive;
use std::collections::HashSet;

type Coord = (isize, isize);

fn manhattan_distance(c1: &Coord, c2: &Coord) -> isize {
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
            dist: manhattan_distance(&sensor, &beacon)
        }
    }

    fn coverage_band(&self, row: isize) -> Option<RangeInclusive<isize>> {
        let dy = (self.sensor.1 - row).abs();
        if dy > self.dist {
            return None
        }
        let start = self.sensor.0 - (self.dist - dy);
        let end = self.sensor.0 + (self.dist - dy);
        Some(start..=end)
    }

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
    r1: &RangeInclusive<isize>,
    r2: &RangeInclusive<isize>
) -> Option<RangeInclusive<isize>> {
    if r1.contains(r2.start()) && r1.contains(r2.end()) {
        return Some(r1.clone())
    } else if r2.contains(r1.start()) && r2.contains(r1.end()) {
        return Some(r2.clone())
    } else if r1.contains(r2.start()) {
        return Some(*r1.start()..=*r2.end())
    } else if r1.contains(r2.end()) {
        return Some(*r2.start()..=*r1.end())
    }
    None
}

#[derive(Clone)]
struct RangeSet {
    ranges: HashSet<RangeInclusive<isize>>
}

impl RangeSet {
    fn new() -> Self {
        RangeSet {
            ranges:HashSet::new()
        }
    }

    fn find_overlapping(
        &self,
        other: &RangeInclusive<isize>
    ) -> Option<RangeInclusive<isize>> {
        for old in self.ranges.iter() {
            if combine_ranges(old, other).is_some() {
                return Some(old.clone());
            }
        }

        None
    }

    fn add(&mut self, new: RangeInclusive<isize>) {
        match self.find_overlapping(&new) {
            Some(matched) => {
                self.ranges.remove(&matched);
                let combined = combine_ranges(&matched, &new).unwrap();
                self.add(combined);
            },
            None => { 
                self.ranges.insert(new);
            }
        }
    
    }

    fn contains(&self, i: isize) -> bool {
        self.ranges.iter().any(|r| r.contains(&i))
    }

    fn size(&self) -> isize {
        self.ranges.iter().map(|r| r.end() - r.start() + 1).sum()
    }

    fn size_in_range(&self, lower: isize, upper: isize) -> isize {
        self.ranges
            .iter()
            .map(|r| {
                if *r.start() > upper {
                    return 0
                } else if *r.end() < lower {
                    return 0
                } else {
                    let start_clamped = r.start().clamp(&lower, &upper);
                    let end_clamped = r.end().clamp(&lower, &upper);
                    return end_clamped - start_clamped + 1
                }
            }).sum()
    }
}

fn get_blocked_in_row(pairs: &[Pair], row: isize) -> RangeSet {
    let mut blocked = RangeSet::new();
    for pair in pairs.iter() {
        match pair.coverage_band(row) {
            Some(band) => { blocked.add(band); },
            None => {}
        }
    }
    blocked
}

fn main() {
    let pairs = read_sensor_beacon_pairs();
    let sensors: HashSet<Coord> = pairs.iter().map(|p| p.sensor).collect();
    let beacons: HashSet<Coord> = pairs.iter().map(|p| p.beacon).collect();
    
    // Part 1
    let row = 2_000_000;
    let blocked = get_blocked_in_row(&pairs, row);
    let n_sensors_in_row = sensors.iter().filter(|&s| s.1 == row).count();
    let n_beacons_in_row = beacons.iter().filter(|&b| b.1 == row).count();
    let result = blocked.size() 
                    - n_sensors_in_row as isize 
                    - n_beacons_in_row as isize;
    println!("{result} spots blocked in row {row}");

    // Part 2
    let max_row = 4_000_000;
    for row in 0..=max_row {
        let blocked = get_blocked_in_row(&pairs, row);
        if blocked.size_in_range(0, max_row) < max_row + 1 {
            for col in 0..=max_row {
                if !blocked.contains(col) {
                    println!("tuning frequency is {}", 4_000_000 * col + row);
                    break
                }
            }
        }
    }
}
