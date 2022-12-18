fn totals(input: String) -> std::collections::BTreeSet<u32> {
    let mut totals = std::collections::BTreeSet::new();

    let mut total = 0;
    for line in input.lines() {
        if line.is_empty() {
            totals.insert(total);
            total = 0;
            continue;
        }
        total += line
            .parse::<u32>()
            .expect("could not parse line into number");
    }

    // insert once more incase input file didn't end with blank line.
    totals.insert(total);

    totals
}

pub fn part1(input: String) -> u32 {
    *totals(input).last().unwrap()
}

pub fn part2(input: String) -> u32 {
    let mut totals = totals(input);

    let foo = totals.pop_last().unwrap();
    let bar = totals.pop_last().unwrap();
    let baz = totals.pop_last().unwrap();

    foo + bar + baz
}
