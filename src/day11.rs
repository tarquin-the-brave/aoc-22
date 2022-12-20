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
    items_count: u32,
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
            items_count: 0,
        }
    }

    fn run_turn(&mut self) -> Vec<(usize, u32)> {
        let mut pass_to_monkeys = Vec::<(usize, u32)>::new();

        while let Some(worry) = self.items.pop_front() {
            let worry = match self.op {
                Op::Plus(x) => worry + x,
                Op::Multiply(x) => worry * x,
                Op::Square => worry * worry,
            } / 3;
            let to_monkey: usize = if worry % self.test_divisible_by == 0 {
                self.to_true
            } else {
                self.to_false
            };
            pass_to_monkeys.push((to_monkey, worry));
            self.items_count += 1;
        }

        pass_to_monkeys
    }
}

pub fn part1(input: String) -> u32 {
    use itertools::Itertools as _;
    let mut monkeys = input
        .lines()
        .chunks(7)
        .into_iter()
        .map(|chunk| Monkey::parse(chunk.map(|s| s.to_string()).collect()))
        .collect::<Vec<Monkey>>();
    println!("{:?}", monkeys);

    for round in 0..20 {
        for idx in 0..monkeys.len() {
            let updates = monkeys[idx].run_turn();
            for (idx, worry) in updates {
                monkeys[idx].items.push_back(worry);
            }
        }
        println!("Round: {}", round + 1);
        println!("{:?}", monkeys.iter().map(|m| &m.items).collect::<Vec<_>>());
    }

    let mut scores = monkeys
        .iter()
        .map(|monkey| monkey.items_count)
        .sorted()
        .collect::<Vec<_>>();

    scores.pop().unwrap() * scores.pop().unwrap()
}

#[allow(unused_variables)]
pub fn part2(input: String) -> usize {
    todo!()
}
