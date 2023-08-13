use core::str::Lines;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

type File = HashMap<String, usize>;
type RefDir = Rc<RefCell<Dir>>;

struct Dir {
    name: String,
    files: File,
    dirs: HashMap<String, DirRef>,
    parent: Option<RefDir>,
    size: usize,
}

struct DirRef {
    dir: RefDir,
}

impl DirRef {
    fn new(name: String, parent: Option<RefDir>) -> DirRef {
        let new_dir = Dir {
            name,
            files: HashMap::new(),
            dirs: HashMap::new(),
            parent,
            size: 0,
        };

        DirRef {
            dir: Rc::new(RefCell::new(new_dir)),
        }
    }

    fn add_dir(&mut self, new_dir: String) -> Result<(), String> {
        let mut dir = self.dir.borrow_mut();
        let existing_dir = dir.dirs.get(&new_dir);

        match existing_dir {
            Some(_) => Err("directory with this name already exists".to_string()),
            None => {
                dir.dirs.insert(
                    new_dir.clone(),
                    DirRef::new(new_dir, Some(self.dir.clone())),
                );
                Ok(())
            }
        }
    }

    pub fn add_file(&self, size: usize, name: String) -> Result<(), String> {
        let mut dir = self.dir.borrow_mut();
        let existing_file = dir.files.get("name");

        match existing_file {
            Some(_) => Err("file with this name already exists".to_string()),
            None => {
                dir.files.insert(name, size);
                dir.size += size;

                let mut current_dir = dir.parent.clone();

                loop {
                    match current_dir {
                        Some(cd) => {
                            let mut cd_borrow = cd.borrow_mut();
                            cd_borrow.size += size;
                            current_dir = cd_borrow.parent.clone();
                        }
                        None => break,
                    }
                }

                Ok(())
            }
        }
    }

    pub fn get_parent(&self) -> Option<DirRef> {
        Some(DirRef {
            dir: self.dir.borrow().parent.clone()?,
        })
    }

    pub fn get_child(&self, name: String) -> Option<DirRef> {
        Some(DirRef {
            dir: self.dir.borrow_mut().dirs.get(&name)?.dir.clone(),
        })
    }

    pub fn clone(&self) -> DirRef {
        DirRef {
            dir: self.dir.clone(),
        }
    }
}

fn build_fs(lines: &mut Lines) -> Option<DirRef> {
    let root = DirRef::new("/".to_string(), None);
    let mut current_dir = root.clone();

    while let Some(line) = lines.next() {
        if line.starts_with("$ cd") {
            let mut parts = line.split(" ");
            parts.next();
            parts.next();
            let dir = parts.next()?;

            if dir.starts_with("..") {
                current_dir = current_dir.get_parent()?;
            } else if !dir.starts_with("/") {
                current_dir = current_dir.get_child(dir.to_string())?;
            }
        } else if line.starts_with("dir") {
            let mut parts = line.split(" ");
            parts.next();
            let dir = parts.next()?;

            let _ = current_dir.add_dir(dir.to_string());
        } else if !line.starts_with("$ ls") {
            let mut parts = line.split(" ");
            let size: usize = parts.next()?.parse().expect("size string is valid usize");
            let name = parts.next()?;

            let _ = current_dir.add_file(size, name.to_string());
        }
    }

    Some(root.clone())
}

// part 1
fn at_most_100000(total_size: &mut usize, dir_ref: &DirRef) {
    let dir_borrow = dir_ref.dir.borrow();

    if dir_borrow.size <= 100000 {
        *total_size += dir_borrow.size;
    }

    let mut children = dir_borrow.dirs.values();

    while let Some(child) = children.next() {
        at_most_100000(total_size, child);
    }
}

// part 2
fn at_least_6233734(deletion_candidates: &mut Vec<DirRef>, dir_ref: &DirRef) {
    let dir_borrow = dir_ref.dir.borrow();

    if dir_borrow.size >= 6233734 {
        deletion_candidates.push(dir_ref.clone());
    }

    let mut children = dir_borrow.dirs.values();

    while let Some(child) = children.next() {
        at_least_6233734(deletion_candidates, child);
    }
}

fn main() {
    let mut lines = include_str!("../input/7.txt").lines();
    let fs = build_fs(&mut lines).unwrap();

    // part 1
    let mut total_size = 0;
    at_most_100000(&mut total_size, &fs);
    println!("total_size: {total_size:?}");

    // part 2
    let mut deletion_candidates: Vec<DirRef> = Vec::new();
    at_least_6233734(&mut deletion_candidates, &fs);

    let mut min_size = usize::MAX;
    let mut min_folder = "".to_string();

    for candidate in deletion_candidates.iter() {
        let dir_borrow = candidate.dir.borrow();
        println!("dir: {}, size: {}", dir_borrow.name, dir_borrow.size);

        if dir_borrow.size < min_size {
            min_size = dir_borrow.size;
            min_folder = dir_borrow.name.clone();
        }
    }

    println!("min folder: {}", min_folder);
}
