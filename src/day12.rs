use petgraph::graph::{NodeIndex, UnGraph};
use std::collections::{HashMap, HashSet};

fn parse_height(c: char) -> i8 {
    match c {
        'S' => 0,
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'E' => 27,
        _ => panic!("Unrecognised character"),
    }
}

pub fn part1(input: String) -> usize {
    let points = input
        .lines()
        .map(|line| line.chars().map(parse_height).collect::<Vec<i8>>())
        .collect::<Vec<Vec<i8>>>();

    // for line in &points {
    //     println!("{:?}", line);
    // }
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut found_start = false;
    let mut found_end = false;

    for j in 0..points.len() {
        for i in 0..points[0].len() {
            if points[j][i] == 0 {
                start = (i, j);
                found_start = true;
            } else if points[j][i] == 27 {
                end = (i, j);
                found_end = true;
            }
        }
    }

    assert!(found_start);
    assert!(found_end);

    println!("Start: {:?}, End: {:?}", start, end);

    // initiate undirected graph
    let mut g = UnGraph::<(usize, usize), u8>::new_undirected();
    let mut point_to_node_idx = HashMap::<(usize, usize), NodeIndex>::new();

    let ylen = points.len();
    let xlen = points[0].len();
    // load in all points as nodes and save off a mapping of value to node indices
    for j in 0..ylen {
        for i in 0..xlen {
            let point = (i, j);
            let node_index = g.add_node(point.clone());
            point_to_node_idx.insert(point, node_index);
        }
    }

    assert_eq!(ylen * xlen, point_to_node_idx.len());
    assert_eq!(ylen * xlen, g.node_count());

    // calculate edges by points with adjacent heights
    let mut edges = HashSet::<(NodeIndex, NodeIndex)>::new();

    let mut add_edge = |ii: usize, jj: usize, height: i8, node_idx: &NodeIndex| {
        if (points[jj][ii] - height).abs() < 2 {
            let other_node_idx = point_to_node_idx.get(&(ii, jj)).unwrap();
            if !edges.contains(&(*node_idx, *other_node_idx))
                && !edges.contains(&(*other_node_idx, *node_idx))
            {
                edges.insert((*node_idx, *other_node_idx));
                // println!(
                //     "Edge added from {:?} to {:?}",
                //     g.node_weight(*node_idx).unwrap(),
                //     g.node_weight(*other_node_idx).unwrap()
                // );
            }
        }
    };

    println!("\nAdding edges from inner points\n");

    // inner points
    for j in 1..ylen - 1 {
        for i in 1..xlen - 1 {
            let height = points[j][i];
            let node_idx = point_to_node_idx.get(&(i, j)).unwrap();

            add_edge(i + 1, j, height, node_idx);
            // add_edge(i + 1, j - 1, height, node_idx);
            add_edge(i, j - 1, height, node_idx);
            // add_edge(i - 1, j - 1, height, node_idx);
            add_edge(i - 1, j, height, node_idx);
            // add_edge(i - 1, j + 1, height, node_idx);
            add_edge(i, j + 1, height, node_idx);
            // add_edge(i + 1, j + 1, height, node_idx);
        }
    }

    println!("\nAdding edges from top edge\n");

    // top edge
    for i in 1..xlen - 1 {
        let height = points[0][i];
        let node_idx = point_to_node_idx.get(&(i, 0)).unwrap();
        add_edge(i - 1, 0, height, node_idx);
        add_edge(i, 1, height, node_idx);
        add_edge(i + 1, 0, height, node_idx);
    }

    println!("\nAdding edges from bottom edge\n");

    // bottom edge
    for i in 1..xlen - 1 {
        let height = points[ylen - 1][i];
        let node_idx = point_to_node_idx.get(&(i, ylen - 1)).unwrap();
        add_edge(i - 1, ylen - 1, height, node_idx);
        add_edge(i, ylen - 2, height, node_idx);
        add_edge(i + 1, ylen - 1, height, node_idx);
    }

    println!("\nAdding edges from left edge\n");

    // left edge
    for j in 1..ylen - 1 {
        let height = points[j][0];
        let node_idx = point_to_node_idx.get(&(0, j)).unwrap();
        add_edge(0, j - 1, height, node_idx);
        add_edge(1, j, height, node_idx);
        add_edge(0, j + 1, height, node_idx);
    }

    println!("\nAdding edges from right edge\n");

    // right edge
    for j in 1..ylen - 1 {
        let height = points[j][xlen - 1];
        let node_idx = point_to_node_idx.get(&(xlen - 1, j)).unwrap();
        add_edge(xlen - 1, j - 1, height, node_idx);
        add_edge(xlen - 2, j, height, node_idx);
        add_edge(xlen - 1, j + 1, height, node_idx);
    }

    // top left corner
    {
        let height = points[0][0];
        let node_idx = point_to_node_idx.get(&(0, 0)).unwrap();
        add_edge(1, 0, height, node_idx);
        add_edge(0, 1, height, node_idx);
    }

    // top right corner
    {
        let height = points[0][xlen - 1];
        let node_idx = point_to_node_idx.get(&(xlen - 1, 0)).unwrap();
        add_edge(xlen - 2, 0, height, node_idx);
        add_edge(xlen - 1, 1, height, node_idx);
    }

    // bottom right corner
    {
        let height = points[ylen - 1][xlen - 1];
        let node_idx = point_to_node_idx.get(&(xlen - 1, ylen - 1)).unwrap();
        add_edge(xlen - 2, ylen - 1, height, node_idx);
        add_edge(xlen - 1, ylen - 2, height, node_idx);
    }

    // bottom left corner
    {
        let height = points[ylen - 1][0];
        let node_idx = point_to_node_idx.get(&(0, ylen - 1)).unwrap();
        add_edge(0, ylen - 2, height, node_idx);
        add_edge(1, ylen - 1, height, node_idx);
    }

    // add all those edges into the graph
    for (from, to) in edges {
        // let from_p = g.node_weight(from).unwrap();
        // let to_p = g.node_weight(to).unwrap();

        // println!("Edge from: {:?}, to: {:?}", from_p, to_p);
        g.add_edge(from, to, 1);
    }

    println!("\nGraph has {} edges\n", g.edge_count());
    // a-star for shortest path from start to end
    let start_idx = point_to_node_idx.get(&start).unwrap();
    let end_idx = point_to_node_idx.get(&end).unwrap();

    let (steps, path) =
        petgraph::algo::astar(&g, *start_idx, |finish| finish == *end_idx, |_| 1, |_| 0).unwrap();

    path.into_iter()
        .for_each(|node| println!("{:?}", g.node_weight(node).unwrap()));

    steps
}

#[allow(unused_variables)]
pub fn part2(input: String) -> usize {
    todo!()
}
