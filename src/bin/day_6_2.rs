fn main() {
    let start_time = std::time::Instant::now();
    let input = include_str!("../../data/day_6.txt");

    // parse input
    let mut lines = input.lines();
    let time = lines.next().unwrap()["Time:".len()..]
        .replace(' ', "")
        .parse::<i64>().unwrap() as f64;
    let record = lines.next().unwrap()["Distance:".len()..]
        .replace(' ', "")
        .parse::<i64>().unwrap() as f64;

    // solve
    let tmp = f64::sqrt(time * time - 4.0 * record);
    let a = 0.5 * (time - tmp);
    let b = 0.5 * (time + tmp);
    let result = b.ceil() as i32 - a as i32 - 1;

    let elapsed_time = start_time.elapsed().as_secs_f64() * 1e3;
    println!("{} ({:.3}ms)", result, elapsed_time); // 34278221
}