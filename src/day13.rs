#[derive(Clone, PartialEq, Debug)]
enum Data {
    List(Vec<Data>),
    Int(u64),
}

impl Data {
    fn from_json(json: &serde_json::Value) -> Self {
        use serde_json::Value::*;
        match json {
            Number(n) => Self::Int(n.as_u64().expect("Not a natural number")),
            Array(list) => Self::List(list.iter().map(Self::from_json).collect::<Vec<Data>>()),
            _ => panic!("Not an int or list"),
        }
    }

    fn correct_order(&self, other: &Data) -> Option<bool> {
        use Data::*;
        match (self, other) {
            (Int(me), Int(them)) => {
                if me == them {
                    None
                } else {
                    Some(me < them)
                }
            }
            (Int(me), them) => List(vec![Int(*me)]).correct_order(them),
            (me, Int(them)) => me.correct_order(&List(vec![Int(*them)])),
            (List(me), List(them)) => {
                for (me, them) in me.iter().zip(them.iter()) {
                    if let Some(ans) = me.correct_order(them) {
                        return Some(ans);
                    }
                }
                if me.len() == them.len() {
                    None
                } else {
                    Some(me.len() < them.len())
                }
            }
        }
    }
}

// impl std::fmt::Display for Data {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         match self {
//             Self::Int(n) => n.fmt(f),
//             Self::List(list) => format!("{list}").fmt(f),
//         }
//     }
// }

pub fn part1(input: String) -> usize {
    use itertools::Itertools as _;
    input
        .split("\n\n")
        .map(|pair| {
            pair.lines()
                .map(|line| Data::from_json(&serde_json::from_str(line).unwrap()))
                .collect_tuple()
                .unwrap()
        })
        .zip(1..)
        .filter_map(|((left, right), n)| {
            if left
                .correct_order(&right)
                .expect("Couldn't order top level packet")
            {
                Some(n)
            } else {
                None
            }
        })
        .sum()
}

pub fn part2(input: String) -> usize {
    use itertools::Itertools as _;
    let div2 = Data::from_json(&serde_json::from_str("[[2]]").unwrap());
    let div6 = Data::from_json(&serde_json::from_str("[[6]]").unwrap());
    input
        .lines()
        .filter(|line| *line != "")
        .map(|line| Data::from_json(&serde_json::from_str(line).unwrap()))
        .chain(vec![div2.clone(), div6.clone()].into_iter())
        .sorted_by(|a, b| {
            use std::cmp::Ordering::*;
            match a.correct_order(&b) {
                Some(true) => Less,
                Some(false) => Greater,
                None => Equal,
            }
        })
        .zip(1..)
        .fold(1, |decoder_key, (packet, n)| {
            if packet == div2 || packet == div6 {
                decoder_key * n
            } else {
                decoder_key
            }
        })
}
