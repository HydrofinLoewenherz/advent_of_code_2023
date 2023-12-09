
fn main() {
    let start_time = std::time::Instant::now();
    let input = include_str!("../../data/day_9.txt");

    let result: i64 = input.lines()
        .map(|str| {
            let mut sum = 0;
            let mut nums = str.split_whitespace()
                .map(|s| s.trim().parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            let mut len = nums.len();
            let mut zeros = false;
            while !zeros {
                sum += nums[len-1];
                len -= 1;
                zeros = true;
                for i in 0..len {
                    nums[i] = nums[i+1] - nums[i];
                    zeros &= nums[i] == 0;
                }
            }
            // sum ist the sum of the right triangle side
            sum
        })
        .sum();

    let elapsed_time = start_time.elapsed().as_secs_f64() * 1e3;
    println!("{} ({:.3}ms)", result, elapsed_time); // 1731106378
}