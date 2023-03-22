use std::{fs};

struct File {
    name: String,
    size: usize
}

impl File {
    fn new(name: &str, size: usize) -> Self {
        File {
            name: name.to_string(),
            size: size
        }
    }
}

struct Directory {
    name: String,
    files: Vec<usize>,
    dirs: Vec<usize>,
    parent: Option<usize>,
}

impl Directory {
    fn new(name: &str, parent: Option<usize>) -> Self {
        Directory {
            name: name.to_string(),
            files: Vec::new(),
            dirs: Vec::new(),
            parent: parent,
        }
    }

    fn add_file(&mut self, index: usize) {
        self.files.push(index);
    }

    fn add_dir(&mut self, index: usize) {
        self.dirs.push(index);
    }
}

struct FileSystem {
    cwd_index: usize,
    files: Vec<File>,
    dirs: Vec<Directory>
}

impl FileSystem {
    fn new() -> Self {
        FileSystem {
            cwd_index: 0,
            files: Vec::new(),
            dirs: vec![Directory::new("/", None)],
        }
    }

    fn create_file(&mut self, name: &str, size: usize) {
        self.files.push(File::new(name, size));
        let index = self.files.len() - 1;
        self.dirs[self.cwd_index].add_file(index);
    }

    fn create_dir(&mut self, name: &str) {
        self.dirs.push(Directory::new(name, Some(self.cwd_index)));
        let index = self.dirs.len() - 1;
        self.dirs[self.cwd_index].add_dir(index);
    }

    fn move_in(&mut self, name: &str) {
        let cwd = &self.dirs[self.cwd_index];
        for &dir_index in cwd.dirs.iter() {
            if self.dirs[dir_index].name == name {
                self.cwd_index = dir_index;
                return
            }
        }
        panic!("Could not find dir {name}!");
    }

    fn move_up(&mut self) {
        let cwd = &self.dirs[self.cwd_index];
        self.cwd_index = cwd.parent.expect("Could not move up, reached root!");
    }

    fn move_to_root(&mut self) {
        self.cwd_index = 0;
    }

    fn compute_sizes(&self) -> Vec<Option<usize>> {
        let n_dirs = self.dirs.len();
        let mut sizes: Vec<Option<usize>> = vec![None; n_dirs];
        while (0..n_dirs).any(|i| sizes[i].is_none()) {
            for i in 0..n_dirs {
                let dir = &self.dirs[i];
                if dir.dirs.iter().all(|&j| sizes[j].is_some()) {
                    let mut size = 0;
                    for &file_index in dir.files.iter() {
                        size += self.files[file_index].size;
                    }
                    for &dir_index in dir.dirs.iter() {
                        size += sizes[dir_index].unwrap();
                    }
                    sizes[i] = Some(size);
                }
            }
        }
        sizes
    }
}

fn main() {
    let input = fs::read_to_string("test_input.txt").unwrap();
    let lines = input.lines().skip(1).peekable();

    let mut file_sys = FileSystem::new();
        
    for line in lines {
        if line.contains("$ ls") {
            continue;
        } else if line.contains("$ cd ..") {
            file_sys.move_up();
        } else if line.contains("$ cd") {
            let name = line.split_ascii_whitespace().last().unwrap();
            file_sys.move_in(name);
        } else {
            let (size, name) = line.split_once(' ').unwrap();
            if size == "dir" {
                file_sys.create_dir(name);
            } else {
                let size = size.parse::<usize>().unwrap();
                file_sys.create_file(name, size);
            }
        }
    }

    file_sys.move_to_root();

    let result: usize = file_sys
                    .compute_sizes()
                    .iter()
                    .map(|&x| x.unwrap())
                    .filter(|&x| x < 100_000)
                    .sum();
    println!("{result}");

    let mut sizes: Vec<usize> = file_sys
                                    .compute_sizes()
                                    .iter()
                                    .map(|&x| x.unwrap())
                                    .collect();
    let required_delete_space = 30000000 - (70000000 - sizes[0]);
    sizes.sort();
    let result = sizes
                    .iter()
                    .filter(move |&x| *x >= required_delete_space)
                    .nth(0)
                    .unwrap();
    println!("{result}");
}
