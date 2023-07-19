use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::read_to_string;
use std::rc::Rc;

struct Dir {
    parent: Option<Link>,
    child_dirs: RefCell<ChildDirs>,
    child_files: RefCell<ChildFiles>,
    size: Cell<usize>,
}

type Link = Rc<Dir>;
type ChildDirs = HashMap<String, Link>;
type ChildFiles = HashMap<String, usize>;

impl Dir {
    fn new(parent: Option<Link>) -> Self {
        Self {
            parent,
            child_dirs: RefCell::new(HashMap::new()),
            child_files: RefCell::new(HashMap::new()),
            size: 0.into(),
        }
    }

    fn get_child_ref(&self, name: &str) -> Option<Link> {
        self.child_dirs.borrow().get(name).cloned()
    }

    fn add_child(&self, name: &str, child: Link) {
        self.child_dirs.borrow_mut().insert(name.to_string(), child);
    }

    fn add_file(&self, name: &str, size: usize) {
        self.child_files
            .borrow_mut()
            .entry(name.to_string())
            .or_insert(size);
    }

    fn update_size(&self) {
        let file_sizes: usize = self.child_files.borrow().values().sum();
        let dir_sizes: usize = self
            .child_dirs
            .borrow_mut()
            .values()
            .map(|d| {
                d.update_size();
                d.size.get()
            })
            .sum();

        self.size.set(file_sizes + dir_sizes)
    }

    fn get_sizes_le_10000(&self, acc: &mut usize) -> usize {
        for d in self.child_dirs.borrow().values() {
            if d.size.get() <= 100000 {
                *acc += d.size.get();
            }
            d.get_sizes_le_10000(acc);
        }
        *acc
    }

    fn get_dir_to_delete(&self, acc: &mut i32, size_needed: i32) -> i32 {
        for d in self.child_dirs.borrow().values() {
            let s = d.size.get() as i32;
            if s >= size_needed && s < *acc {
                *acc = s
            }
            d.get_dir_to_delete(acc, size_needed);
        }
        *acc
    }

    fn print_self(&self, f: &mut std::fmt::Formatter<'_>, depth: usize) -> std::fmt::Result {
        for (k, v) in self.child_files.borrow().iter() {
            write!(f, "{}", " ".repeat(depth))?;
            writeln!(f, "{}", format!("-{} (file, size={})", k, v))?;
        }
        for (k, v) in self.child_dirs.borrow().iter() {
            write!(f, "{}", " ".repeat(depth))?;
            writeln!(f, "-{} (dir, size={})", k, v.size.get())?;
            v.print_self(f, depth + 2)?;
        }
        Ok(())
    }
}

impl Debug for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.print_self(f, 0)
    }
}

fn main() {
    let f = read_to_string("d7.txt").unwrap();

    let top = Rc::new(Dir::new(None));
    let mut crnt = top.clone();

    for line in f.lines() {
        match line.chars().nth(0).unwrap() {
            '$' => match &line[2..4] {
                "cd" => match &line[5..] {
                    "/" => {
                        while crnt.parent.is_some() {
                            crnt = crnt.parent.as_ref().unwrap().clone();
                        }
                    }
                    ".." => {
                        if let Some(parent) = &crnt.parent {
                            crnt = parent.clone();
                        }
                    }
                    d => {
                        if let Some(child) = crnt.get_child_ref(d) {
                            crnt = child;
                        }
                    }
                },
                _ => (),
            },
            _ => {
                let s: Vec<&str> = line.split(' ').collect();
                match s[0] {
                    "dir" => {
                        let new_dir = Rc::new(Dir::new(Some(crnt.clone())));
                        crnt.add_child(s[1], new_dir.clone());
                    }
                    _ => crnt.add_file(s[1], s[0].parse::<usize>().unwrap()),
                }
            }
        }
    }
    top.update_size();
    println!("{:?}", top);

    let mut acc = 0;
    println!(
        "Sum of directories < 100000: {}",
        top.get_sizes_le_10000(&mut acc)
    );

    top.update_size();
    let mut acc = 70000000;
    let size_needed = top.size.get() as i32 - 40000000;

    println!(
        "Size of dir to delete: {}",
        top.get_dir_to_delete(&mut acc, size_needed)
    );
}
