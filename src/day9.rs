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

/// Part 1 implemented a GameState using an approximation
/// of the game physics that worked for a two knot rope,
/// this is a fuller treatment of the game physics as
/// described in the problem description.
struct GameState2 {
    rope: Vec<(i32, i32)>,
    tail_visited: HashSet<(i32, i32)>,
}

impl GameState2 {
    fn new(length: u32) -> Self {
        let start = (0, 0);
        let mut rope = Vec::new();
        for _ in 0..length {
            rope.push(start.clone());
        }
        let mut tail_visited = HashSet::new();
        tail_visited.insert(start.clone());
        Self { rope, tail_visited }
    }

    fn move_head(&mut self, direction: &Direction) {
        use Direction::*;

        let mut knots = self.rope.iter_mut();
        let head = knots.next().unwrap();
        match direction {
            Up => *head = (head.0, head.1 + 1),
            Down => *head = (head.0, head.1 - 1),
            Left => *head = (head.0 - 1, head.1),
            Right => *head = (head.0 + 1, head.1),
        }

        let mut last_knot_moved_to = head.clone();

        for knot in knots {
            if !Self::move_knot(&last_knot_moved_to, knot) {
                break;
            }
            last_knot_moved_to = knot.clone();
        }
        self.tail_visited
            .insert(self.rope[self.rope.len() - 1].clone());
    }

    // move the knot if it needs to catch up with the previous
    // knot.  If the knot moves, returns true.
    fn move_knot(previous: &(i32, i32), knot: &mut (i32, i32)) -> bool {
        match (previous.0 - knot.0, previous.1 - knot.1) {
            // Above
            (0, 2) => knot.1 += 1,
            // Above-right / Right-above
            (1, 2) | (2, 1) | (2, 2) => {
                knot.0 += 1;
                knot.1 += 1
            }
            // Right
            (2, 0) => knot.0 += 1,
            // Right-down / Down-right
            (2, -1) | (1, -2) | (2, -2) => {
                knot.0 += 1;
                knot.1 -= 1
            }
            // Down
            (0, -2) => knot.1 -= 1,
            // Down-left / Left-down
            (-1, -2) | (-2, -1) | (-2, -2) => {
                knot.0 -= 1;
                knot.1 -= 1
            }
            // Left
            (-2, 0) => knot.0 -= 1,
            // Left-up / Up-left
            (-2, 1) | (-1, 2) | (-2, 2) => {
                knot.0 -= 1;
                knot.1 += 1
            }
            (x, y) => {
                if x.abs() > 1 || y.abs() > 1 {
                    panic!("Rope broke! x: {x}, y: {y}");
                }
                return false;
            }
        };
        true
    }

    fn tail_visited_count(&self) -> usize {
        self.tail_visited.len()
    }
}

pub fn part2(input: String) -> usize {
    use itertools::Itertools as _;
    let mut game = GameState2::new(10);
    for line in input.lines() {
        let (direction_unparsed, times_unparsed) = line.split(' ').collect_tuple().unwrap();
        let direction = Direction::parse(direction_unparsed);
        let times: u32 = times_unparsed.parse().unwrap();

        for _ in 0..times {
            game.move_head(&direction);
        }
    }
    println!("{:?}", game.rope);
    game.tail_visited_count()
}
