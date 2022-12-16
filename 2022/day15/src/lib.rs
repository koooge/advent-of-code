use std::collections::HashSet;

pub fn solve_part1(inputs: &[String], target_y: i32) -> u32 {
    let mut nobeacon: HashSet<i32> = HashSet::new();
    let mut beacon_ony: HashSet<i32> = HashSet::new();

    for input in inputs {
        let strs: Vec<&str> = input.split(' ').collect();
        let sx: i32 = strs[2].split('=').collect::<Vec<&str>>()[1]
            .split(',')
            .collect::<Vec<&str>>()[0]
            .parse()
            .unwrap();
        let sy: i32 = strs[3].split('=').collect::<Vec<&str>>()[1]
            .split(':')
            .collect::<Vec<&str>>()[0]
            .parse()
            .unwrap();
        let bx: i32 = strs[8].split('=').collect::<Vec<&str>>()[1]
            .split(',')
            .collect::<Vec<&str>>()[0]
            .parse()
            .unwrap();
        let by: i32 = strs[9].split('=').collect::<Vec<&str>>()[1]
            .parse()
            .unwrap();
        if by == target_y {
            beacon_ony.insert(bx);
        }

        // is overlap then nobeacon
        let d = sx.abs_diff(bx) + sy.abs_diff(by);
        let d_target = sy.abs_diff(target_y);
        if d_target > d {
            continue;
        }

        for i in 0..=(d - d_target) {
            nobeacon.insert(sx + i as i32);
            nobeacon.insert(sx - i as i32);
        }
    }

    nobeacon.len() as u32 - beacon_ony.len() as u32
}

// use std::ops::Range;
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn new(start: i64, end: i64) -> Self {
        Self { start, end }
    }
    fn contains(&self, value: i64) -> bool {
        value >= self.start && value <= self.end
    }
}

fn merge_ranges(sorted_ranges: &[Range]) -> Vec<Range> {
    let mut result = Vec::with_capacity(sorted_ranges.len());
    let mut index = 0;
    let mut current = sorted_ranges.get(index).copied();
    loop {
        let next = sorted_ranges.get(index);
        index += 1;
        match (current, next) {
            (Some(r1), None) => {
                result.push(r1);
                return result;
            }
            (Some(r1), Some(r2)) if r1.contains(r2.start) => {
                current = Some(Range::new(r1.start, r1.end.max(r2.end)))
            }
            (Some(r1), Some(&r2)) => {
                if r1.end + 1 == r2.start {
                    current = Some(Range::new(r1.start, r1.end.max(r2.end)))
                } else {
                    current = Some(r2);
                    result.push(r1);
                }
            }
            (None, _) => return result,
        }
    }
}

pub fn solve_part2(inputs: &[String]) -> u64 {
    let mut min_x: u32 = 4000000;
    let mut min_y: u32 = 4000000;
    let mut max_x: u32 = 0;
    let mut max_y: u32 = 0;
    let mut sensor_data: Vec<(u32, u32, u32)> = Vec::new();

    for input in inputs {
        let strs: Vec<&str> = input.split(' ').collect();
        let sx: u32 = strs[2].split('=').collect::<Vec<&str>>()[1]
            .split(',')
            .collect::<Vec<&str>>()[0]
            .parse()
            .unwrap();
        let sy: u32 = strs[3].split('=').collect::<Vec<&str>>()[1]
            .split(':')
            .collect::<Vec<&str>>()[0]
            .parse()
            .unwrap();
        let bx: i32 = strs[8].split('=').collect::<Vec<&str>>()[1]
            .split(',')
            .collect::<Vec<&str>>()[0]
            .parse()
            .unwrap();
        let by: i32 = strs[9].split('=').collect::<Vec<&str>>()[1]
            .parse()
            .unwrap();

        let d = (sx as i32).abs_diff(bx) + (sy as i32).abs_diff(by);
        sensor_data.push((sx, sy, d));

        min_x = std::cmp::min(sx, min_x);
        min_y = std::cmp::min(sy, min_y);
        max_x = if sx <= 4000000 && sx > max_x {
            sx
        } else {
            max_x
        };
        max_y = if sy <= 4000000 && sy > max_y {
            sy
        } else {
            max_y
        };
    }

    for target_y in min_y..=max_y {
        let mut ranges: Vec<Range> = Vec::new();
        for data in sensor_data.iter() {
            let sx = data.0;
            let sy = data.1;
            let d = data.2;
            let d_target = sy.abs_diff(target_y);
            if d_target > d {
                continue;
            }
            let dx = d - d_target;

            let start = std::cmp::max(0, if sx > dx { sx - dx } else { 0 });
            let end = std::cmp::min(sx + dx, 4000000);
            // let range: Range<u32> = start..end;
            let range: Range = Range {
                start: start as i64,
                end: end as i64,
            };
            ranges.push(range);
        }
        ranges.sort_unstable_by_key(|r| r.start);
        let ranges = merge_ranges(&ranges);
        for r in ranges {
            if r.start != 0 || r.end != 4000000 && r.end <= max_x.into() {
                return (r.end as u64 + 1) * 4000000 + target_y as u64;
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1_case1() {
        let inputs = read_file("./src/test1.txt");
        let result = solve_part1(&inputs, 10);
        println!("{}", result);
        assert_eq!(result, 26);
    }

    #[test]
    fn part1() {
        let inputs = read_file("./src/input1.txt");
        let result = solve_part1(&inputs, 2000000);
        println!("{}", result);
        assert_eq!(result, 5166077);
    }

    #[test]
    fn part2_case1() {
        let inputs = read_file("./src/test1.txt");
        let result = solve_part2(&inputs);
        assert_eq!(result, 56000011);
    }

    #[test]
    fn part2() {
        let inputs = read_file("./src/input1.txt");
        let result = solve_part2(&inputs);
        assert_eq!(result, 13071206703981);
    }

    fn read_file(file_path: &str) -> Vec<String> {
        let contents = fs::read_to_string(file_path);
        let mut ret: Vec<String> = vec![];
        match contents {
            Ok(contents) => {
                for line in contents.lines() {
                    ret.push(line.to_string());
                }
            }
            Err(why) => eprintln!("{}", why),
        }
        ret
    }
}
