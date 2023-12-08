use std::collections::{HashMap, HashSet};

type Node = [u8; 3];

// from this test we learn how the routes are structured and can optimize the actual solution
// we expect that this internal structure was not communicated in the exercise but is intended!
fn main() {
    let input = include_str!("../../data/day_8.txt");

    let mut lines = input.lines();

    // parse instructions (left: 76, right: 82)
    let instructions = lines.next().unwrap().as_bytes();
    lines.next(); // skip empty line

    // parse nodes
    let mut starting_nodes = vec![];
    let nodes: HashMap<Node, (Node, Node)> = lines
        //.par_bridge()
        .map(|str| {
            // parse 'SGR = (JLL, VRV)'
            let bs = str.as_bytes();
            let from: Node = bs[0..3].try_into().unwrap();
            let to_left: Node = bs[7..10].try_into().unwrap();
            let to_right: Node = bs[12..15].try_into().unwrap();
            if from[2] == 65 { // ??A
                starting_nodes.push(from);
            }
            (from, (to_left, to_right))
        })
        .collect();

    // move starting nodes into loop
    let data: Vec<(usize, usize, Vec<usize>)> = starting_nodes
        .iter()
        .map(|starting_node| {
            // move starting nodes to ends of loops
            let mut current_node = *starting_node;
            let mut end_nodes = vec![];
            let mut seen: HashSet<(usize, Node)> = HashSet::new();
            let loop_data = instructions
                .iter()
                .cycle()
                .enumerate()
                .find_map(|(i, inst)| {
                    let (left, right) = nodes.get(&current_node).unwrap();
                    if *inst == 76 {
                        current_node = *left;
                    } else {
                        current_node = *right;
                    }
                    if current_node[2] == 90 {
                        end_nodes.push(i + 1)
                    }
                    seen.replace((i % instructions.len(), current_node))
                        .map(|(j, _)| (j, i)) // (loop_start, loop_len)
                })
                .unwrap(); // loop must exist at some point
            (loop_data.0, loop_data.1, end_nodes)
        })
        .collect();
    println!("data {:?}", data); // -> exactly one end per start and "symmetrical"
}