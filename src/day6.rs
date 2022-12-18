use std::collections::HashSet;

pub fn part1(input: String) -> usize {
    let chars = input.chars().collect::<Vec<_>>();
    chars
        .windows(4)
        .enumerate()
        .find_map(|(idx, window)| {
            if window.iter().collect::<HashSet<_>>().len() == window.len() {
                Some(idx + 4)
            } else {
                None
            }
        })
        .unwrap()
}

pub fn part2(input: String) -> u32 {
    todo!()
}
