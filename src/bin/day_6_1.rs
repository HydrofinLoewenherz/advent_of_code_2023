fn main() {
    let start_time = std::time::Instant::now();
    let input = include_str!("../../data/day_6.txt");

    // parse input
    let mut lines = input.lines();
    let times = lines.next().unwrap()["Time:".len()..]
        .trim()
        .split_whitespace()
        .map(|s| s.trim().parse::<i64>().unwrap() as f64)
        .collect::<Vec<_>>();
    let records = lines.next().unwrap()["Distance:".len()..]
        .trim()
        .split_whitespace()
        .map(|s| s.trim().parse::<i64>().unwrap() as f64)
        .collect::<Vec<_>>();

    // solve for upper/lower
    let result = times.iter().zip(records).map(|(t, r)| {
        let g = *t;
        let a = 0.5 * (g - f64::sqrt(g * g - 4.0 * r));
        let b = 0.5 * (f64::sqrt(g * g - 4.0 * r) + g);
        b.ceil() as i32 - a as i32 - 1
    }).reduce(|acc, e| acc * e).unwrap();

    let elapsed_time = start_time.elapsed().as_secs_f64() * 1e3;
    println!("{} ({:.3}ms)", result, elapsed_time); // 32076
}