use std::{fs, iter::{Once, Chain, once}, collections::{ HashMap, hash_map::Values}};

struct File {
    name: String,
    size: usize
}

impl File {
    fn new(name: &str, size: usize) -> Self {
        Self { name: name.to_string(), size: size }
    }
}

struct Directory {
    name: String,
    files: HashMap<String, File>,
    dirs: HashMap<String, Directory>,
    size: Option<usize>

}

impl Directory {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            files: HashMap::new(),
            dirs: HashMap::new(),
            size: None
        }
    }

    fn add_file(self, name: &str, size: usize) -> Self {
        let mut files = self.files;
        files.insert(name.to_string(), File::new(name, size));
        Self { files: files, size: None, ..self }
    }

    fn add_dir(self, name: &str) -> Self {
        let mut dirs = self.dirs;
        dirs.insert(name.to_string(), Self::new(name));
        Self { dirs: dirs, size: None, ..self }
    }

    fn move_up(self, dir_stack: &mut Vec<Directory>) -> Self {
        let mut parent = dir_stack.pop().unwrap();
        parent.dirs.insert(self.name.clone(), self);
        parent
    }

    fn move_in(mut self, dir_stack: &mut Vec<Directory>, name: &str) -> Self {
        let child = self.dirs.remove(name).unwrap();
        dir_stack.push(self);
        child
    }

    fn move_to_root(mut self, dir_stack: &mut Vec<Directory>) -> Self {
        while !dir_stack.is_empty() {
            self = self.move_up(dir_stack);
        }
        self
    }

    fn recompute_size(&mut self) -> usize {
        let mut size: usize = 0;
        for child in self.dirs.values_mut() {
            size += child.recompute_size();
        }
        for file in self.files.values() {
            size += file.size;
        }
        self.size = Some(size);
        size
    }
}

fn main() {
    let input = fs::read_to_string("test_input.txt").unwrap();
    let lines = input.lines().skip(1).peekable();

    let mut cwd = Directory::new("/");
    
    let mut dir_stack: Vec<Directory> = Vec::new();
    
    for line in lines {
        if line.contains("$ ls") {
            continue;
        } else if line.contains("$ cd ..") {
            cwd = cwd.move_up(&mut dir_stack);
        } else if line.contains("$ cd") {
            let name = line.split_ascii_whitespace().last().unwrap();
            cwd = cwd.move_in(&mut dir_stack, name);
        } else {
            let (size, name) = line.split_once(' ').unwrap();
            if size == "dir" {
                cwd = cwd.add_dir(name);
            } else {
                let size = size.parse::<usize>().unwrap();
                cwd = cwd.add_file(name, size);
            }
        }
    }

    cwd = cwd.move_to_root(&mut dir_stack);

    cwd.recompute_size();
}
