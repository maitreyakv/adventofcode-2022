use std::fs;

struct Crates {
    stacks: Vec<Vec<char>>
}

impl Crates {
    fn puzzle_input() -> Self {
        Crates {
            stacks: vec![
                String::from("WBDNCFJ").chars().collect(),
                String::from("PZVQLST").chars().collect(),
                String::from("PZBGJT").chars().collect(),
                String::from("DTLJZBHC").chars().collect(),
                String::from("GVBJS").chars().collect(),
                String::from("PSQ").chars().collect(),
                String::from("BVDFLMPN").chars().collect(),
                String::from("PSMFBDLR").chars().collect(),
                String::from("VDTR").chars().collect(),
            ]
        }
    }

    fn apply_move_9000(&mut self, move_: Move) {
        for _ in 0..move_.num {
            let b = self.stacks[move_.start].pop().unwrap();
            self.stacks[move_.end].push(b);
        }
    }

    fn apply_move_9001(&mut self, move_: Move) {
        let mut crane: Vec<char> = Vec::new();
        for _ in 0..move_.num {
            let b = self.stacks[move_.start].pop().unwrap();
            crane.push(b);
        }
        for _ in 0..move_.num {
            let b = crane.pop().unwrap();
            self.stacks[move_.end].push(b);
        }
    }

    fn print_top_crates(&self) {
        for stack in &self.stacks {
            print!("{}", stack.last().unwrap());
        }
        println!("");
    }
}

#[derive(Clone)]
struct Move {
    num: usize,
    start: usize,
    end: usize
}

impl Move {
    fn parse(line: &str) -> Self {
        let line: Vec<&str> = line.split_ascii_whitespace().collect();
        Move {
            num: line[1].parse::<usize>().unwrap(),
            start: line[3].parse::<usize>().unwrap() - 1,
            end: line[5].parse::<usize>().unwrap() - 1,
        }
    }
}

fn simulate_inplace(crates: &mut Crates, moves: Vec<Move>, multi: bool) {
    for move_ in moves {
        if multi {
            crates.apply_move_9001(move_);
        } else {
            crates.apply_move_9000(move_);
        }
    }
}

fn main() {
    let moves: Vec<Move> = fs::read_to_string("test_input.txt")
                            .unwrap()
                            .lines()
                            .map(|l| Move::parse(l))
                            .collect();
    
    let mut crates = Crates::puzzle_input();
    simulate_inplace(&mut crates, moves.clone(), false);
    crates.print_top_crates();

    let mut crates = Crates::puzzle_input();
    simulate_inplace(&mut crates, moves.clone(), true);
    crates.print_top_crates();
}
