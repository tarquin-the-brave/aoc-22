use std::collections::HashSet;

fn parse_input(input: &str) -> Vec<((i64, i64), (i64, i64))> {
    use regex::Regex;

    let re = Regex::new(r"Sensor at x=(.*), y=(.*): closest beacon is at x=(.*), y=(.*)").unwrap();
    input
        .lines()
        .map(|line| {
            let mut sb_pair = ((0, 0), (0, 0));
            for cap in re.captures_iter(&line) {
                sb_pair = (
                    (
                        cap[1].parse::<i64>().unwrap(),
                        cap[2].parse::<i64>().unwrap(),
                    ),
                    (
                        cap[3].parse::<i64>().unwrap(),
                        cap[4].parse::<i64>().unwrap(),
                    ),
                );
            }
            sb_pair
        })
        .collect()
}

fn manhattan_diff((x1, y1): &(i64, i64), (x2, y2): &(i64, i64)) -> i64 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

pub fn part1(input: String, check_row: i64) -> usize {
    let sb_pairs = parse_input(&input);

    let mut non_empty_points = HashSet::new();
    for (sensor, beacon) in &sb_pairs {
        non_empty_points.insert(sensor.clone());
        non_empty_points.insert(beacon.clone());
    }

    let diffs_and_sensors = sb_pairs
        .into_iter()
        .map(|(sensor, beacon)| (manhattan_diff(&sensor, &beacon), sensor))
        .collect::<Vec<_>>();

    let (xmin, xmax) = diffs_and_sensors
        .iter()
        .fold((0, 0), |(xmin, xmax), (diff, (sx, _))| {
            let mi = sx - diff;
            let min_ = if mi < xmin { mi } else { xmin };

            let ma = sx + diff;
            let max_ = if ma > xmax { ma } else { xmax };

            (min_, max_)
        });

    println!("xmin={}, xmax={}, range={}", xmin, xmax, xmax - xmin);
    let mut empty_count = 0;
    for i in xmin..=xmax {
        let point = (i, check_row);
        if non_empty_points.contains(&point) {
            continue;
        }
        if diffs_and_sensors
            .iter()
            .any(|(diff, sensor)| manhattan_diff(sensor, &point) <= *diff)
        {
            empty_count += 1;
        }
    }
    empty_count
}

pub fn part2(input: String, search_min: i64, search_max: i64) -> i64 {
    let diffs_and_sensors = parse_input(&input)
        .into_iter()
        .map(|(sensor, beacon)| (manhattan_diff(&sensor, &beacon), sensor))
        .collect::<Vec<_>>();

    for y in 0..=search_max {
        let empty_ranges = diffs_and_sensors
            .iter()
            .map(|(diff, (xs, ys))| {
                let ydiff = (y - ys).abs();
                let xdiff = diff - ydiff;
                xs - xdiff..=xs + xdiff
            })
            .collect::<HashSet<_>>();

        let mut x = search_min;
        while x <= search_max {
            if let Some(range) = empty_ranges.iter().find(|range| range.contains(&x)) {
                x = range.end() + 1;
            } else {
                return x * 4_000_000 + y;
            }
        }
    }
    panic!("no place found")
}
