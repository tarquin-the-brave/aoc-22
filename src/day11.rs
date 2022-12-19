use std::collections::VecDeque;

#[derive(Debug)]
enum Op {
    Plus(u32),
    Multiply(u32),
    Square,
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u32>,
    op: Op,
    test_divisible_by: u32,
    to_true: usize,
    to_false: usize,
}

impl Monkey {
    fn parse(input: Vec<String>) -> Self {
        let items = input[1].trim().split(':').collect::<Vec<_>>()[1]
            .trim()
            .split(", ")
            .map(|n| n.parse().unwrap())
            .collect();

        let op_line = input[2].trim().split(' ').collect::<Vec<_>>();
        let op_value_str = op_line[5];
        let op;
        if op_value_str == "old" {
            op = Op::Square;
        } else {
            let op_value = op_value_str.parse().unwrap();
            op = if op_line[4] == "+" {
                Op::Plus(op_value)
            } else {
                Op::Multiply(op_value)
            };
        }

        let test_divisible_by = input[3].trim().split(' ').collect::<Vec<_>>()[3]
            .parse()
            .unwrap();
        let to_true = input[4].trim().split(' ').collect::<Vec<_>>()[5]
            .parse()
            .unwrap();
        let _to_false = input[5].trim().split(' ').collect::<Vec<_>>()[5];
        let to_false = _to_false.parse().unwrap();
        Self {
            items,
            op,
            test_divisible_by,
            to_true,
            to_false,
        }
    }
}

pub fn part1(input: String) -> usize {
    use itertools::Itertools as _;
    let mut monkeys = input
        .lines()
        .chunks(7)
        .into_iter()
        .map(|chunk| Monkey::parse(chunk.map(|s| s.to_string()).collect()))
        .collect::<Vec<Monkey>>();
    println!("{:?}", monkeys);
    6
}

#[allow(unused_variables)]
pub fn part2(input: String) -> usize {
    todo!()
}
