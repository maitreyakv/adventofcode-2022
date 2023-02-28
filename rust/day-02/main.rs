use std::fs::File;
use std::io::{ BufRead, BufReader };

fn main() {
    let file = File::open("./test_input.txt".to_string()).unwrap();
    let lines = BufReader::new(file).lines();

    let mut total_score_part_1 = 0;
    let mut total_score_part_2 = 0;

    for line in lines {
        let l = line.unwrap();
        let pick_op = l.chars().nth(0).expect("error with parsing line");
        let pick_me = l.chars().nth(2).expect("error with parsing line");

        let score = match (pick_op, pick_me) {
            ('A', 'X') => Ok(4),
            ('A', 'Y') => Ok(8),
            ('A', 'Z') => Ok(3),
            ('B', 'X') => Ok(1),
            ('B', 'Y') => Ok(5),
            ('B', 'Z') => Ok(9),
            ('C', 'X') => Ok(7),
            ('C', 'Y') => Ok(2),
            ('C', 'Z') => Ok(6),
            (_, _) => Err(0)
        };
        total_score_part_1 += score.expect("error with unrecognized move");

        let score = match (pick_op, pick_me) {
            ('A', 'X') => Ok(3),
            ('A', 'Y') => Ok(4),
            ('A', 'Z') => Ok(8),
            ('B', 'X') => Ok(1),
            ('B', 'Y') => Ok(5),
            ('B', 'Z') => Ok(9),
            ('C', 'X') => Ok(2),
            ('C', 'Y') => Ok(6),
            ('C', 'Z') => Ok(7),
            (_, _) => Err(0)
        };
        total_score_part_2 += score.expect("error with unrecognized move");
    }

    println!("{}", total_score_part_1);
    println!("{}", total_score_part_2);
}