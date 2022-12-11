use std::{cell::RefCell, collections::HashMap, fs::read_to_string, rc::Rc};

use regex::Regex;

#[derive(Debug)]
enum Command {
    ChangeDirectoryBack,
    ChangeDirectoryForward(String),
    List { file_strs: Vec<String> },
}
use Command::{ChangeDirectoryBack, ChangeDirectoryForward, List};

#[derive(Debug)]
struct FileTree {
    parent: Option<Rc<RefCell<FileTree>>>,
    name: String,
    files: Option<HashMap<String, Rc<RefCell<FileTree>>>>,
    size: u32,
}

fn add_directory_size(file_tree: Rc<RefCell<FileTree>>, directory_size: u32) {
    {
        let mut file_tree_borrow = file_tree.borrow_mut();
        file_tree_borrow.size += directory_size;
    }
    if let Some(parent) = file_tree.borrow().parent.as_ref() {
        add_directory_size(Rc::clone(parent), directory_size)
    }
}

fn build_file_tree(commands: impl Iterator<Item = Command>) -> Rc<RefCell<FileTree>> {
    let root = Rc::new(RefCell::new(FileTree {
        parent: None,
        name: String::from("/"),
        files: None,
        size: 0,
    }));

    commands.fold(Rc::clone(&root), |acc, command| match command {
        ChangeDirectoryBack => Rc::clone(acc.borrow().parent.as_ref().unwrap()),
        ChangeDirectoryForward(directory) => {
            match acc.borrow().files.as_ref().unwrap().get(&directory) {
                Some(file_tree) => Rc::clone(file_tree),
                None => {
                    println!("Cannot find directory {directory:?}");
                    panic!()
                }
            }
        }
        List { file_strs } => {
            if acc.borrow().files.is_some() {
                return acc;
            }

            let mut files: HashMap<String, Rc<RefCell<FileTree>>> = HashMap::new();
            let mut directory_size = 0;

            file_strs.iter().for_each(|file_str| {
                let mut file_tree = FileTree {
                    parent: Some(Rc::clone(&acc)),
                    name: String::from(""),
                    files: None,
                    size: 0,
                };

                if let Some(name_ref) = file_str.strip_prefix("dir ") {
                    file_tree.name = name_ref.to_owned();
                } else {
                    let mut split = file_str.split(" ");
                    let size: u32 = split.next().unwrap().parse().unwrap();
                    let name = split.next().unwrap().to_owned();

                    directory_size += size;
                    file_tree.size = size;
                    file_tree.name = name;
                }

                files.insert(file_tree.name.clone(), Rc::new(RefCell::new(file_tree)));
            });

            add_directory_size(Rc::clone(&acc), directory_size);
            acc.borrow_mut().files = Some(files);

            acc
        }
    });

    return Rc::clone(&root);
}

fn read_input() -> Rc<RefCell<FileTree>> {
    let input = read_to_string("inputs/day7.txt").unwrap();
    let command_pattern = Regex::new(r"\n\$ ").unwrap();
    let change_directory_pattern = Regex::new("cd (.*)").unwrap();
    let commands = command_pattern.split(&input).skip(1).map(|command_str| {
        if let Some(caps) = change_directory_pattern.captures(command_str) {
            let directory = caps.get(1).unwrap().as_str();
            match directory {
                ".." => ChangeDirectoryBack,
                _ => ChangeDirectoryForward(directory.to_owned()),
            }
        } else if command_str.starts_with("ls") {
            let file_strs: Vec<String> = command_str
                .split_terminator("\n")
                .skip(1)
                .map(String::from)
                .collect();
            List { file_strs }
        } else {
            println!("{command_str:?}");
            panic!()
        }
    });

    build_file_tree(commands)
}

fn get_directory_sizes(file_tree_ref: Rc<RefCell<FileTree>>) -> Vec<u32> {
    let file_tree = file_tree_ref.borrow();
    let mut sizes: Vec<u32> = Vec::new();

    if let Some(files) = &file_tree.files {
        sizes.push(file_tree.size);
        files
            .values()
            .for_each(|child_tree| sizes.append(&mut get_directory_sizes(Rc::clone(child_tree))));
    }

    sizes
}

fn part1(directory_sizes: &Vec<u32>) -> u32 {
    directory_sizes
        .iter()
        .filter(|size| **size <= 100_000)
        .sum()
}

fn part2(directory_sizes: &Vec<u32>) -> u32 {
    let total_disk_space = 70_000_000;
    let used_space = directory_sizes.first().unwrap();
    let free_space = total_disk_space - used_space;
    let needed_space = 30_000_000 - free_space;

    directory_sizes
        .iter()
        .filter(|size| **size >= needed_space)
        .min_by_key(|size| **size - needed_space)
        .unwrap()
        .clone()
}

pub fn solve() -> (u32, u32) {
    let file_tree = read_input();
    let directory_sizes = get_directory_sizes(file_tree);

    (part1(&directory_sizes), part2(&directory_sizes))
}
