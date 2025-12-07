pub fn solve_part1(inputs: &[String]) -> usize {
    let mut ret = 0;
    let mut diagram: Vec<Vec<char>> = vec![];

    for line in inputs {
        diagram.push(line.chars().collect());
    }

    for y in 0..diagram.len() - 1 {
        for x in 0..diagram[y].len() {
            let c = diagram[y][x];
            if c != 'S' && c != '|' {
                continue;
            }

            if diagram[y + 1][x] == '^' {
                if x > 0 {
                    diagram[y + 1][x - 1] = '|';
                }
                if x < diagram[y + 1].len() - 1 {
                    diagram[y + 1][x + 1] = '|';
                }
                ret += 1;
            } else {
                diagram[y + 1][x] = '|';
            }
        }
    }

    ret
}

pub fn solve_part2(inputs: &[String]) -> usize {
    let mut diagram: Vec<Vec<char>> = vec![];
    let mut timelines: Vec<Vec<usize>> = vec![];
    for line in inputs {
        diagram.push(line.chars().collect());
        if let Some(idx) = line.find('S') {
            timelines.push(vec![idx]);
        }
    }

    for y in 0..diagram.len() - 1 {
        let mut new_timelines: Vec<Vec<usize>> = vec![];
        for timeline in timelines {
            let x = timeline[timeline.len() - 1];
            if diagram[y + 1][x] == '^' {
                if x > 0 {
                    let mut new_timeline = timeline.to_vec();
                    new_timeline.push(x - 1);
                    new_timelines.push(new_timeline);
                }
                if x < diagram[y + 1].len() - 1 {
                    let mut new_timeline = timeline.to_vec();
                    new_timeline.push(x + 1);
                    new_timelines.push(new_timeline);
                }
            } else {
                let mut new_timeline = timeline.to_vec();
                new_timeline.push(x);
                new_timelines.push(new_timeline);
            }
        }
        timelines = new_timelines;
    }

    timelines.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_file;

    #[test]
    fn day07_part1_case1() {
        let inputs = read_file("./src/day07/test1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 21);
    }

    #[test]
    fn part1() {
        let inputs = read_file("./src/day07/input1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 1555);
    }

    #[test]
    fn day07_part2_case1() {
        let inputs = read_file("./src/day07/test1.txt");
        let result = solve_part2(&inputs);
        assert_eq!(result, 40);
    }

    #[test]
    fn part2() {
        let inputs = read_file("./src/day07/input1.txt");
        let result = solve_part2(&inputs);
        assert_eq!(result, 1555);
    }
}
