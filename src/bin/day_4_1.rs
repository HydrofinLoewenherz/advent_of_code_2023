use std::collections::HashSet;

fn main() {
    let cards = include_str!("../../data/day_4.txt");

    let base: i32 = 2;
    let mut sum: i32 = 0;

    for card in cards.trim().lines() {
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

        // calculate card score
        if matches > 0 {
            sum += base.pow(matches as u32 - 1);
        }
    }

    println!("sum {sum}") // 23673
}
