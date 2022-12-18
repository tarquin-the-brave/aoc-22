#![allow(unused_variables)]

struct Move {
    number_of_crates: usize,
    from_stack: usize,
    to_stack: usize,
}

impl Move {
    fn parse<S: AsRef<str>>(s: S) -> Self {
        let words = s.as_ref().split(' ').collect::<Vec<_>>();
        Self {
            number_of_crates: words[1].parse().unwrap(),
            // we're going to sift the stack numbers down by one so they can zero
            // index.
            from_stack: words[3].parse::<usize>().unwrap() - 1,
            to_stack: words[5].parse::<usize>().unwrap() - 1,
        }
    }
}

pub fn parse_stacks<'a, I>(lines: I) -> Vec<Vec<char>>
where
    I: Iterator<Item = &'a str> + Clone,
{
    let number_of_stacks = lines
        .clone()
        .find(|line| line.starts_with(" 1"))
        .unwrap()
        .trim()
        .split(' ')
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    println!("there are {} stacks", number_of_stacks);

    let mut stacks: Vec<Vec<char>> = Vec::with_capacity(number_of_stacks);
    for _ in 0..number_of_stacks {
        stacks.push(Vec::new());
    }

    let crates = lines
        .clone()
        .take_while(|line| !line.starts_with(" 1"))
        .map(|line| {
            let chars = line.chars().collect::<Vec<_>>();
            (0..number_of_stacks).map(move |n| match chars.get(4 * n + 1) {
                None | Some(' ') => None,
                Some(a) => Some(a.clone()),
            })
        })
        // need to collect to vec then iter so it's double ended and
        // can be reversed.
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .for_each(|line| {
            for (idx, maybe_crate) in line.enumerate() {
                if let Some(cr) = maybe_crate {
                    stacks[idx].push(cr);
                }
            }
        });

    stacks
}

pub fn part1(input: String) -> String {
    let lines = input.lines();

    let mut stacks = parse_stacks(lines.clone());

    for stack in &stacks {
        println!("{:?}", stack);
    }

    let moves = lines
        .clone()
        .filter(|line| line.starts_with("move"))
        .map(Move::parse);

    for move_ in moves {
        for _ in 0..move_.number_of_crates {
            let crate_ = stacks[move_.from_stack].pop().unwrap();
            stacks[move_.to_stack].push(crate_)
        }
    }

    println!();
    for stack in &stacks {
        println!("{:?}", stack);
    }

    stacks
        .iter()
        .map(|stack| stack.iter().last().unwrap())
        .collect::<String>()
}

pub fn part2(input: String) -> String {
    let lines = input.lines();

    let mut stacks = parse_stacks(lines.clone());

    for stack in &stacks {
        println!("{:?}", stack);
    }

    let moves = lines
        .clone()
        .filter(|line| line.starts_with("move"))
        .map(Move::parse);

    for move_ in moves {
        let mut temp_stack = Vec::new();
        for _ in 0..move_.number_of_crates {
            let crate_ = stacks[move_.from_stack].pop().unwrap();
            temp_stack.push(crate_)
        }
        for _ in 0..move_.number_of_crates {
            let crate_ = temp_stack.pop().unwrap();
            stacks[move_.to_stack].push(crate_)
        }
    }

    println!();
    for stack in &stacks {
        println!("{:?}", stack);
    }

    stacks
        .iter()
        .map(|stack| stack.iter().last().unwrap())
        .collect::<String>()
}
