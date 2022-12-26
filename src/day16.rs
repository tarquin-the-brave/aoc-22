use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
struct Valve {
    flow_rate: u32,
    connections: Vec<String>,
}

fn parse_valves(input: &str) -> HashMap<String, Valve> {
    use regex::Regex;

    let re = Regex::new(r"^Valve ([A-Z]{2}) has flow rate=(\d*); tunnels? leads? to valves? (.*)$")
        .unwrap();
    input
        .lines()
        .map(|line| {
            let mut name_valve = (
                "".to_string(),
                Valve {
                    flow_rate: 0,
                    connections: vec![],
                },
            );
            for cap in re.captures_iter(&line) {
                name_valve = (
                    cap[1].to_string(),
                    Valve {
                        flow_rate: cap[2].parse::<u32>().unwrap(),
                        connections: cap[3].split(", ").map(str::to_string).collect(),
                    },
                );
            }
            name_valve
        })
        .collect()
}

pub fn part1(input: String) -> u32 {
    struct State {
        flow_rate: u32,
        total: u32,
        minutes_elapsed: u32,
        valve_id: String,
        open_valves: HashSet<String>,
    }

    impl State {
        fn start(valve_id: String) -> Self {
            Self {
                flow_rate: 0,
                total: 0,
                minutes_elapsed: 0,
                valve_id,
                open_valves: HashSet::new(),
            }
        }
    }

    let valves = parse_valves(&input);
    let mut queue = VecDeque::<State>::new();

    let mut tracking_mins = 0;

    queue.push_back(State::start("AA".to_string()));

    loop {
        let State {
            flow_rate,
            total,
            valve_id,
            minutes_elapsed,
            open_valves,
        } = queue.pop_front().unwrap();

        if minutes_elapsed > tracking_mins {
            println!("Elapsed: {}", minutes_elapsed);
            println!("Possible States: {}", queue.len() + 1);
            tracking_mins = minutes_elapsed;
        }

        if minutes_elapsed == 30 {
            break;
        }

        let total = total + flow_rate;
        let minutes_elapsed = minutes_elapsed + 1;
        let valve = valves.get(&valve_id).unwrap();

        if !open_valves.contains(&valve_id) && valve.flow_rate != 0 {
            // If the valves is closed and there is a non-zero flow rate, we might
            // want to open the valve as a next step.
            let mut open_valves = open_valves.clone();
            open_valves.insert(valve_id.clone());
            queue.push_back(State {
                total,
                minutes_elapsed,
                valve_id: valve_id.clone(),
                flow_rate: flow_rate + valve.flow_rate,
                open_valves,
            });
        }

        for next_valve_id in &valve.connections {
            let possible_next_state = State {
                flow_rate,
                total,
                minutes_elapsed,
                valve_id: next_valve_id.clone(),
                open_valves: open_valves.clone(),
            };

            // To avoid the space of possibilities exploding,
            // exclude the state if it won't beat any state already
            // queued.
            // I.e: only add new state if we can't find a queued state
            // which is at the same valve with a greater total and
            // flow rate.
            if let None = queue.iter().find(|state| {
                &state.valve_id == next_valve_id
                    && state.total >= total
                    && state.flow_rate >= flow_rate
            }) {
                queue.push_back(possible_next_state);
            }
        }
    }

    queue.iter().map(|state| state.total).max().unwrap()
}

pub fn part2(input: String) -> u32 {
    struct State {
        flow_rate: u32,
        total: u32,
        minutes_elapsed: u32,
        me_valve_id: String,
        elephant_valve_id: String,
        open_valves: HashSet<String>,
        previous_me_valve_id: String,
        previous_elephant_valve_id: String,
    }

    impl State {
        fn start(valve_id: String) -> Self {
            Self {
                flow_rate: 0,
                total: 0,
                minutes_elapsed: 0,
                me_valve_id: valve_id.clone(),
                elephant_valve_id: valve_id.clone(),
                open_valves: HashSet::new(),
                previous_me_valve_id: "foo".to_string(),
                previous_elephant_valve_id: "bar".to_string(),
            }
        }
    }

    let valves = parse_valves(&input);
    let mut queue = VecDeque::<State>::new();

    let mut tracking_mins = 0;

    queue.push_back(State::start("AA".to_string()));

    loop {
        let State {
            flow_rate,
            total,
            me_valve_id,
            elephant_valve_id,
            minutes_elapsed,
            open_valves,
            previous_me_valve_id,
            previous_elephant_valve_id,
        } = queue.pop_front().unwrap();

        if minutes_elapsed > tracking_mins {
            println!("Elapsed: {}", minutes_elapsed);
            println!("Possible States: {}", queue.len() + 1);
            tracking_mins = minutes_elapsed;
        }

        if minutes_elapsed == 26 {
            break;
        }

        let total = total + flow_rate;
        let minutes_elapsed = minutes_elapsed + 1;
        let me_valve = valves.get(&me_valve_id).unwrap();
        let elephant_valve = valves.get(&elephant_valve_id).unwrap();

        // in all cases where we're adding a possible state, do not add
        // if we can find a queued state where:
        // - set of elephant and my valves are the same,
        // - greater flow rate,
        // - greater total.
        //
        // Next possible states:
        // - Maybe I can open a valve, if so:
        //   - add ways elephant can move (filter)
        //   - maybe add elephant opening valve (filter)
        // - add ways I can move
        //   - ways elephant can move for each (filter)
        //   - maybe elephant open valve (filter)

        // me open
        if !open_valves.contains(&me_valve_id) && me_valve.flow_rate != 0 {
            let mut open_valves = open_valves.clone();
            open_valves.insert(me_valve_id.clone());

            // me open + elephant move
            for elephant_valve_id in &elephant_valve.connections {
                if elephant_valve_id == &previous_elephant_valve_id {
                    continue;
                }
                let possible_next_state = State {
                    total,
                    minutes_elapsed,
                    me_valve_id: me_valve_id.clone(),
                    elephant_valve_id: elephant_valve_id.clone(),
                    flow_rate: flow_rate + me_valve.flow_rate,
                    open_valves: open_valves.clone(),
                    previous_me_valve_id: previous_me_valve_id.clone(),
                    previous_elephant_valve_id: previous_elephant_valve_id.clone(),
                };
                if let None = queue.iter().find(|queued_state| {
                    let queued_valve_set =
                        vec![&queued_state.me_valve_id, &queued_state.elephant_valve_id]
                            .into_iter()
                            .collect::<HashSet<_>>();
                    let possible_next_valve_set = vec![
                        &possible_next_state.me_valve_id,
                        &possible_next_state.elephant_valve_id,
                    ]
                    .into_iter()
                    .collect::<HashSet<_>>();
                    queued_valve_set == possible_next_valve_set
                        && queued_state.total >= possible_next_state.total
                        && queued_state.flow_rate >= possible_next_state.flow_rate
                }) {
                    queue.push_back(possible_next_state);
                }
            }

            // me open + elephant open
            if !open_valves.contains(&elephant_valve_id) && elephant_valve.flow_rate != 0 {
                let mut open_valves = open_valves.clone();
                open_valves.insert(elephant_valve_id.clone());
                let possible_next_state = State {
                    total,
                    minutes_elapsed,
                    me_valve_id: me_valve_id.clone(),
                    elephant_valve_id: elephant_valve_id.clone(),
                    flow_rate: flow_rate + me_valve.flow_rate + elephant_valve.flow_rate,
                    open_valves: open_valves.clone(),
                    previous_me_valve_id: previous_me_valve_id.clone(),
                    previous_elephant_valve_id: previous_elephant_valve_id.clone(),
                };
                if let None = queue.iter().find(|queued_state| {
                    let queued_valve_set =
                        vec![&queued_state.me_valve_id, &queued_state.elephant_valve_id]
                            .into_iter()
                            .collect::<HashSet<_>>();
                    let possible_next_valve_set = vec![
                        &possible_next_state.me_valve_id,
                        &possible_next_state.elephant_valve_id,
                    ]
                    .into_iter()
                    .collect::<HashSet<_>>();
                    queued_valve_set == possible_next_valve_set
                        && queued_state.total >= possible_next_state.total
                        && queued_state.flow_rate >= possible_next_state.flow_rate
                }) {
                    queue.push_back(possible_next_state);
                }
            }
        }

        // me move
        for me_valve_id in &me_valve.connections {
            if me_valve_id == &previous_me_valve_id {
                continue;
            }
            // me move + elephant move
            for elephant_valve_id in &elephant_valve.connections {
                if elephant_valve_id == &previous_elephant_valve_id {
                    continue;
                }
                // we won't move to the same place...
                if me_valve_id == elephant_valve_id {
                    continue;
                }
                let possible_next_state = State {
                    total,
                    minutes_elapsed,
                    me_valve_id: me_valve_id.clone(),
                    elephant_valve_id: elephant_valve_id.clone(),
                    flow_rate,
                    open_valves: open_valves.clone(),
                    previous_me_valve_id: previous_me_valve_id.clone(),
                    previous_elephant_valve_id: previous_elephant_valve_id.clone(),
                };
                if let None = queue.iter().find(|queued_state| {
                    let queued_valve_set =
                        vec![&queued_state.me_valve_id, &queued_state.elephant_valve_id]
                            .into_iter()
                            .collect::<HashSet<_>>();
                    let possible_next_valve_set = vec![
                        &possible_next_state.me_valve_id,
                        &possible_next_state.elephant_valve_id,
                    ]
                    .into_iter()
                    .collect::<HashSet<_>>();
                    queued_valve_set == possible_next_valve_set
                        && queued_state.total >= possible_next_state.total
                        && queued_state.flow_rate >= possible_next_state.flow_rate
                }) {
                    queue.push_back(possible_next_state);
                }
            }

            // me move + elephant open
            if !open_valves.contains(&elephant_valve_id) && elephant_valve.flow_rate != 0 {
                let mut open_valves = open_valves.clone();
                open_valves.insert(elephant_valve_id.clone());
                let possible_next_state = State {
                    total,
                    minutes_elapsed,
                    me_valve_id: me_valve_id.clone(),
                    elephant_valve_id: elephant_valve_id.clone(),
                    flow_rate: flow_rate + elephant_valve.flow_rate,
                    open_valves: open_valves.clone(),
                    previous_me_valve_id: previous_me_valve_id.clone(),
                    previous_elephant_valve_id: previous_elephant_valve_id.clone(),
                };
                if let None = queue.iter().find(|queued_state| {
                    let queued_valve_set =
                        vec![&queued_state.me_valve_id, &queued_state.elephant_valve_id]
                            .into_iter()
                            .collect::<HashSet<_>>();
                    let possible_next_valve_set = vec![
                        &possible_next_state.me_valve_id,
                        &possible_next_state.elephant_valve_id,
                    ]
                    .into_iter()
                    .collect::<HashSet<_>>();
                    queued_valve_set == possible_next_valve_set
                        && queued_state.total >= possible_next_state.total
                        && queued_state.flow_rate >= possible_next_state.flow_rate
                }) {
                    queue.push_back(possible_next_state);
                }
            }
        }
    }

    queue.iter().map(|state| state.total).max().unwrap()
}
