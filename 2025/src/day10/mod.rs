use itertools::Itertools;

fn min_push(lights: &[bool], buttons: &[Vec<usize>]) -> usize {
    for n in 1..buttons.len() {
        for comb in buttons.iter().combinations(n) {
            let mut l = vec![false; lights.len()];
            for c in comb {
                for &n in c {
                    l[n] = !l[n];
                }
            }
            if l == lights {
                return n;
            }
        }
    }
    unreachable!()
}

pub fn solve_part1(inputs: &[String]) -> usize {
    let mut ret = 0;

    for line in inputs {
        let (a, b) = line.split_once(' ').unwrap();
        let lights: Vec<bool> = a
            .chars()
            .filter_map(|c| match c {
                '#' => Some(true),
                '.' => Some(false),
                _ => None,
            })
            .collect();
        let (s, _) = b.split_once('{').unwrap();
        let buttons: Vec<Vec<usize>> = s
            .split_whitespace()
            .map(|group| {
                group
                    .trim_matches(|c| c == '(' || c == ')')
                    .split(',')
                    .filter(|x| !x.is_empty())
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect();
        ret += min_push(&lights, &buttons);
    }

    ret
}

fn push_button(cur: &[usize], button: &[usize]) -> Vec<usize> {
    let mut ret = cur.to_vec();
    for &n in button {
        ret[n] += 1;
    }
    ret
}

fn min_push_2(requirements: &[usize], buttons: &[Vec<usize>]) -> usize {
    let mut states = vec![vec![0; requirements.len()]];
    let mut next_states = vec![];
    for i in 1.. {
        while let Some(cur) = states.pop() {
            for button in buttons {
                let next = push_button(&cur, button);
                if next == requirements {
                    return i;
                } else if next.iter().zip(requirements.iter()).all(|(a, b)| a <= b) {
                    next_states.push(next);
                }
            }
        }
        states = next_states;
        next_states = vec![];
    }
    unreachable!()
}

pub fn solve_part2(inputs: &[String]) -> usize {
    let mut ret = 0;

    for line in inputs {
        let (_, b) = line.split_once(' ').unwrap();
        let (s, c) = b.split_once('{').unwrap();
        let buttons: Vec<Vec<usize>> = s
            .split_whitespace()
            .map(|group| {
                group
                    .trim_matches(|c| c == '(' || c == ')')
                    .split(',')
                    .filter(|x| !x.is_empty())
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect();
        let requirements: Vec<usize> = c
            .split(',')
            .map(|s| {
                s.trim_matches(|c: char| c == '}' || c.is_whitespace())
                    .parse::<usize>()
                    .unwrap()
            })
            .collect();
        ret += min_push_2(&requirements, &buttons);
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_file;

    #[test]
    fn part1_case1() {
        let inputs = read_file("./src/day10/test1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 7);
    }

    #[test]
    fn part1() {
        let inputs = read_file("./src/day10/input1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 434);
    }

    #[test]
    fn part2_case1() {
        let inputs = read_file("./src/day10/test1.txt");
        let result = solve_part2(&inputs);
        assert_eq!(result, 33);
    }

    // #[test]
    // fn part2() {
    //     let inputs = read_file("./src/day10/input1.txt");
    //     let result = solve_part2(&inputs);
    //     assert_eq!(result, 434);
    // }
}
