use std::collections::HashMap;
use rayon::prelude::*;

#[derive(Debug, Default)]
struct RangeMap {
    src_start: i64,
    dst_start: i64,
    range: i64,
}

impl RangeMap {
    fn map(&self, val: i64) -> Option<i64> {
        let delta = val - self.src_start;
        if val >= self.src_start && delta <= self.range {
            return Some(self.dst_start + delta)
        }
        None
    }
}

fn main() {
    let start = std::time::Instant::now();
    let almanac = include_str!("../../data/day_5.txt");

    let mut lines_iter = almanac.lines();
    let pattern = regex::Regex::new(r"([a-z]+)-to-([a-z]+) map:").unwrap();

    // parse seeds
    let mut vals: Vec<i64> = lines_iter.next().unwrap()["seeds: ".len()..]
        .split_whitespace()
        .map(|s| s.trim().parse::<i64>().unwrap())
        .collect();
    let mut vals_type = "seed";
    lines_iter.next(); // skip empty line

    // parse maps
    let mut maps: HashMap<String, (String, Vec<RangeMap>)> = HashMap::new();
    while let Some(line) = lines_iter.next() {
        let from_to = pattern.captures(line).unwrap();
        let mut ranges: Vec<RangeMap> = Vec::new();
        for line in lines_iter.by_ref() {
            // break on empty line (or file end)
            if line.is_empty() {
                break
            }
            let digits: Vec<i64> = line
                .split_whitespace()
                .filter_map(|s| s.trim().parse::<i64>().ok())
                .collect();
            ranges.push(RangeMap{
                src_start: digits[1],
                dst_start: digits[0],
                range: digits[2],
            })
        }
        maps.insert(from_to[1].to_string(), (from_to[2].to_string(), ranges));
    }

    // apply maps until type matches 'location'
    while vals_type != "location" {
        let (to, maps) = &maps[vals_type];
        vals_type = to;

        vals.par_iter_mut().for_each(|val| {
            if let Some(v) = maps.iter().find_map(|m| m.map(*val)) {
                *val = v
            }
        });

        /*for i in 0..vals.len() {
            if let Some(v) = maps.iter().find_map(|m| m.map(vals[i])) {
                vals[i] = v
            }
        }*/
    }

    let result = vals.iter().min().unwrap();
    let elapsed = start.elapsed().as_secs_f64() * 1e3;
    println!("{} ({:.3}ms)", result, elapsed); // 218513636
}