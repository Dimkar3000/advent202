use crate::Problem;
use std::cmp::{max, min};

#[derive(Debug, Default, Clone, PartialEq)]
struct File(String, usize);

#[derive(Debug, Default, Clone, PartialEq)]
struct Folder {
    folder_name: String,
    folders: Vec<Folder>,
    files: Vec<File>,
}

impl Folder {
    fn total_size(&self) -> usize {
        let mut total_size = self.files.iter().map(|x| x.1).sum();
        for f in &self.folders {
            total_size += f.total_size();
        }
        total_size
    }

    fn add_folder_to_child(&mut self, path: &[&str], folder: &Folder) {
        let mut item = None;
        if let Some(folder_name) = path.first() {
            item = Some(folder_name.to_string())
        }
        if let Some(folder_name) = item {
            for f in &mut self.folders {
                if f.folder_name == *folder_name {
                    f.add_folder_to_child(&path[1..], folder);
                    break;
                }
            }
        } else {
            self.folders.push(folder.clone())
        }
    }
    fn add_file_to_child(&mut self, path: &[&str], file: &File) {
        if let Some(folder_name) = path.first() {
            for folder in &mut self.folders {
                if folder.folder_name == *folder_name {
                    folder.add_file_to_child(&path[1..], file);
                    break;
                }
            }
        } else {
            self.files.push(file.clone())
        }
    }
}

#[derive(Default, Debug)]
pub struct Problem7 {
    root: Folder,
}

impl Problem7 {
    fn proccess_input(&mut self, input: &str) {
        let mut lines = input.lines().peekable();
        let mut current_dir: Vec<&str> = Vec::new();

        let mut root = Folder {
            folder_name: "/".to_string(),
            ..Default::default()
        };

        while let Some(line) = lines.next() {
            if line.starts_with('$') {
                let mut parts = line.split(' ');
                parts.next().unwrap();
                let cmd = parts.next().unwrap();

                if cmd == "cd" {
                    let folder = parts.next().unwrap();
                    if folder == "/" {
                        current_dir = Vec::new();
                    } else if folder == ".." {
                        current_dir.pop();
                    } else {
                        current_dir.push(folder);
                    }
                } else if cmd == "ls" {
                    loop {
                        let next_line = lines.peek();
                        if next_line.is_none() {
                            break;
                        }
                        let content = next_line.unwrap();
                        if content.starts_with('$') {
                            break;
                        }

                        let content = lines.next().unwrap();
                        let (p1, p2) = content.split_once(' ').unwrap();
                        if p1 == "dir" {
                            // Create Folder
                            let folder = Folder {
                                folder_name: p2.to_string(),
                                ..Default::default()
                            };
                            root.add_folder_to_child(&current_dir, &folder);
                        } else {
                            // Create File
                            let new_file = File(p2.to_string(), p1.parse::<usize>().unwrap());
                            root.add_file_to_child(&current_dir, &new_file);
                        }
                    }
                } else {
                    panic!("else {cmd}");
                }
            }
        }

        self.root = root
    }
}

impl Problem7 {
    pub fn new() -> Box<Self> {
        Box::new(Self::default())
    }
}

fn callback(x: &Folder) -> usize {
    let total = x.total_size();
    let children: usize = x.folders.iter().map(callback).sum();
    if total > 100000 {
        return children;
    }
    total + children
}

fn callback2(required_space: usize, current: usize, folder: &Folder) -> usize {
    let my_size = max(required_space, folder.total_size());

    let t = folder
        .folders
        .iter()
        .fold(min(current, my_size), |current, folder| {
            callback2(required_space, current, folder)
        });

    if t > required_space && t < current {
        return t;
    }
    current
}

impl Problem for Problem7 {
    fn part1(&mut self, input: &str) -> String {
        self.proccess_input(input);
        self.root
            .folders
            .iter()
            .map(callback)
            .sum::<usize>()
            .to_string()
    }

    fn part2(&mut self, _input: &str) -> String {
        let used_space = self.root.total_size();
        let free_space = 70000000 - used_space;
        let required_space = 30000000 - free_space;

        let total = self.root.total_size();
        let result = self.root.folders.iter().fold(total, |current, folder| {
            callback2(required_space, current, folder)
        });

        result.to_string()
    }
}
