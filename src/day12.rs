fn parse_height(c: char) -> u8 {
    match c {
        'S' => 0,
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
        'E' => 27,
        _ => panic!("Unrecognised character"),
    }
}

pub fn part1(input: String) -> usize {
    let points = input
        .lines()
        .map(|line| line.chars().map(parse_height).collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();

    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut found_start = false;
    let mut found_end = false;

    for j in 0..points.len() {
        for i in 0..points[0].len() {
            if points[j][i] == 0 {
                start = (i, j);
                found_start = true;
            } else if points[j][i] == 27 {
                end = (i, j);
                found_end = true;
            }
        }
    }

    assert!(found_start);
    assert!(found_end);

    // initiate undirected graph

    // load in all points as nodes

    // calculate edges by points with adjacent heights

    // a-star for shortest path from start to end

    7
}

#[allow(unused_variables)]
pub fn part2(input: String) -> usize {
    todo!()
}
