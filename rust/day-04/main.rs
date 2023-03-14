use std::fs;

fn parse_line(l: &str) -> ((usize, usize), (usize, usize)) {
    let (left, right) = l.split_once(',').unwrap();
    let (ls, le) = left.split_once('-').unwrap();
    let (rs, re) = right.split_once('-').unwrap();
    (
        (ls.parse::<usize>().unwrap(), le.parse::<usize>().unwrap()),
        (rs.parse::<usize>().unwrap(), re.parse::<usize>().unwrap())
    )
}

fn fully_contains(pair: ((usize, usize), (usize, usize))) -> bool {
    let (l, r) = pair;
    if l.0 <= r.0 && r.1 <= l.1 {
        true
    } else if r.0 <= l.0 && l.1 <= r.1 {
        true
    } else {
        false
    }
}

fn overlaps(pair: ((usize, usize), (usize, usize))) -> bool {
    let (l, r) = pair;
    if l.1 < r.0 || r.1 < l.0 { false } else { true }
}

fn main() {
    let input = fs::read_to_string("test_input.txt").unwrap();

    let result = input.lines()
                      .map(parse_line)
                      .map(fully_contains)
                      .filter(|b| *b)
                      .count();
    println!("{}", result);

    let result = input.lines()
                      .map(parse_line)
                      .map(overlaps)
                      .filter(|b| *b)
                      .count();
    println!("{}", result);
}