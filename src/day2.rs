#[derive(Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn score(&self, opponent: &RPS) -> u32 {
        use RPS::*;
        let self_score = match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        };
        let win_score = match (self, opponent) {
            (Rock, Rock) => 3,
            (Rock, Paper) => 0,
            (Rock, Scissors) => 6,
            (Paper, Rock) => 6,
            (Paper, Paper) => 3,
            (Paper, Scissors) => 0,
            (Scissors, Rock) => 0,
            (Scissors, Paper) => 6,
            (Scissors, Scissors) => 3,
        };

        self_score + win_score
    }
}

impl From<&str> for RPS {
    fn from(value: &str) -> Self {
        use RPS::*;
        match value {
            "A" | "X" => Rock,
            "B" | "Y" => Paper,
            "C" | "Z" => Scissors,
            _ => panic!("Unrecognised character"),
        }
    }
}

fn parse_game(input: String) -> Vec<(RPS, RPS)> {
    input
        .lines()
        .into_iter()
        .map(|line| {
            let values = line.split(' ').map(Into::into).collect::<Vec<RPS>>();
            (values[0], values[1])
        })
        .collect()
}

pub fn part1(input: String) -> u32 {
    let mut score = 0;
    for (them, me) in parse_game(input) {
        score += me.score(&them)
    }
    score
}

enum DesiredOutcome {
    Lose,
    Draw,
    Win,
}

impl From<&str> for DesiredOutcome {
    fn from(value: &str) -> Self {
        use DesiredOutcome::*;
        match value {
            "X" => Lose,
            "Y" => Draw,
            "Z" => Win,
            _ => panic!("Unrecognised character"),
        }
    }
}

fn my_move(opponent: RPS, outcome: DesiredOutcome) -> RPS {
    use DesiredOutcome::*;
    use RPS::*;
    match (opponent, outcome) {
        (Rock, Lose) => Scissors,
        (Rock, Draw) => Rock,
        (Rock, Win) => Paper,
        (Paper, Lose) => Rock,
        (Paper, Draw) => Paper,
        (Paper, Win) => Scissors,
        (Scissors, Lose) => Paper,
        (Scissors, Draw) => Scissors,
        (Scissors, Win) => Rock,
    }
}

pub fn part2(input: String) -> u32 {
    input
        .lines()
        .into_iter()
        .map(|line| {
            let mut values = line.split(' ');
            let them: RPS = values.next().unwrap().into();
            let desired_outcome: DesiredOutcome = values.next().unwrap().into();
            let me = my_move(them, desired_outcome);
            me.score(&them)
        })
        .sum()
}
