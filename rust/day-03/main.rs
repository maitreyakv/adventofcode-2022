use std::fs::File;
use std::io::{ BufRead, BufReader, Error };
use std::collections::HashSet;
use std::iter::FromIterator;

fn priority(c: char) -> u32 {
    let p = c.to_digit(36).expect("error invalid character") - 10;
    if c.is_uppercase() {p + 26} else {p}    
}

fn solve_line(line: Result<String, Error>) -> u32 {
    let l = line.unwrap();
    let n = l.len() / 2;

    let left: HashSet<char> = HashSet::from_iter(l.chars().take(n));
    let right: HashSet<char> = HashSet::from_iter(l.chars().skip(n).take(n));

    let common = left
        .intersection(&right)
        .nth(0)
        .expect("error no common item");

    priority(*common)
}

fn main() {
    let file = File::open("./test_input.txt".to_string()).unwrap();
    let lines = BufReader::new(file).lines();

    let result: u32 = lines.map(solve_line).sum();
    println!("{}", result);
}