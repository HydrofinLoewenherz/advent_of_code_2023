use std::cmp::{max, min};
use std::collections::HashMap;

#[derive(Debug, Default)]
struct RangeMap {
    src_start: i64,
    dst_start: i64,
    range: i64,
}

impl RangeMap {
    fn map(&self, (start, range): (i64, i64)) -> Vec<(i64, i64, bool)> {
        let mut mapped = Vec::new();
        // create unmapped
        if start < self.src_start {
            let s = start;
            let e = min(start + range, self.src_start);
            mapped.push((start, e - s, false));
        }
        if start + range > self.src_start + self.range {
            let s = max(self.src_start + self.range, start);
            let e = start + range;
            mapped.push((s, e - s, false));
        }
        // create mapped
        if max(start, self.src_start) < min(start + range, self.src_start + self.range) {
            let d = self.dst_start - self.src_start;
            let s = max(self.src_start, start) + d;
            let e = min(self.src_start + self.range, start + range) + d;
            mapped.push((s, e - s, true));
        }
        mapped
    }
}

fn main() {
    let start_time = std::time::Instant::now();
    let almanac = include_str!("../../data/day_5.txt");

    let mut lines_iter = almanac.lines();
    let pattern = regex::Regex::new(r"([a-z]+)-to-([a-z]+) map:").unwrap();

    // parse seeds
    let mut vals: Vec<(i64, i64)> = lines_iter.next().unwrap()["seeds: ".len()..]
        .split_whitespace()
        .map(|s| s.trim().parse::<i64>().unwrap())
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|c| (c[0], c[1]))
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

        let mut vals_next_type: Vec<(i64, i64)> = Vec::new();
        for map in maps {
            let mut vals_next_map: Vec<(i64, i64)> = Vec::new();
            for val in vals {
                for (start, range, ok) in map.map(val) {
                    if ok {
                        // try to find first that intersects to merge with
                        let min_found = vals_next_type.iter()
                            .enumerate()
                            .find(|(_, (s, r))| {
                                max(*s, start) < min(*s+*r, start+range)
                            });
                        match min_found {
                            Some((min_index, (s, r))) => {
                                let start = min(*s, start);
                                let end = max(*s+*r, start+range);
                                vals_next_type[min_index] = (start, end-start);
                            }
                            None => vals_next_type.push((start, range))
                        }
                    } else {
                        // merge above seems to be enough optimization, no second merge required
                        vals_next_map.push((start, range));
                    }
                }
            }
            vals = vals_next_map;
        }
        vals.append(&mut vals_next_type);
    }

    let result = vals.iter().min_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let elapsed = start_time.elapsed().as_secs_f64() * 1e3;
    println!("{} ({:.3}ms)", result, elapsed); // 81956384
}