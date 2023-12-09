
fn main() {
    let start_time = std::time::Instant::now();
    let input = include_str!("../../data/day_9.txt");

    let result: i64 = input.lines()
        .map(|str| {
            let mut nums = str.split_whitespace()
                .map(|s| s.trim().parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            let mut len = 0;
            let mut zeros = false;
            while !zeros {
                len += 1;
                zeros = true;
                for i in (len..nums.len()).rev() {
                    nums[i] -= nums[i-1];
                    zeros &= nums[i] == 0;
                }
            }
            // 'nums[..len]' is the left triangle side
            let r = nums[..len].iter()
                .rev()
                .fold(0, |acc, n| {
                    n - acc
                });
            r
        })
        .sum();

    let elapsed_time = start_time.elapsed().as_secs_f64() * 1e3;
    println!("{} ({:.3}ms)", result, elapsed_time); // 1087
}