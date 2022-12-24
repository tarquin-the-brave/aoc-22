use std::collections::{HashMap, HashSet};
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

fn parse_input(input: &str) -> Vec<((i32, i32), (i32, i32))> {
    use regex::Regex;

    let re = Regex::new(r"Sensor at x=(.*), y=(.*): closest beacon is at x=(.*), y=(.*)").unwrap();
    input
        .lines()
        .map(|line| {
            let mut sb_pair = ((0, 0), (0, 0));
            for cap in re.captures_iter(&line) {
                sb_pair = (
                    (
                        cap[1].parse::<i32>().unwrap(),
                        cap[2].parse::<i32>().unwrap(),
                    ),
                    (
                        cap[3].parse::<i32>().unwrap(),
                        cap[4].parse::<i32>().unwrap(),
                    ),
                );
            }
            sb_pair
        })
        .collect()
}

fn manhattan_diff((x1, y1): &(i32, i32), (x2, y2): &(i32, i32)) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn manhattan_diamond((x, y): &(i32, i32), diff: i32) -> Vec<(i32, i32)> {
    assert!(diff > 0);
    let mut diamond = Vec::new();
    for j in y - diff..=y + diff {
        for i in x - diff..=x + diff {
            if manhattan_diff(&(*x, *y), &(i, j)) <= diff {
                diamond.push((i, j));
            }
        }
    }
    diamond
}

struct Points(HashMap<i32, HashMap<i32, Cell>>);

impl Points {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn get(&self, (x, y): &(i32, i32)) -> Cell {
        self.0
            .get(y)
            .and_then(|row| row.get(x))
            .cloned()
            .unwrap_or(Cell::Unknown)
    }

    fn insert(&mut self, (x, y): (i32, i32), item: Cell) {
        if let Some(row) = self.0.get_mut(&y) {
            row.insert(x, item);
        } else {
            self.0.insert(y, vec![(x, item)].into_iter().collect());
        }
    }

    fn insert_empty_checked(&mut self, coord: (i32, i32)) {
        if let Cell::Unknown = self.get(&coord) {
            self.insert(coord, Cell::Empty)
        }
    }

    fn sum_row_empties(&self, row: i32) -> Option<usize> {
        self.0
            .get(&row)
            .map(|row| row.iter().filter(|(x, item)| **item == Cell::Empty).count())
    }
}

pub fn part1(input: String, check_row: i32) -> usize {
    let sb_pairs = parse_input(&input);
    // let mut points = Points::new();

    let mut non_empty_points = HashSet::new();
    for (signal, beacon) in &sb_pairs {
        non_empty_points.insert(signal.clone());
        non_empty_points.insert(beacon.clone());
    }

    let sbd_triplets = sb_pairs
        .into_iter()
        .map(|(sensor, beacon)| (manhattan_diff(&sensor, &beacon), sensor, beacon))
        .collect::<Vec<_>>();

    let (xmin, xmax) = sbd_triplets
        .iter()
        .fold((0, 0), |(xmin, xmax), (diff, (sx, _), (_, _))| {
            let mi = sx - diff;
            let min_ = if mi < xmin { mi } else { xmin };

            let ma = sx + diff;
            let max_ = if ma > xmax { ma } else { xmax };

            (min_, max_)
        });

    println!("xmin={}, xmax={}", xmin, xmax);

    let mut empty_count = 0;
    for i in xmin..=xmax {
        let point = (i, check_row);
        if non_empty_points.contains(&point) {
            continue;
        }
        if sbd_triplets
            .iter()
            .any(|(diff, sensor, _)| manhattan_diff(sensor, &point) <= *diff)
        {
            empty_count += 1;
        }
    }
    // for (sensor, beacon) in &sb_pairs {
    //     points.insert(sensor.clone(), Cell::Sensor);
    //     points.insert(beacon.clone(), Cell::Beacon);
    // }

    // for (sensor, beacon) in &sb_pairs {
    //     let diff = manhattan_diff(sensor, beacon);
    //     for point in manhattan_diamond(sensor, diff) {
    //         points.insert_empty_checked(point);
    //     }
    // }

    // points
    //     .sum_row_empties(check_row)
    //     .expect("row does not exist to check")
    empty_count
}

#[allow(unused_variables)]
pub fn part2(input: String) -> usize {
    todo!()
}
