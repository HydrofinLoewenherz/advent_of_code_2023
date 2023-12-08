use std::collections::HashMap;
use rayon::prelude::*;
use num::integer::lcm;

type Node = [u8; 3];

fn main() {
    let start_time = std::time::Instant::now();
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

    let steps: Vec<usize> = starting_nodes
        .par_iter_mut()
        .map(|curr_node| {
            instructions
                .iter()
                .cycle()
                .take_while(|inst| {
                    let (left, right) = nodes.get(curr_node).unwrap();
                    if **inst == 76 {
                        *curr_node = *left;
                    } else {
                        *curr_node = *right;
                    }
                    curr_node[2] != 90 // ZZZ
                })
                .count() + 1
        })
        .collect();

    println!("steps {:?}", steps);
    let result = steps.iter().fold(1, |acc, s| lcm(acc, *s));

    let elapsed_time = start_time.elapsed().as_secs_f64() * 1e3;
    println!("{:?} ({:.3}ms)", result, elapsed_time); // 15690466351717
}