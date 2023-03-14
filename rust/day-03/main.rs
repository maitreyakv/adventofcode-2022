use std::fs::File;
use std::io::{ BufRead, BufReader, Error, Read };
use std::collections::HashSet;
use std::iter::FromIterator;

fn priority(c: char) -> u32 {
    let p = c.to_digit(36).expect("error invalid character") - 9;
    if c.is_uppercase() {p + 26} else {p}    
}

fn get_common(line: Result<String, Error>) -> char {
    let l = line.unwrap();
    let n = l.len() / 2;

    let left: HashSet<char> = HashSet::from_iter(l.chars().take(n));
    let right: HashSet<char> = HashSet::from_iter(l.chars().skip(n).take(n));

    let common = left.intersection(&right)
                     .nth(0)
                     .expect("error no common item");
    *common
}

fn get_badge_values<R>(buf: BufReader<R>) -> u32
where R: Read {
    let mut iter = buf.lines();
    let mut sum: u32 = 0;
    loop {
        let r1 = iter.next();
        let r2 = iter.next();
        let r3 = iter.next();

        if r1.is_none() | r2.is_none() | r3.is_none() {
            return sum;
        }

        let (r1, r2, r3) = (r1.unwrap(), r2.unwrap(), r3.unwrap());

        let r1: HashSet<char> = HashSet::from_iter(r1.unwrap().chars());
        let r2: HashSet<char> = HashSet::from_iter(r2.unwrap().chars());
        let r3: HashSet<char> = HashSet::from_iter(r3.unwrap().chars());
        
        let common = r1.intersection(&r2)
                       .map(|x| x.clone())
                       .collect::<HashSet<char>>()
                       .intersection(&r3)
                       .nth(0)
                       .expect("error no common badge")
                       .clone();

        sum += priority(common);
    }
}

fn main() {
    let file = File::open("./test_input.txt".to_string()).unwrap();
    let result: u32 = BufReader::new(file)
                        .lines()
                        .map(get_common)
                        .map(priority)
                        .sum();
    println!("{}", result);

    let file = File::open("./test_input.txt".to_string()).unwrap();
    let result: u32 = get_badge_values(BufReader::new(file));
    println!("{}", result);
}