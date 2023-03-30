use std::{ fs, cmp };
use std::collections::{HashMap, HashSet, VecDeque};

struct Graph<T> {
    vertices: Vec<T>,
    edges: HashMap<(usize, usize), usize>
}

impl<T> Graph<T>
where T: Clone + cmp::PartialEq {
    fn new() -> Self {
        Self {
            vertices: Vec::new(),
            edges: HashMap::new()
        }
    }

    fn reverse(&self) -> Self {
        let mut G = Self {
            vertices: self.vertices.clone(),
            edges: HashMap::new()
        };

        for ((v1, v2), &edge) in self.edges.iter() {
            G.edges.insert((*v2, *v1), edge);
        }

        G
    }

    fn add_vertex(&mut self, content: T) {
        self.vertices.push(content);
    }

    fn add_edge(&mut self, v1: usize, v2: usize, edge: usize) {
        self.edges.insert((v1, v2), edge);
    }

    fn get_neighbors(&self, u: usize) -> impl Iterator<Item = usize> + '_ {
        let iter = 0..self.vertices.len();
        iter.filter(move |&v| self.edges.contains_key(&(u, v)))
    }

    fn dijkstra(&self, source: usize, target: usize) -> usize {
        let mut dist = vec![usize::MAX; self.vertices.len()];
        let mut prev: Vec<Option<usize>> = vec![None; self.vertices.len()];
        let mut Q: HashSet<usize> = (0..self.vertices.len()).collect();

        dist[source] = 0;

        while !Q.is_empty() {
            let u = *Q.iter()
                      .reduce(|x, y| if dist[*x] <= dist[*y] {x} else {y})
                      .unwrap();
            if u == target {
                break
            }
            if !Q.remove(&u) {
                panic!("problem removing vertex from queue");
            };

            for v in Q.iter().filter(|&v| self.edges.contains_key(&(u, *v))) {
                let alt = dist[u] + self.edges.get(&(u, *v)).unwrap();
                if alt < dist[*v] {
                    dist[*v] = alt;
                    prev[*v] = Some(u);
                }
            }
        }

        dist[target]
    }

    fn breadth_first_search(
        &self,
        source: usize,
        target_content: T
    ) -> Option<usize> {
        let mut Q = VecDeque::new();
        let mut explored = HashSet::new();
        let mut dist: Vec<usize> = vec![usize::MAX; self.vertices.len()];

        explored.insert(source);
        Q.push_back(source);
        dist[source] = 0;
        
        while !Q.is_empty() {
            let v = Q.pop_front().unwrap();

            if self.vertices[v] == target_content {
                return Some(dist[v]);
            }

            let neighbors = self.get_neighbors(v);
            for w in neighbors {
                dist[w] = dist[v] + self.edges.get(&(v, w)).unwrap();
                if !explored.contains(&w) {
                    explored.insert(w);
                    Q.push_back(w);
                }
            }
        }

        None
    }
}

struct Map {
    heights: Vec<usize>,
    n: usize,
    m: usize
}

impl Map {
    fn from_file(filename: &str) -> (Self, usize, usize) {
        let file = fs::read_to_string(filename.to_string()).unwrap();
        let n = file.lines().count();
        let mut m = 0;
        let mut heights = Vec::new();
        let mut source = 0;
        let mut target = 0;
        for line in file.lines() {
            for c in line.chars() {
                if c == 'S' {
                    source = heights.len();
                } else if c == 'E' {
                    target = heights.len();
                }

                let height = match c {
                    'S' => 'a' as usize,
                    'E' => 'z' as usize,
                    _ => c as usize
                };
                heights.push(height);
            }
            m = line.len();
        }
        (Self { heights, n, m }, source, target)
    }

    fn flat_index(&self, i: usize, j: usize) -> usize {
        self.m * i + j
    }

    fn get(&self, i: usize, j: usize) -> Option<&usize> {
        self.heights.get(self.flat_index(i, j))
    }

    fn to_graph(&self) -> Graph<usize> {
        // Initialize empty graph
        let mut G = Graph::new();

        // Add all vertices to graph
        for i in 0..self.n {
            for j in 0..self.m {
                G.add_vertex(*self.get(i, j).unwrap());
            }
        }

        // Add edges for each vertex
        for i in 0..self.n {
            for j in 0..self.m {
                // Get height of vertex
                let h = self.get(i, j).unwrap();

                // Add up neighbor
                if i > 0 && *self.get(i - 1, j).unwrap() <= h + 1 {
                    G.add_edge(
                        self.flat_index(i, j),
                        self.flat_index(i - 1, j),
                        1
                    );
                }

                // Add down neighbor
                if i < self.n - 1 && *self.get(i + 1, j).unwrap() <= h + 1 {
                    G.add_edge(
                        self.flat_index(i, j),
                        self.flat_index(i + 1, j),
                        1
                    );
                }

                // Add left neighbor
                if j > 0 && *self.get(i, j - 1).unwrap() <= h + 1 {
                    G.add_edge(
                        self.flat_index(i, j),
                        self.flat_index(i, j - 1),
                        1
                    );
                }

                // Add right neighbor
                if j < self.m - 1 && *self.get(i, j + 1).unwrap() <= h + 1 {
                    G.add_edge(
                        self.flat_index(i, j),
                        self.flat_index(i, j + 1),
                        1
                    );
                }
            }
        }
        
        G
    }
}

fn main() {
    let (map, source, target) = Map::from_file("test_input.txt");
    let graph = map.to_graph();
    let result = graph.dijkstra(source, target);
    println!("Path from S to E takes {result} steps");

    println!(
        "Path from S to some 'a' takes {} steps",
        graph.reverse().breadth_first_search(target, 'a' as usize).unwrap()
    )
}
