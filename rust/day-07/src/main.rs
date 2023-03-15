use std::collections::HashMap;
use std::fs;

struct File {
    name: String,
    size: usize
}

struct Directory {
    name: String,
    files: HashMap<String, File>,
    directories: HashMap<String, Directory>
}

impl Directory {
    fn build_root() -> Self {
        Directory {
            name: "/".to_string(),
            files: HashMap::new(),
            directories: HashMap::new()
        }
    }

    fn add_file(&mut self, name: String, size: usize) {
        let new_file = File {
            name: name.clone(),
            size: size
        };
        self.files.insert(name, new_file);
    }

    fn add_directory(&mut self, name: String) {
        let new_dir = Directory {
            name: name.clone(),
            files: HashMap::new(),
            directories: HashMap::new()
        };
        self.directories.insert(name, new_dir);
    }

    fn get_directory(&mut self, name: String) -> &mut Directory {
        self.directories.get_mut(&name).unwrap()
    }
}

fn main() {
    let input = fs::read_to_string("test_input.txt").unwrap();
    let lines = input.lines().skip(1).peekable();

    let mut root = Directory::build_root();
    let mut cwd = &mut root;
    let mut dir_stack: Vec<&mut Directory> = Vec::new();
    
    for line in lines {
        if line.contains("$ ls") {
            continue;
        } else if line.contains("$ cd ..") {
            cwd = dir_stack.pop().expect("error, no parent dirs on stack");
        } else if line.contains("$ cd") {
            let next_dir_name = line.split_ascii_whitespace().last().unwrap();
            dir_stack.push(cwd);
            cwd = dir_stack.last_mut().unwrap().get_directory(next_dir_name.to_string());
        } else {
            let (size, name) = line.split_once(' ').unwrap();
            if size == "dir" {
                cwd.add_directory(name.to_string())
            } else {

            }
        }
    }
}
