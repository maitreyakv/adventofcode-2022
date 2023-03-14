use std::collections::HashMap;
use std::rc::Rc;
use std::fs;

trait FileSystemObject {
    fn name(&self) -> String;
    fn size(&self) -> usize;
    fn parent(&self) -> Rc<Directory>;
}

struct File {
    name: String,
    size: usize,
    parent: Option<Rc<Directory>>
}

impl File {
    fn new(
        name: String,
        size: usize,
        parent: Option<Rc<Directory>>
    ) -> Self { File { name, size, parent } }
}

impl FileSystemObject for File {
    fn name(&self) -> String { self.name.clone() }
    fn size(&self) -> usize { self.size }
    fn parent(&self) -> Rc<Directory> {
        Rc::clone(self.parent.as_ref().expect("error, no parent"))
    }
}

struct Directory {
    name: String,
    contents: HashMap<String, Rc<dyn FileSystemObject>>,
    parent: Option<Rc<Directory>>
}

impl Directory {
    fn new(name: String, parent: Option<Rc<Directory>>) -> Self {
        Directory { name: name, contents: HashMap::new(), parent: parent }
    }

    fn add_content(&mut self, object: Rc<dyn FileSystemObject>) {
        self.contents.insert(object.name(), Rc::clone(&object));
    }

    fn get_child_directory(&self, name: String) -> Rc<Directory> {
        let result = self.contents.get(&name).unwrap();
        Rc::clone(result)
    }
}

impl FileSystemObject for Directory {
    fn name(&self) -> String { self.name.clone() }
    fn size(&self) -> usize {
        // TODO: Implement
        0
    }

    fn parent(&self) -> Rc<Directory> {
        Rc::clone(self.parent.as_ref().expect("error, no parent"))
    }
}

fn main() {
    let input = fs::read_to_string("test_input.txt").unwrap();
    let lines = input.lines().skip(1).peekable();

    let root: Rc<Directory> = Rc::new(Directory::new("/".to_string(), None));
    let mut cwd = Rc::clone(&root);
    
    for line in lines {
        if line.contains("$ ls") {
            continue;
        } else if line.contains("$ cd ..") {
            cwd = cwd.parent();
        } else if line.contains("$ cd") {
            let dir = line.split_ascii_whitespace().last().unwrap();
            cwd = cwd.get_content("").unwrap();   //.get_content(dir.to_string()).unwrap();
        }
    }
}
