fn main() {
    let start_time = std::time::Instant::now();
    let input = include_str!("../../data/day_6.txt");

    // parse input
    let mut lines = input.lines();
    let times = lines.next().unwrap()["Time:".len()..]
        .split_whitespace()
        .map(|s| s.trim().parse::<i64>().unwrap() as f64)
        .collect::<Vec<_>>();
    let records = lines.next().unwrap()["Distance:".len()..]
        .split_whitespace()
        .map(|s| s.trim().parse::<i64>().unwrap() as f64)
        .collect::<Vec<_>>();

    // solve
    let mut tmp = 0.0;
    let result = times.iter().zip(records).map(|(t, r)| {
        tmp = f64::sqrt(*t * *t - 4.0 * r);
        let a = 0.5 * (*t - tmp);
        let b = 0.5 * (*t + tmp);
        b.ceil() as i32 - a as i32 - 1
    }).reduce(|acc, e| acc * e).unwrap();

    let elapsed_time = start_time.elapsed().as_secs_f64() * 1e3;
    println!("{} ({:.3}ms)", result, elapsed_time); // 32076
}