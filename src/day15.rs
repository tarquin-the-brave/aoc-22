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

pub fn part1(input: String, check_row: usize) -> usize {
    let sb_pairs = parse_input(&input);
    // big dock off offset to avoid negative numbers
    let offset = 5_000_000;
    let sb_pairs = sb_pairs.iter().map(|((xs, ys), (xb, yb))| {
        let a = (xs + offset) as usize;
        let b = (ys + offset) as usize;
        let c = (xb + offset) as usize;
        let d = (xb + offset) as usize;
        ((a, b), (c, d))
    });

    let offs = offset as usize;
    let mut grid = Vec::new();
    for _ in 0..offs * 2 {
        grid.push([Cell::Unknown].repeat(offs * 2));
    }

    for (sensor, beacon) in sb_pairs {
        let (xs, ys) = sensor;
        let (xb, yb) = beacon;

        grid[ys][xs] = Cell::Sensor;
        grid[yb][xb] = Cell::Beacon;

        let diff = manhattan_diff(&sensor, &beacon);

        for j in ys - diff..=ys + diff {
            for i in xs - diff..=xs + diff {
                if manhattan_diff(&sensor, &(i, j)) <= diff && grid[j][i] == Cell::Unknown {
                    grid[j][i] = Cell::Empty;
                }
            }
        }
    }

    grid[check_row + offs]
        .iter()
        .filter(|cell| **cell == Cell::Empty)
        .count()
}

#[allow(unused_variables)]
pub fn part2(input: String) -> usize {
    todo!()
}
