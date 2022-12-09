use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

const SPACE_NEEDED: i32 = 30_000_000;
const MAX_MEMORY: i32 = 70_000_000;

type Directories = Rc<RefCell<Directory>>;

#[derive(Debug)]
struct Directory {
    name: String,
    sub_dir: Vec<Directories>,
    sub_files: Vec<File>,
    parent: Option<Weak<RefCell<Directory>>>,
}
impl Directory {
    fn new(s: &str) -> Self {
        Directory {
            name: s.split(" ").last().unwrap().to_string(),
            sub_dir: vec![],
            sub_files: vec![],
            parent: None,
        }
    }

    fn new_parent() -> Self {
        Directory {
            name: "/".to_string(),
            sub_dir: vec![],
            sub_files: vec![],
            parent: None,
        }
    }

    fn set_parent(&mut self, d: Rc<RefCell<Directory>>) {
        self.parent = Some(Rc::downgrade(&d));
    }

    fn add_file(&mut self, file: File) {
        if let None = self.sub_files.iter().find(|f| f.name == file.name) {
            self.sub_files.push(file);
        }
    }

    fn add_dir(&mut self, dir: Directory) {
        if let None = self.sub_dir.iter().find(|d| d.borrow().name == dir.name) {
            self.sub_dir.push(Rc::new(RefCell::new(dir)));
        }
    }

    fn get_mem(&self) -> i32 {
        let mut mem = 0;
        for f in &self.sub_files {
            mem += f.mem;
        }
        for d in &self.sub_dir {
            mem += d.borrow().get_mem();
        }
        mem
    }

    fn p1(&self) -> i32 {
        let mut sum = 0;
        let d_mem = self.get_mem();
        if d_mem <= 100000 {
            sum += d_mem;
        }
        for d in &self.sub_dir {
            sum += d.borrow().p1();
        }
        sum
    }

    fn p2(&self) -> i32 {
        let mem_needed = SPACE_NEEDED - (MAX_MEMORY - self.get_mem());
        let mut closest = i32::max_value();

        self.p2_recur(mem_needed, &mut closest);
        closest
    }
    fn p2_recur(&self, mem_needed: i32, closest: &mut i32) {
        for d in &self.sub_dir {
            let mem = d.borrow().get_mem();
            if mem >= mem_needed && mem < *closest {
                *closest = mem;
            }
            d.borrow().p2_recur(mem_needed, closest);
        }
    }
}

#[derive(Debug)]
struct File {
    name: String,
    mem: i32,
}
impl File {
    fn new(s: &str) -> Self {
        let mut s = s.split(" ");
        Self {
            mem: s.next().unwrap().parse().unwrap(),
            name: s.next().unwrap().to_string(),
        }
    }
}

fn main() {
    let contents = include_str!("input.txt");
    let lines = contents.lines();

    let parent = Rc::new(RefCell::new(Directory::new_parent()));
    let mut current_dir = parent.clone();

    for line in lines {
        if line.contains("$") {
            let mut w = line.split(" ");
            w.next();
            match (w.next(), w.next()) {
                (Some("cd"), Some("..")) => {
                    if let Some(x) = &current_dir.clone().borrow().parent {
                        current_dir = x.upgrade().unwrap().clone();
                    }
                }
                (Some("cd"), Some(name)) => {
                    if let Some(d) = current_dir
                        .clone()
                        .borrow()
                        .sub_dir
                        .iter()
                        .find(|d| d.borrow().name == name.to_string())
                    {
                        current_dir = d.clone();
                    }
                }
                (Some("ls"), None) => {}
                _ => println!("Invalid command"),
            }
        } else if line.contains("dir") {
            let mut d = Directory::new(line);
            d.set_parent(current_dir.clone());
            current_dir.borrow_mut().add_dir(d);
        } else {
            current_dir.borrow_mut().add_file(File::new(line));
        }
    }

    let mem = parent.borrow_mut().p1();
    let mem2 = parent.borrow().p2();

    println!("P1:{:?}, P2:{:?}", mem, mem2);
}
