use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

#[derive(Debug)]
enum Node {
    Dir(HashSet<PathBuf>),
    File(u32),
}

impl Node {
    fn new_dir() -> Self {
        Self::Dir(HashSet::new())
    }

    fn new_file(size: u32) -> Self {
        Self::File(size)
    }

    fn add_child(&mut self, child: PathBuf) {
        if let Node::Dir(children) = self {
            children.insert(child);
        }
    }
}

fn build_tree(input: String) -> HashMap<PathBuf, Node> {
    let mut lines = input.lines();
    let mut tree = HashMap::<PathBuf, Node>::new();
    lines.next();
    let mut current_dir: PathBuf = "/".into();
    tree.insert(current_dir.clone(), Node::new_dir());

    for line in lines {
        let args = line.split(' ').collect::<Vec<_>>();
        if args[0] == "$" {
            // This is a command
            if args[1] == "cd" {
                // change directory
                if args[2] == ".." {
                    // go up a directory
                    current_dir = current_dir.parent().unwrap().into();
                } else {
                    current_dir = current_dir.join(args[2]);
                }
            } else {
                // ls is the only other command
                continue;
            }
        } else {
            // this is information about the current directory
            let new_path = current_dir.join(args[1]);
            tree.get_mut(&current_dir)
                .unwrap()
                .add_child(new_path.clone());
            // TODO: maybe need to say if node not already in tree
            if args[0] == "dir" {
                tree.insert(new_path, Node::new_dir());
            } else {
                tree.insert(new_path, Node::new_file(args[0].parse().unwrap()));
            }
        }
    }

    tree
}

fn directory_sizes(tree: &HashMap<PathBuf, Node>) -> HashMap<PathBuf, u32> {
    let mut dir_sizes = HashMap::<PathBuf, u32>::new();
    get_dir_sizes(tree, &mut dir_sizes, &"/".into());
    dir_sizes
}

fn get_dir_sizes(
    tree: &HashMap<PathBuf, Node>,
    dir_sizes: &mut HashMap<PathBuf, u32>,
    current_dir: &PathBuf,
) -> u32 {
    if let Some(Node::Dir(children)) = tree.get(current_dir) {
        let mut total = 0;
        for child in children {
            if let Node::File(size) = tree.get(child).unwrap() {
                total += size;
            } else {
                total += get_dir_sizes(tree, dir_sizes, child);
            }
        }
        dir_sizes.insert(current_dir.clone(), total);
        total
    } else {
        panic!("{}, not a node in the tree", current_dir.to_string_lossy());
    }
}

pub fn part1(input: String) -> u32 {
    let tree = build_tree(input);
    let dir_sizes = directory_sizes(&tree);

    dir_sizes
        .iter()
        .filter_map(|(_, size)| if *size <= 100000 { Some(size) } else { None })
        .sum()
}

pub fn part2(input: String) -> u32 {
    let tree = build_tree(input);
    let dir_sizes = directory_sizes(&tree);

    let target_mem_use = 40_000_000;
    let total_mem_use = dir_sizes.get(&PathBuf::from("/")).unwrap();

    let min_size_to_delete = total_mem_use - target_mem_use;

    *dir_sizes
        .iter()
        .filter_map(|(_, size)| {
            if *size >= min_size_to_delete {
                Some(size)
            } else {
                None
            }
        })
        .min()
        .unwrap()
}
