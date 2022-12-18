use std::collections::HashSet;

fn construct_grid(input: String) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect()
}

pub fn part1(input: String) -> usize {
    let grid = construct_grid(input);
    println!("{:?}", grid);

    let height = grid.len();
    let width = grid[0].len();

    // Number of perimeter trees that are always visible
    let perimeter_trees = 2 * height + 2 * width - 4;
    println!("Perimeter trees: {perimeter_trees}");

    let mut visible_trees = HashSet::<(usize, usize)>::new();
    // Counting inner trees visible from the left
    for j in 1..height - 1 {
        let mut highest_tree = grid[j][0];
        for i in 1..width - 1 {
            let tree = grid[j][i];
            println!(
                "Highest: {}, current: {}, point: ({},{})",
                highest_tree, tree, j, i
            );
            if tree > highest_tree {
                println!(" Tree is taller and visible");
                visible_trees.insert((j, i));
                highest_tree = tree;
            }
        }
    }
    println!("{:?}", visible_trees);

    // Counting inner trees visible from the top
    for i in 1..width - 1 {
        let mut highest_tree = grid[0][i];
        for j in 1..height - 1 {
            let tree = grid[j][i];
            println!(
                "Highest: {}, current: {}, point: ({},{})",
                highest_tree, tree, j, i
            );
            if tree > highest_tree {
                println!(" Tree is taller and visible");
                visible_trees.insert((j, i));
                highest_tree = tree;
            }
        }
    }

    println!("{:?}", visible_trees);

    // Counting inner trees visible from the right
    for j in 1..height - 1 {
        let mut highest_tree = grid[j][width - 1];
        for i in (1..width - 1).rev() {
            let tree = grid[j][i];
            println!(
                "Highest: {}, current: {}, point: ({},{})",
                highest_tree, tree, j, i
            );
            if tree > highest_tree {
                println!(" Tree is taller and visible");
                visible_trees.insert((j, i));
                highest_tree = tree;
            }
        }
    }
    println!("{:?}", visible_trees);

    // Counting inner trees visible from the bottom
    for i in 1..width - 1 {
        let mut highest_tree = grid[height - 1][i];
        for j in (1..height - 1).rev() {
            let tree = grid[j][i];
            println!(
                "Highest: {}, current: {}, point: ({},{})",
                highest_tree, tree, j, i
            );
            if tree > highest_tree {
                println!(" Tree is taller and visible");
                visible_trees.insert((j, i));
                highest_tree = tree;
            }
        }
    }

    println!("{:?}", visible_trees);

    visible_trees.len() + perimeter_trees
}

#[allow(unused_variables)]
pub fn part2(input: String) -> u32 {
    todo!()
}
