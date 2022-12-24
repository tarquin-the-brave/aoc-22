use std::collections::HashMap;
#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Sensor,
    Beacon,
    Empty,
    Unknown,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Cell::Empty => write!(f, "#"),
            Cell::Beacon => write!(f, "B"),
            Cell::Sensor => write!(f, "S"),
            Cell::Unknown => write!(f, "."),
        }
    }
}

fn parse_input(input: &str) -> Vec<((isize, isize), (isize, isize))> {
    todo!()
}

fn manhattan_diff((x1, y1): &(usize, usize), (x2, y2): &(usize, usize)) -> usize {
    let xdiff = if x1 > x2 { x1 - x2 } else { x2 - x1 };
    let ydiff = if y1 > y2 { y1 - y2 } else { y2 - y1 };
    xdiff + ydiff
}

struct Points(HashMap<i32, HashMap<i32, Cell>>);

pub fn part1(input: String, check_row: usize) -> usize {
    let sb_pairs = parse_input(&input);

    todo!("use struct with hashmap inside for storing 2D points");
}

#[allow(unused_variables)]
pub fn part2(input: String) -> usize {
    todo!()
}
