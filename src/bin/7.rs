use core::str::Lines;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

type File = HashMap<String, usize>;
type Ref = Rc<RefCell<Dir>>;

struct Dir {
    name: String,
    files: File,
    dirs: HashMap<String, DirRef>,
    parent: Option<Ref>,
    size: usize,
}

struct DirRef {
    dir: Ref,
}

impl DirRef {
    fn new(name: String, parent: Option<Ref>) -> DirRef {
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
        println!("{}", &new_dir);
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

fn main() {
    let mut lines = include_str!("../input/7.txt").lines();
    let fs = build_fs(&mut lines).unwrap();
    let fs = fs.dir.borrow();

    let root_file = &fs.name;
    let root_size = &fs.size;
    let first_child = fs.dirs.get("dpbwg").unwrap();
    let first_child = first_child.dir.borrow();
    let first_name = &first_child.name;
    let first_size = &first_child.size;

    let second_child = first_child.dirs.get("dgh").unwrap();
    let second_child = second_child.dir.borrow();
    let second_name = &second_child.name;
    let second_size = &second_child.size;

    println!("{root_file:?}");
    println!("{root_size:?}");
    println!("{first_name:?}");
    println!("{first_size:?}");
    println!("{second_name:?}");
    println!("{second_size:?}");
}
