use std::collections::HashSet;

struct GameState {
    head: (i32, i32),
    tail: (i32, i32),
    tail_visited: HashSet<(i32, i32)>,
}

impl GameState {
    fn new() -> Self {
        let start = (0, 0);
        let mut tail_visited = HashSet::new();
        tail_visited.insert(start.clone());
        Self {
            head: start.clone(),
            tail: start,
            tail_visited,
        }
    }

    fn move_head(&mut self, direction: &Direction) {
        use Direction::*;
        let old_head = self.head.clone();

        match direction {
            Up => self.head = (self.head.0, self.head.1 + 1),
            Down => self.head = (self.head.0, self.head.1 - 1),
            Left => self.head = (self.head.0 - 1, self.head.1),
            Right => self.head = (self.head.0 + 1, self.head.1),
        }

        if (self.head.0 - self.tail.0).abs() > 1 || (self.head.1 - self.tail.1).abs() > 1 {
            self.tail = old_head.clone();
            self.tail_visited.insert(old_head);
        }
    }

    fn tail_visited_count(&self) -> usize {
        self.tail_visited.len()
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn parse<S: AsRef<str>>(s: S) -> Self {
        use Direction::*;
        match s.as_ref() {
            "U" => Up,
            "D" => Down,
            "L" => Left,
            "R" => Right,
            _ => panic!("Unrecognised direction character"),
        }
    }
}

pub fn part1(input: String) -> usize {
    use itertools::Itertools as _;
    let mut game = GameState::new();
    for line in input.lines() {
        let (direction_unparsed, times_unparsed) = line.split(' ').collect_tuple().unwrap();
        let direction = Direction::parse(direction_unparsed);
        let times: u32 = times_unparsed.parse().unwrap();

        for _ in 0..times {
            game.move_head(&direction);
        }
    }
    game.tail_visited_count()
}

#[allow(unused_variables)]
pub fn part2(input: String) -> u32 {
    todo!()
}
