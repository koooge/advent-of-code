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

    // #[test]
    // fn part2_case1() {
    //     let inputs = read_file("./src/day08/test1.txt");
    //     let result = solve_part2(&inputs);
    //     assert_eq!(result, 6);
    // }

    // #[test]
    // fn part2() {
    //     let inputs = read_file("./src/day08/input1.txt");
    //     let result = solve_part2(&inputs);
    //     assert_eq!(result, 1);
    // }
}
