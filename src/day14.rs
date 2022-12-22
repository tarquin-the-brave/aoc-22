use std::collections::HashSet;

#[derive(Clone, Copy)]
enum Cell {
    Empty,
    Rock,
    Sand,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Cell::Empty => write!(f, "."),
            Cell::Rock => write!(f, "#"),
            Cell::Sand => write!(f, "o"),
        }
    }
}

fn populate_grid(input: &str) -> (Vec<Vec<Cell>>, usize, usize, usize) {
    use itertools::Itertools as _;
    let mut ymax = 0;
    let mut xmax = 500;
    let mut xmin = 500;
    let paths: Vec<Vec<(usize, usize)>> = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|coord| {
                    let (i, j) = coord
                        .split(',')
                        .map(|s| s.parse().unwrap())
                        .collect_tuple()
                        .unwrap();
                    if i < xmin {
                        xmin = i;
                    }
                    if i > xmax {
                        xmax = i;
                    }
                    if j > ymax {
                        ymax = j;
                    }
                    (i, j)
                })
                .collect()
        })
        .collect();

    let mut grid = Vec::new();
    for _ in 0..ymax + 2 {
        grid.push([Cell::Empty].repeat(xmax + 2));
    }

    for path in &paths {
        for (i, j) in path_points(path) {
            grid[j][i] = Cell::Rock;
        }
    }

    (grid, ymax, xmin, xmax)
}

fn path_points(path: &[(usize, usize)]) -> HashSet<(usize, usize)> {
    use itertools::Itertools as _;
    path.windows(2)
        .map(|window| window.iter().collect_tuple().unwrap())
        .map(|((start_x, start_y), (end_x, end_y))| {
            if start_x == end_x {
                if end_y > start_y {
                    *start_y..=*end_y
                } else {
                    *end_y..=*start_y
                }
                .map(|j| (*start_x, j))
                .collect::<Vec<_>>()
            } else if start_y == end_y {
                if end_x > start_x {
                    *start_x..=*end_x
                } else {
                    *end_x..=*start_x
                }
                .map(|i| (i, *start_y))
                .collect::<Vec<_>>()
            } else {
                panic!("Not a line!")
            }
        })
        .flatten()
        .collect()
}

fn print_grid(grid: &Vec<Vec<Cell>>, ymax: usize, xmin: usize, xmax: usize) {
    for j in 0..ymax + 1 {
        for i in xmin - 1..xmax + 2 {
            print!("{}", grid[j][i]);
        }
        println!();
    }
}

pub fn part1(input: String) -> usize {
    let (mut grid, ymax, xmin, xmax) = populate_grid(&input);
    print_grid(&grid, ymax, xmin, xmax);
    7
}

#[allow(unused_variables)]
pub fn part2(input: String) -> usize {
    todo!()
}
