use std::collections::HashSet;

fn parse_input(input: String) -> Vec<(HashSet<char>, HashSet<char>)> {
    input
        .lines()
        .map(|line| {
            let chars = line.chars();
            let chars2 = chars.clone();
            let (_, length) = chars.size_hint();
            let length = length.unwrap();
            let left_compartment = chars.take(length / 2).collect();
            let right_compartment = chars2.skip(length / 2).collect();
            (left_compartment, right_compartment)
        })
        .collect()
}

fn priority_value(item: &char) -> u32 {
    match item {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 27,
        'B' => 28,
        'C' => 29,
        'D' => 30,
        'E' => 31,
        'F' => 32,
        'G' => 33,
        'H' => 34,
        'I' => 35,
        'J' => 36,
        'K' => 37,
        'L' => 38,
        'M' => 39,
        'N' => 40,
        'O' => 41,
        'P' => 42,
        'Q' => 43,
        'R' => 44,
        'S' => 45,
        'T' => 46,
        'U' => 47,
        'V' => 48,
        'W' => 49,
        'X' => 50,
        'Y' => 51,
        'Z' => 52,
        _ => panic!("Unrecognised character"),
    }
}

pub fn part1(input: String) -> u32 {
    let mut total = 0;

    for (left_pocket, right_pocket) in parse_input(input) {
        // println!("Left: {:?}", left_pocket);
        // println!("Right: {:?}", right_pocket);
        let common_items = left_pocket.intersection(&right_pocket).collect::<Vec<_>>();
        // println!("{:?}", common_items);
        // Assuming only one common item from problem description
        assert_eq!(common_items.len(), 1);
        total += priority_value(common_items[0]);
    }

    total
}
