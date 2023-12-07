use rayon::prelude::*;

fn main() {
    let start_time = std::time::Instant::now();
    let input = include_str!("../../data/day_7.txt");

    let mut hands = input.lines()
        .par_bridge()
        .map(|str| {
            // parse line
            let (hand_str, bid_str) = str.split_once(" ").unwrap();
            let bid = bid_str.parse::<i64>().unwrap();

            // count occurrences
            let mut occ: [u8; 13] = [0; 13];
            let mut js = 0;
            let hand_norm: String = hand_str.chars()
                .map(|c| {
                    // map chars to be in order
                    match c {
                        'A' => { occ[12] += 1; 'E' },
                        'K' => { occ[11] += 1; 'D' },
                        'Q' => { occ[10] += 1; 'C' },
                        'J' => { js += 1; '0' }, // inc joker counter and use 'smallest' char
                        'T' => { occ[8] += 1; 'A' },
                        c => {
                            occ[c.to_digit(10).unwrap() as usize - 2] += 1;
                            c
                        },
                    }
                })
                .collect();

            // find strongest matching hand
            occ.sort();
            occ[12] += js; // use js to push already highest multiple
            // 5 0 0 0 0 ... -> 20 + 0 + 0 = 20
            // 4 1 0 0 0 ... -> 16 + 2 + 0 = 18
            // 3 2 0 0 0 ... -> 12 + 4 + 0 = 16
            // 3 1 1 0 0 ... -> 12 + 2 + 1 = 15
            // 2 2 1 0 0 ... ->  8 + 4 + 0 = 12
            // 2 1 1 1 0 ... ->  8 + 2 + 1 = 11
            // 1 1 1 1 1 ... ->  4 + 2 + 1 = 10
            let strength = 4 * occ[12] + 2 * occ[11] + occ[10];

            (strength, hand_norm, bid)
        })
        .collect::<Vec<_>>();
    hands.sort(); // tuples are build in sort order

    let result = hands.iter()
        .enumerate()
        .fold(0, |acc, (rank, (_, _, bid))| {
            acc + (rank + 1) as i64 * *bid
        });

    let elapsed_time = start_time.elapsed().as_secs_f64() * 1e3;
    println!("{} ({:.3}ms)", result, elapsed_time); // 249666369
}