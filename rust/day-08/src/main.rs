use std::{ fs, cmp };

struct Matrix<T> {
    data: Vec<T>,
    n: usize,
    m: usize
}

impl<T> Matrix<T> {
    fn from_vec(data: Vec<T>, n: usize, m: usize) -> Self {
        Matrix { data, n, m }
    }

    fn get(&self, i: usize, j: usize) -> &T {
        self.data.get(i * self.n + j).expect("Out of bounds indices!")
    }
}

fn read_input() -> Matrix<usize> {
    let file = fs::read_to_string("test_input.txt").unwrap();
    let mut data = Vec::new();
    for line in file.lines() {
        for tree in line.chars() {
            let tree = tree.to_digit(10).unwrap() as usize;
            data.push(tree);
        }
    }
    let n = file.lines().count();
    let m = file.lines().nth(0).unwrap().len();
    Matrix::from_vec(data, n, m)
}

fn viewing_distances(
    forest: &Matrix<usize>, i: usize, j: usize
) -> (usize, usize, usize, usize) {
    let mut up = i;
    let mut down = forest.n - i - 1;
    let mut left = j;
    let mut right = forest.m - j - 1;
    let tree = forest.get(i, j);
    for ii in 1..up {
        if forest.get(i - ii, j) >= tree { up = ii; break }
    }
    for ii in 1..down {
        if forest.get(i + ii, j) >= tree { down = ii; break }
    }
    for jj in 1..left {
        if forest.get(i, j - jj) >= tree { left = jj; break }
    }
    for jj in 1..right {
        if forest.get(i, j + jj) >= tree { right = jj; break }
    }
    (up, down, left, right)
}

fn is_visible(forest: &Matrix<usize>, i: usize, j: usize) -> bool {
    let tree = forest.get(i, j);
    if i == 0 || i == forest.n - 1 { return true }
    if j == 0 || j == forest.m - 1 { return true }
    let (up, down, left, right) = viewing_distances(forest, i, j);
    if up == i && forest.get(0, j) < tree {
        return true
    }
    if down == forest.n - i - 1 && forest.get(forest.n - 1, j) < tree {
        return true
    }
    if left == j && forest.get(i, 0) < tree {
        return true
    }
    if right == forest.m - j - 1 && forest.get(i, forest.m - 1) < tree {
        return true
    }
    false
}

fn count_visible(forest: &Matrix<usize>) -> usize {
    let mut num_visible = 0;
    for i in 0..forest.n {
        for j in 0..forest.m {
            if is_visible(forest, i, j) {
                num_visible += 1;
            }
        }
    }
    num_visible
}

fn highest_scenic_score(forest: &Matrix<usize>) -> usize {
    let mut max_score = 0;
    for i in 0..forest.n {
        for j in 0..forest.m {
            let (up, down, left, right) = viewing_distances(forest, i, j);
            let score = up * down * left * right;
            max_score = cmp::max(score, max_score);
        }
    }
    max_score
}

fn main() {
    let forest = read_input();

    let result = count_visible(&forest);
    println!("{result} visible trees");

    let result = highest_scenic_score(&forest);
    println!("{result} maximum score");
}
