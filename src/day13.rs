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

#[allow(unused_variables)]
pub fn part2(input: String) -> usize {
    todo!()
}
