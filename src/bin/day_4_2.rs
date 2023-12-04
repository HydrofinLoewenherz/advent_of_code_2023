use std::collections::{HashMap, HashSet};
use std::ops::{AddAssign};

fn main() {
    let start = std::time::Instant::now();

    let cards = include_str!("../../data/day_4.txt");

    let mut amounts: HashMap<usize, i32> = HashMap::new();
    let mut cards_n = 0;

    for (ci, card) in cards.trim().lines().enumerate() {
        cards_n = ci;
        let i = card.find(':').unwrap();
        let j = card.find('|').unwrap();

        // get winning numbers
        let winning: HashSet<i32> = card[(i+1)..j] // remove leading whitespace
            .split_whitespace()
            .map(|s| s.trim().parse::<i32>().unwrap())
            .collect();

        // get matches
        let matches = card[(j+1)..] // remove leading whitespace
            .split_whitespace()
            .map(|s| s.trim().parse::<i32>().unwrap())
            .filter(|n| winning.contains(n))
            .count();

        // increment amounts
        let cm = *amounts.entry(ci).or_insert(1);
        for m in 1..=matches {
            amounts
                .entry(ci + m)
                .or_insert(1)
                .add_assign(cm);
        }
    }

    let sum: i32 = amounts.iter()
        .filter(|(ci, _cv)| ci.le(&&cards_n))
        .map(|(_ci, cv)| cv)
        .sum();

    let elapsed = start.elapsed().as_secs_f64() * 1e3;
    println!("{} ({:.3}ms)", sum, elapsed); // 12263631
}
