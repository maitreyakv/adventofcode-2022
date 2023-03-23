use std::fs;
use std::collections::HashSet;

fn knot_diff(knot_1: (isize, isize), knot_2: (isize, isize)) -> (isize, isize) {
    (knot_1.0 - knot_2.0, knot_1.1 - knot_2.1)
}

fn clamp(x: isize, lower: isize, upper: isize) -> isize {
    if x > upper {
        return upper
    } else if x < lower {
        return lower
    } else {
        return x
    }
}

struct Rope {
    knots: Vec<(isize, isize)>,
    tail_history: HashSet<(isize, isize)>
}

impl Rope {
    fn new(length: usize) -> Self {
        Rope {
            knots: vec![(0, 0); length],
            tail_history: HashSet::new()
        }
    }

    fn make_move(&mut self, step: (isize, isize)) {
        let head = self.knots.get_mut(0).unwrap();
        *head = (head.0 + step.0, head.1 + step.1);
        for i in 1..self.knots.len() {
            let knot_ahead = self.knots.get(i - 1).unwrap().clone();
            let knot_behind = self.knots.get(i).unwrap().clone();
            let diff = knot_diff(knot_ahead, knot_behind);
            if diff.0.abs() > 1 || diff.1.abs() > 1 {
                let knot_adjust = self.knots.get_mut(i).unwrap();
                (*knot_adjust).0 += clamp(diff.0, -1, 1);
                (*knot_adjust).1 += clamp(diff.1, -1, 1);
            }
        }
        self.tail_history.insert(self.knots.last().unwrap().clone());
    }

    fn simulate(&mut self, file: &str) {
        for line in file.lines() {
            let (direction, number) = line.split_once(' ').unwrap();
            let step = match direction {
                "R" => { (0, 1) },
                "L" => { (0, -1) },
                "U" => { (-1, 0) },
                "D" => { (1, 0) }
                _ => { panic!("Unrecognized direction!") }
            };
            for _ in 0..number.parse().expect("Unrecognized number!") {
                self.make_move(step)
            }
        }

        println!("number of spots visited: {}", self.tail_history.len());
    }
}

fn main() {
    let file = fs::read_to_string("test_input.txt").unwrap();
    
    let mut rope = Rope::new(2);
    rope.simulate(&file);

    let mut rope = Rope::new(10);
    rope.simulate(&file);
}
