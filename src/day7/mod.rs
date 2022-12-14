use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;
use itertools::Itertools;

pub fn day7_part1() {
    let mut tree = infer_file_tree();
    let mut result: Vec<u32> = Vec::new();
    calc_sizes(&mut tree);
    // print_tree(&tree, 0);
    find_dir_sizes_smaller_than(&tree, 100000, &mut result);
    let sum: u32 = result.iter().sum();
    println!("Sum: {}", sum);
}

pub fn day7_part2() {
    let mut tree = infer_file_tree();
    calc_sizes(&mut tree);
    // print_tree(&tree, 0);
    let disk_space = 70000000;
    let target_space = 30000000;
    let used_space = tree.size;
    let free_space = disk_space - used_space;
    let missing_space = target_space - free_space;
    let dir = find_dir_to_delete(&tree, missing_space, tree.size).unwrap();
    println!("Dir to delete: {}, size of that dir: {}", dir.name, dir.size);
}

fn calc_sizes(tree: &mut FileTreeNode) {
    if tree.is_dir {
        let size = get_size(tree);
        tree.size = size;
    }
    for child in &mut tree.children {
        calc_sizes(child);
    }
}

fn find_dir_to_delete(node: &FileTreeNode, target_size: u32, mut current_size: u32) -> Option<&FileTreeNode> {
    let mut smallest_dir: Option<&FileTreeNode> = None;
    if node.is_dir && node.size >= target_size && node.size < current_size {
        smallest_dir = Some(node);
        current_size = node.size;
    }
    for child in &node.children {
        if let Some(dir) = find_dir_to_delete(child, target_size, current_size) {
            smallest_dir = Some(dir);
            current_size = dir.size;
        }
    }
    smallest_dir
}

fn find_dir_sizes_smaller_than(node: &FileTreeNode, target: u32, result: &mut Vec<u32>) {
    if node.is_dir {
        if node.size <= target {
            result.push(node.size);
        }
    }
    for child in &node.children {
        find_dir_sizes_smaller_than(child, target, result);
    }
}

fn get_size(node: &FileTreeNode) -> u32 {
    let mut size = node.size;
    for child in &node.children {
        size += get_size(child);
    }
    size
}

fn print_tree(node: &FileTreeNode, depth: usize) {
    print_tree_node(node, depth);
    for child in &node.children {
        print_tree(child, depth + 1);
    }
}

fn print_tree_node(node: &FileTreeNode, depth: usize) {
    println!("{}- {} ({}, size={})",
             "  ".repeat(depth), node.name, if node.is_dir { "dir" } else { "file" }, node.size);
}

pub struct FileTreeNode {
    name: String,
    children: Vec<FileTreeNode>,
    size: u32,
    is_dir: bool,
}

fn infer_file_tree() -> FileTreeNode {
    let file = File::open("src/day7/input.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut tree = FileTreeNode {
        name: String::from("/"),
        children: Vec::new(),
        size: 0,
        is_dir: true,
    };
    // I wanted to use a stack of FileTreeNodes, but I couldn't get it to work because of the borrow
    // checker. Hence this hacky solution.
    let mut path_stack: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        match &line[..4] {
            "$ cd" => {
                match &line[5..] {
                    ".." => {
                        path_stack.pop();
                    }
                    _ => {
                        path_stack.push((&line[5..]).parse().unwrap());
                    }
                }
            }
            "$ ls" => {}
            _ => {
                match &line[..3] {
                    "dir" => {
                        let dir = FileTreeNode {
                            name: String::from(&line[4..]),
                            children: Vec::new(),
                            size: 0,
                            is_dir: true,
                        };
                        let path = get_current_path(&path_stack);
                        find_node_by_path(&mut tree, &path).unwrap().children.push(dir);
                    }
                    _ => {
                        let (size, name) = line.split_whitespace().next_tuple().unwrap();
                        let file = FileTreeNode {
                            name: String::from(name),
                            children: Vec::new(),
                            size: size.parse().unwrap(),
                            is_dir: false,
                        };
                        let path = get_current_path(&path_stack);
                        find_node_by_path(&mut tree, &path).unwrap().children.push(file);
                    }
                }
            }
        }
    }

    tree
}

fn get_current_path(stack: &Vec<String>) -> String {
    let mut path = String::new();
    for node in stack {
        path.push_str(node);
        if node != "/" {
            path.push_str("/");
        }
    }
    path
}

fn find_node_by_path<'a>(tree: &'a mut FileTreeNode, path: &str) -> Option<&'a mut FileTreeNode> {
    let mut current_node = tree;
    for path_part in path.split("/") {
        if path_part == "" {
            continue;
        }
        current_node = current_node.children.iter_mut().find(|child| child.name == path_part)?;
    }
    Some(current_node)
}
