use std::collections::HashMap;
use std::collections::HashSet;

pub fn solve_part1(inputs: &[String]) -> usize {
    let mut antenna: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for (i, input) in inputs.iter().enumerate() {
        for (j, c) in input.chars().enumerate() {
            if c == '.' {
                continue;
            }
            antenna.entry(c).or_default().push((i, j));
        }
    }

    fn get_antinode(a: (usize, usize), b: (usize, usize)) -> Option<(usize, usize)> {
        let x = (2 * a.0).checked_sub(b.0);
        let y = (2 * a.1).checked_sub(b.1);
        match (x, y) {
            (Some(x), Some(y)) => Some((x, y)),
            _ => None,
        }
    }

    let mut freqs: HashSet<(usize, usize)> = HashSet::new();
    for values in antenna.values() {
        for (i, &a) in values.iter().enumerate() {
            for &b in values.iter().skip(i + 1) {
                if let Some(ab) = get_antinode(a, b) {
                    if ab.0 < inputs[0].len() && ab.1 < inputs.len() {
                        freqs.insert(ab);
                    }
                }
                if let Some(ba) = get_antinode(b, a) {
                    if ba.0 < inputs[0].len() && ba.1 < inputs.len() {
                        freqs.insert(ba);
                    }
                }
            }
        }
    }

    freqs.len()
}

pub fn solve_part2(inputs: &[String]) -> usize {
    let mut antenna: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for (i, input) in inputs.iter().enumerate() {
        for (j, c) in input.chars().enumerate() {
            if c == '.' {
                continue;
            }
            antenna.entry(c).or_default().push((i, j));
        }
    }

    fn get_antinodes(
        x_len: usize,
        y_len: usize,
        first: (usize, usize),
        second: (usize, usize),
    ) -> HashSet<(usize, usize)> {
        let mut ret = HashSet::new();
        let a = (second.1 as isize - first.1 as isize) as f64
            / (second.0 as isize - first.0 as isize) as f64;
        let b: f64 = first.1 as f64 - (a * first.0 as f64);
        let dx = second
            .0
            .checked_sub(first.0)
            .unwrap_or_else(|| first.0 - second.0);
        // left
        for i in (0..=first.0).rev().step_by(dx) {
            let y = a * i as f64 + b;
            if y >= 0.0 && y < y_len as f64 {
                ret.insert((i, y.round() as usize));
            }
        }
        // right
        for i in (second.0..x_len).step_by(dx) {
            let y = a * i as f64 + b;
            if y >= 0.0 && y < y_len as f64 {
                ret.insert((i, y.round() as usize));
            }
        }

        ret
    }

    let mut freqs: HashSet<(usize, usize)> = HashSet::new();
    for values in antenna.values() {
        for (i, &a) in values.iter().enumerate() {
            for &b in values.iter().skip(i + 1) {
                let anti_nodes = get_antinodes(inputs[0].len(), inputs.len(), a, b);
                freqs.extend(anti_nodes);
            }
        }
    }

    freqs.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_file;

    #[test]
    fn part1_case1() {
        let inputs = read_file("./src/day08/test1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 14);
    }

    #[test]
    fn part1() {
        let inputs = read_file("./src/day08/input1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 329);
    }

    #[test]
    fn part2_case1() {
        let inputs = read_file("./src/day08/test2.txt");
        let result = solve_part2(&inputs);
        assert_eq!(result, 9);
    }

    #[test]
    fn part2_case2() {
        let inputs = read_file("./src/day08/test1.txt");
        let result = solve_part2(&inputs);
        assert_eq!(result, 34);
    }

    #[test]
    fn part2() {
        let inputs = read_file("./src/day08/input1.txt");
        let result = solve_part2(&inputs);
        assert_eq!(result, 1190);
    }
}
