use std::collections::HashSet;

fn day6_general(input: String, window_size: usize) -> usize {
    let chars = input.chars().collect::<Vec<_>>();
    chars
        .windows(window_size)
        .enumerate()
        .find_map(|(idx, window)| {
            if window.iter().collect::<HashSet<_>>().len() == window.len() {
                Some(idx + window_size)
            } else {
                None
            }
        })
        .unwrap()
}

pub fn part1(input: String) -> usize {
    day6_general(input, 4)
}

pub fn part2(input: String) -> usize {
    day6_general(input, 14)
}
