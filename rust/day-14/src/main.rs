use std::{ fs, cmp };
use std::collections::HashSet;

type Coord = (usize, usize);

fn parse_coordinate(coord_string: &str) -> Coord {
    let coord = coord_string.split_once(",").unwrap();
    (coord.0.parse().unwrap(), coord.1.parse().unwrap())
}

fn parse_line(line: &str) -> Vec<Coord> {
    line.split(" -> ")
        .map(parse_coordinate)
        .collect()
}

fn expand_segment(start: Coord, end: Coord) -> Vec<Coord> {
    if start.0 == end.0 {
        let y1 = cmp::min(start.1, end.1);
        let y2 = cmp::max(start.1, end.1);
        return (y1..=y2).map(|y| (start.0, y)).collect()
    } else if start.1 == end.1 {
        let x1 = cmp::min(start.0, end.0);
        let x2 = cmp::max(start.0, end.0);
        return (x1..=x2).map(|x| (x, start.1)).collect()
    } else {
        panic!("error, segment is not horizontal or vertical!");
    }
}
 
fn read_initial_cave() -> HashSet<Coord> {
    let mut cave = HashSet::new();

    let file = fs::read_to_string("test_input.txt").unwrap();
    for line in file.lines() {
        let path = parse_line(line);
        for segment in path.windows(2) {
            let start = segment[0];
            let end = segment[1];
            cave.extend(expand_segment(start, end).into_iter());
        }
    }

    cave
}

fn simulate(mut cave: HashSet<Coord>, bottomless: bool) -> Option<usize> {
    let floor = cave.iter().map(|&c| c.1).max().unwrap() + 2;
    
    for i in 0.. {
        let mut sand = (500, 0);

        if cave.contains(&sand) {
            return Some(i)
        }

        loop {
            if sand.1 == floor - 1 {
                if bottomless {
                    return Some(i)
                } else {
                    cave.insert(sand);
                    break
                }
            }

            if !cave.contains(&(sand.0, sand.1 + 1)) {
                sand.1 += 1;
                continue
            } else if !cave.contains(&(sand.0 - 1, sand.1 + 1)) {
                sand.0 -= 1;
                sand.1 += 1;
                continue
            } else if !cave.contains(&(sand.0 + 1, sand.1 + 1)) {
                sand.0 += 1;
                sand.1 += 1;
                continue
            } else {
                cave.insert(sand);
                break
            }
        }
    }
    None
}

fn main() {
    let cave = read_initial_cave();
    let result = simulate(cave, true).unwrap();
    println!("{result} units of sand came to rest");

    let cave = read_initial_cave();
    let result = simulate(cave, false).unwrap();
    println!("{result} units of sand came to rest");
}
