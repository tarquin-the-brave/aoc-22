struct Pair {
    min1: u32,
    max1: u32,
    min2: u32,
    max2: u32,
}

impl Pair {
    fn fully_contained(&self) -> bool {
        // 1st range engulfs the 2nd
        (self.min1 <= self.min2 && self.max1 >= self.max2)
        // 2nd contains 1st
        || (self.min2 <= self.min1 && self.max2 >= self.max1)
    }

    fn overlap(&self) -> bool {
        use std::collections::HashSet;
        let left = (self.min1..=self.max1).collect::<HashSet<_>>();
        let right = (self.min2..=self.max2).collect::<HashSet<_>>();
        !left.is_disjoint(&right)
    }

    fn parse<S: AsRef<str>>(s: S) -> Self {
        use itertools::Itertools as _;
        let (left, right) = s.as_ref().split(',').collect_tuple().unwrap();
        let (min1, max1) = left
            .split('-')
            .map(|n| n.parse().unwrap())
            .collect_tuple()
            .unwrap();
        let (min2, max2) = right
            .split('-')
            .map(|n| n.parse().unwrap())
            .collect_tuple()
            .unwrap();

        Self {
            min1,
            max1,
            min2,
            max2,
        }
    }
}

pub fn part1(input: String) -> u32 {
    input
        .lines()
        .filter(|line| Pair::parse(line).fully_contained())
        .count() as u32
}
pub fn part2(input: String) -> u32 {
    input
        .lines()
        .filter(|line| Pair::parse(line).overlap())
        .count() as u32
}
