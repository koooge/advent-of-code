pub fn solve_part1(inputs: &[String]) -> usize {
    let mut ret = 0;
    let mut fresh_ranges: Vec<(usize, usize)> = vec![];

    for line in inputs {
        if line.is_empty() {
            continue;
        }
        if line.contains('-') {
            let (a, b) = line
                .split_once('-')
                .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                .unwrap();
            fresh_ranges.push((a, b));
            continue;
        }

        let n: usize = line.parse().unwrap();
        for (begin, end) in &fresh_ranges {
            if n >= *begin && n <= *end {
                ret += 1;
                break;
            }
        }
    }

    ret
}

pub fn solve_part2(inputs: &[String]) -> usize {
    let mut fresh_ids: Vec<(usize, usize)> = vec![];

    for line in inputs {
        if !line.contains('-') {
            continue;
        }

        let (a, b) = line
            .split_once('-')
            .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
            .unwrap();
        fresh_ids.push((a, b));
    }
    fresh_ids.sort_by_key(|x| x.0);

    let mut ret = 0;
    let mut prev_end = 0;
    for (begin, end) in fresh_ids {
        if prev_end >= end {
            continue;
        }

        if prev_end >= begin {
            ret += end - prev_end;
        } else {
            ret += end - begin + 1;
        }
        prev_end = end;
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_file;

    #[test]
    fn day05_part1_case1() {
        let inputs = read_file("./src/day05/test1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 3);
    }

    #[test]
    fn part1() {
        let inputs = read_file("./src/day05/input1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 821);
    }

    #[test]
    fn day05_part2_case1() {
        let inputs = read_file("./src/day05/test1.txt");
        let result = solve_part2(&inputs);
        assert_eq!(result, 14);
    }

    #[test]
    fn part2() {
        let inputs = read_file("./src/day05/input1.txt");
        let result = solve_part2(&inputs);
        assert_eq!(result, 344771884978261);
    }
}
