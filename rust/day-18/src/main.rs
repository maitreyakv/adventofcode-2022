use std::fs;
use std::collections::HashSet;
use std::ops::RangeInclusive;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Cube {
    x: usize,
    y: usize,
    z: usize
}

impl Cube {
    fn from_string(s: &str) -> Self {
        let coords: Vec<&str> = s.split(",").collect();
        Self {
            x: coords[0].parse::<usize>().unwrap(),
            y: coords[1].parse::<usize>().unwrap(),
            z: coords[2].parse::<usize>().unwrap()
        }
    }

    fn neighbors(&self) -> Vec<Self> {
        vec![
            Self { x: self.x - 1, ..*self },
            Self { x: self.x + 1, ..*self },
            Self { y: self.y - 1, ..*self },
            Self { y: self.y + 1, ..*self },
            Self { z: self.z - 1, ..*self },
            Self { z: self.z + 1, ..*self }
        ]
    }
}

struct BoundingBox {
    x: RangeInclusive<usize>,
    y: RangeInclusive<usize>,
    z: RangeInclusive<usize>
}

impl BoundingBox {
    fn contains(&self, cube: &Cube) -> bool {
        if cube.x < *self.x.start() || cube.x > *self.x.end() {
            false
        } else if cube.y < *self.y.start() || cube.y > *self.y.end() {
            false
        } else if cube.z < *self.z.start() || cube.z > *self.z.end() {
            false
        } else {
            true
        }
    }
}

struct Lava {
    cubes: HashSet<Cube>
}

impl Lava {
    fn from_file() -> Self {
        let file = fs::read_to_string("test_input.txt").unwrap();
        let cubes = file.lines()
                        .map(|l| Cube::from_string(l));
        Self { cubes: HashSet::from_iter(cubes) }
    }

    fn get_bounding_box(&self) -> BoundingBox {
        BoundingBox {
            x: 1..=self.cubes.iter().map(|c| c.x).max().unwrap(),
            y: 1..=self.cubes.iter().map(|c| c.y).max().unwrap(),
            z: 1..=self.cubes.iter().map(|c| c.z).max().unwrap()
        }
    }

    fn get_interior(&self) -> Vec<Cube> {
        let bbox = self.get_bounding_box();
        
        let mut unknown: HashSet<Cube> = HashSet::new();
        let mut exterior: HashSet<Cube> = HashSet::new();

        let mut queue: Vec<Cube> = Vec::new();

        for x in 0..=bbox.x.end() + 1 {
            for y in 0..=bbox.y.end() + 1 {
                for z in 0..=bbox.z.end() + 1 {
                    let cube = Cube { x, y, z };
                    if !self.cubes.contains(&cube) {
                        if bbox.contains(&cube) {
                            unknown.insert(cube);
                        } else {
                            queue.push(cube.clone());
                            exterior.insert(cube);
                        }
                    }   
                }
            }
        }

        while !queue.is_empty() {
            let cube = queue.pop().unwrap();
            for neighbor in cube.neighbors() {
                if unknown.contains(&neighbor) {
                    unknown.remove(&neighbor);
                    queue.push(neighbor.clone());
                    exterior.insert(neighbor);
                }
            }
        }

        unknown.into_iter().collect()
    }

    fn surface_area(&self, count_interior: bool) -> usize {

        let total_faces = self.cubes.iter()
                                .map(|c| {
                                    c.neighbors()
                                        .iter()
                                        .filter(|n| !self.cubes.contains(&n) )
                                        .count()
                                })
                                .sum();
        if count_interior {
            return total_faces
        }

        let interior = self.get_interior();
        let interior_faces: usize = interior.iter()
                                .map(|c| {
                                    c.neighbors()
                                        .iter()
                                        .filter(|n| !interior.contains(&n) )
                                        .count()
                                })
                                .sum();

        total_faces - interior_faces
    }
}

fn main() {
    let lava = Lava::from_file();
    let result = lava.surface_area(true);
    println!("surface area of lava is {result} total faces");

    let result = lava.surface_area(false);
    println!("surface area of lava is {result} exterior faces");
}
