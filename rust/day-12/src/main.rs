use std::fs;
use std::collections::{HashMap, HashSet};

struct Graph<T> {
    vertices: Vec<T>,
    edges: HashMap<(usize, usize), usize>
}

impl<T> Graph<T> {
    fn new() -> Self {
        Self {
            vertices: Vec::new(),
            edges: HashMap::new()
        }
    }

    fn add_vertex(&mut self, content: T) -> usize {
        self.vertices.push(content);
        self.vertices.len() - 1
    }

    fn add_edge(&mut self, v1: usize, v2: usize, edge: usize) {
        self.edges.insert((v1, v2), edge);
    }

    fn dijkstra(&self, source: usize, target: usize) {
        let mut dist = vec![usize::MAX; self.vertices.len()];
        let mut prev: Vec<Option<usize>> = vec![None; self.vertices.len()];
        let mut Q: HashSet<usize> = (0..self.vertices.len()).collect();

        dist[source] = 0;

        while !Q.is_empty() {
            println!("{}", Q.len());
            let u = Q.iter().map(|&u| dist[u]).reduce(cmp::min).unwrap();
            if u == target {
                break
            }
            if !Q.remove(&u) {
                panic!("problem removing vertex from queue");
            };

            for v in Q.iter().filter(|&&v| self.edges.contains_key(&(u, v))) {
                let alt = dist[u] + self.edges.get(&(u, *v)).unwrap();
                if alt < dist[*v] {
                    dist[*v] = alt;
                    prev[*v] = Some(u);
                }
            }
        }

        dbg!(dist);
    }
}

struct Map {
    heights: Vec<usize>,
    n: usize,
    m: usize
}

impl Map {
    fn from_file(filename: &str) -> Self {
        let file = fs::read_to_string(filename.to_string()).unwrap();
        let n = file.lines().count();
        let mut m = 0;
        let mut heights = Vec::new();
        for line in file.lines() {
            for c in line.chars() {
                let height = match c {
                    'S' => usize::MAX - 1,
                    'E' => usize::MIN,
                    _ => c as usize
                };
                heights.push(height);
            }
            m = line.len();
        }
        Self { heights, n, m }
    }

    fn get_height(&self, i: usize, j: usize) -> Option<&usize> {
        self.heights.get(self.flat_index(i, j))
    }

    fn flat_index(&self, i: usize, j: usize) -> usize {
        self.m * i + j
    }

    fn to_graph(&self) -> Graph<usize> {
        // Initialize empty graph
        let mut G = Graph::new();

        // Add all vertices to graph
        for i in 0..self.n {
            for j in 0..self.m {
                G.add_vertex(*self.get_height(i, j).unwrap());
            }
        }

        // Add edges for each vertex
        for i in 0..self.n {
            for j in 0..self.m {
                // Get height of vertex
                let height = self.get_height(i, j).unwrap();

                // Add up neighbor
                if i > 0 {
                    if *self.get_height(i - 1, j).unwrap() <= height + 1 {
                        G.add_edge(
                            self.flat_index(i, j),
                            self.flat_index(i - 1, j),
                            1
                        );
                    }
                }

                // Add down neighbor
                if i < self.n - 1 {
                    if *self.get_height(i + 1, j).unwrap() <= height + 1 {
                        G.add_edge(
                            self.flat_index(i, j),
                            self.flat_index(i + 1, j),
                            1
                        );
                    }
                }

                // Add left neighbor
                if j > 0 {
                    if *self.get_height(i, j - 1).unwrap() <= height + 1 {
                        G.add_edge(
                            self.flat_index(i, j),
                            self.flat_index(i, j - 1),
                            1
                        );
                    }
                }

                // Add right neighbor
                if j < self.m - 1 {
                    if *self.get_height(i, j + 1).unwrap() <= height + 1 {
                        G.add_edge(
                            self.flat_index(i, j),
                            self.flat_index(i, j + 1),
                            1
                        )
                    }
                }
            }
        }
        
        G
    }
}

fn main() {
    let map = Map::from_file("test_input.txt");
    let graph = map.to_graph();
    graph.dijkstra(0, 22);
}
