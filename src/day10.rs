mod cpu {
    use std::collections::{HashMap, HashSet};
    pub struct CPU {
        cycle_count: u32,
        record_cycles: HashSet<u32>,
        recorded_values: HashMap<u32, i32>,
        registar: i32,
        display: Vec<char>,
    }

    impl CPU {
        pub fn new<I>(record_cycles: I) -> Self
        where
            I: Iterator<Item = u32>,
        {
            Self {
                cycle_count: 0,
                record_cycles: record_cycles.collect(),
                recorded_values: HashMap::new(),
                registar: 1,
                display: Vec::new(),
            }
        }

        fn cycle_clock(&mut self) {
            self.display.push(
                if (Into::<i64>::into(self.cycle_count % 40) - self.registar as i64).abs() > 1 {
                    '.'
                } else {
                    '#'
                },
            );
            self.cycle_count += 1;
            if self.record_cycles.contains(&self.cycle_count) {
                self.recorded_values.insert(self.cycle_count, self.registar);
            }
        }

        pub fn run_command(&mut self, command: Command) {
            use Command::*;
            match command {
                Noop => self.cycle_clock(),
                Addx(x) => {
                    self.cycle_clock();
                    self.cycle_clock();
                    self.registar += x;
                }
            }
        }

        pub fn score(&self) -> i64 {
            println!("{:?}", self.recorded_values);
            self.recorded_values
                .iter()
                .map(|(a, b)| Into::<i64>::into(*a) * *b as i64)
                .sum()
        }

        pub fn print_display(&self) {
            use itertools::Itertools as _;
            self.display
                .iter()
                .chunks(40)
                .into_iter()
                .map(|chunk| chunk.collect::<String>())
                .for_each(|line| println!("{line}"));
        }
    }

    pub enum Command {
        Noop,
        Addx(i32),
    }

    impl Command {
        pub fn parse<S: AsRef<str>>(s: S) -> Self {
            let args = s.as_ref().split(' ').collect::<Vec<_>>();
            if args.len() == 1 {
                assert_eq!(args[0], "noop");
                Command::Noop
            } else {
                assert_eq!(args[0], "addx");
                Command::Addx(args[1].parse().unwrap())
            }
        }
    }
}

pub fn part1(input: String) -> i64 {
    use cpu::{Command, CPU};
    let mut cpu_ = CPU::new([20, 60, 100, 140, 180, 220].into_iter());
    for line in input.lines() {
        cpu_.run_command(Command::parse(line));
    }
    cpu_.print_display();
    cpu_.score()
}

#[allow(unused_variables)]
pub fn part2(input: String) -> i64 {
    part1(input)
}
