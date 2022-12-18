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

    let height = grid.len();
    let width = grid[0].len();

    // Number of perimeter trees that are always visible
    let perimeter_trees = 2 * height + 2 * width - 4;

    let mut visible_trees = HashSet::<(usize, usize)>::new();
    // Counting inner trees visible from the left
    for j in 1..height - 1 {
        let mut highest_tree = grid[j][0];
        for i in 1..width - 1 {
            let tree = grid[j][i];
            if tree > highest_tree {
                visible_trees.insert((j, i));
                highest_tree = tree;
            }
        }
    }

    // Counting inner trees visible from the top
    for i in 1..width - 1 {
        let mut highest_tree = grid[0][i];
        for j in 1..height - 1 {
            let tree = grid[j][i];
            if tree > highest_tree {
                visible_trees.insert((j, i));
                highest_tree = tree;
            }
        }
    }

    // Counting inner trees visible from the right
    for j in 1..height - 1 {
        let mut highest_tree = grid[j][width - 1];
        for i in (1..width - 1).rev() {
            let tree = grid[j][i];
            if tree > highest_tree {
                visible_trees.insert((j, i));
                highest_tree = tree;
            }
        }
    }

    // Counting inner trees visible from the bottom
    for i in 1..width - 1 {
        let mut highest_tree = grid[height - 1][i];
        for j in (1..height - 1).rev() {
            let tree = grid[j][i];
            if tree > highest_tree {
                visible_trees.insert((j, i));
                highest_tree = tree;
            }
        }
    }

    visible_trees.len() + perimeter_trees
}

pub fn part2(input: String) -> usize {
    let grid = construct_grid(input);

    let height = grid.len();
    let width = grid[0].len();

    let mut top_score = 0;
    let mut ideal_spot = (0, 0);

    for j in 1..height - 1 {
        for i in 1..width - 1 {
            let tree = grid[j][i];

            // looking right +ve i
            let mut right_view = 0;
            for ii in i + 1..width {
                right_view += 1;
                if grid[j][ii] >= tree {
                    break;
                }
            }

            // looking down +ve j
            let mut down_view = 0;
            for jj in j + 1..height {
                down_view += 1;
                if grid[jj][i] >= tree {
                    break;
                }
            }

            // looking left -ve i
            let mut left_view = 0;
            for ii in (0..i).rev() {
                left_view += 1;
                if grid[j][ii] >= tree {
                    break;
                }
            }

            // looking up -ve j
            let mut up_view = 0;
            for jj in (0..j).rev() {
                up_view += 1;
                if grid[jj][i] >= tree {
                    break;
                }
            }

            let score = right_view * down_view * left_view * up_view;
            if score > top_score {
                ideal_spot = (j, i);
                top_score = score;
            }
        }
    }

    println!("Ideal spot: {:?}", ideal_spot);

    top_score
}
