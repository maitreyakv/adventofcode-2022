use std::fs;

type Coord = (isize, isize);

enum Move {
    Left,
    Right,
    Up,
    Down
}

enum Collision {
    Wall,
    Rock
}

const CHAMBER_WIDTH_MINUS_1: isize = 6;

#[derive(Clone)]
struct Rock {
    coords: Vec<Coord>
}

impl Rock {
    fn new(rock_type: u8, bottom: isize) -> Self {
        let mut rock = match rock_type {
            0 => {
                Self {
                    coords: vec![(0, 0), (1, 0), (2, 0), (3, 0)]
                }
            },
            1 => {
                Self {
                    coords: vec![(0, 1), (1, 2), (1, 1), (1, 0), (2, 1)]
                }
            },
            2 => {
                Self {
                    coords: vec![(0, 0), (1, 0), (2, 2), (2, 1), (2, 0)]
                }
            },
            3 => {
                Self {
                    coords: vec![(0, 3), (0, 2), (0, 1), (0, 0)]
                }
            },
            4 => {
                Self {
                    coords: vec![(1, 0), (0, 0), (1, 1), (0, 1)]
                }
            },
            _ => { panic!("unrecognized rock type!") }
        };

        for _ in 0..2 {
            rock.apply_move(&Move::Right);
        }
        for _ in 0..bottom + 4 {
            rock.apply_move(&Move::Up, );
        }

        rock
    }

    fn apply_move(&mut self, mv: &Move) {
        match mv {
            Move::Left => {
                self.coords.iter_mut().for_each(|c| c.0 -= 1)
            },
            Move::Right => {
                self.coords.iter_mut().for_each(|c| c.0 += 1)
            },
            Move::Up => {
                self.coords.iter_mut().for_each(|c| c.1 += 1)
            },
            Move::Down => {
                self.coords.iter_mut().for_each(|c| c.1 -= 1)
            }
        }
    }

    fn detect_collision(&self, fallen_rocks: &Vec<Rock>) -> Option<Collision> {
        let left_edge = self.coords.iter().map(|c| c.0).min().unwrap();
        let right_edge = self.coords.iter().map(|c| c.0).max().unwrap();
        let bottom_edge = self.coords.iter().map(|c| c.1).min().unwrap();
        if left_edge < 0 
            || right_edge > CHAMBER_WIDTH_MINUS_1
            || bottom_edge < 0
        {
            return Some(Collision::Wall)
        }

        let is_rock_collision = fallen_rocks.iter()
                                    .flat_map(|r| r.coords.iter())
                                    .any(|c1| {
                                        self.coords.iter().any(|c2| c1 == c2)
                                    });
        if is_rock_collision {
            return Some(Collision::Rock)
        }
        
        None
    }

    fn attempt_move(
        &mut self,
        mv: &Move,
        fallen_rocks: &Vec<Rock>
    ) -> Option<Collision> {
        let mut rock_after_move = self.clone();
        rock_after_move.apply_move(mv);

        match rock_after_move.detect_collision(fallen_rocks) {
            None => {
                *self = rock_after_move;
                None
            },
            collision => collision
        }
    }
}

fn read_input() -> Vec<Move> {
    fs::read_to_string("test_input.txt")
        .unwrap()
        .trim()
        .chars()
        .map(|c| {
            match c {
                '<' => Move::Left,
                '>' => Move::Right,
                _ => panic!("unrecognized move in input sequence")
            }
        })
        .collect()
}

fn simulate(max_fallen_rocks: usize) -> isize {
    let mut fallen_rocks: Vec<Rock> = Vec::new();

    let mut rock = Rock::new(0, -1);
    let mut next_rock_type = 1;

    let mut peak = -1;

    for jet_move in read_input().iter().cycle() {
        rock.attempt_move(jet_move, &fallen_rocks);
        let collision = rock.attempt_move(&Move::Down, &fallen_rocks);

        if collision.is_some() {
            fallen_rocks.push(rock);
            peak = fallen_rocks.iter()
                               .flat_map(|r| r.coords.iter())
                               .map(|&c| c.1)
                               .max()
                               .unwrap();
            rock = Rock::new(next_rock_type, peak);
            next_rock_type = (next_rock_type + 1) % 5;
        }

        if fallen_rocks.len() == max_fallen_rocks {
            break
        }
    }

    peak
}

fn main() {
    let result = simulate(2022);
    println!("tower will be {} units tall after 2022 rocks fall", result);
}
