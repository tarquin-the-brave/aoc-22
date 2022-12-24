use std::collections::HashSet;

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

pub fn part1(input: String, check_row: i32) -> usize {
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

#[allow(unused_variables)]
pub fn part2(input: String) -> usize {
    todo!()
}
