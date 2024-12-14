pub fn solve_part1(inputs: &[String]) -> usize {
    let mut tokens: Vec<usize> = vec![];

    fn parse_line(s: &str) -> Vec<usize> {
        s.split(|c: char| !c.is_ascii_digit())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<usize>().unwrap())
            .collect()
    }

    for chunk in inputs.chunks(4) {
        let a = parse_line(&chunk[0]);
        let b = parse_line(&chunk[1]);
        let prize = parse_line(&chunk[2]);

        let mut won = false;
        for push_a in 0..=100 {
            let x0 = a[0] * push_a;
            let y0 = a[1] * push_a;
            for push_b in 0..=100 {
                let x = x0 + b[0] * push_b;
                let y = y0 + b[1] * push_b;
                if x == prize[0] && y == prize[1] {
                    tokens.push(push_a * 3 + push_b);
                    won = true;
                    break;
                }
            }
            if won {
                break;
            }
        }
    }

    tokens.iter().sum()
}

pub fn solve_part2(inputs: &[String]) -> usize {
    let mut tokens: Vec<usize> = vec![];

    fn parse_line(s: &str) -> Vec<usize> {
        s.split(|c: char| !c.is_ascii_digit())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<usize>().unwrap())
            .collect()
    }

    for chunk in inputs.chunks(4) {
        let a = parse_line(&chunk[0]);
        let b = parse_line(&chunk[1]);
        let prize: Vec<usize> = parse_line(&chunk[2])
            .iter()
            .map(|n| n + 10000000000000)
            .collect();

        fn push_button(
            remain: &[usize],
            a: &[usize],
            b: &[usize],
            pushed: (usize, usize),
        ) -> Option<(usize, usize)> {
            if remain[0] == 0 && remain[1] == 0 {
                return Some(pushed);
            }
            if remain[0] >= b[0] && remain[1] >= b[1] {
                let r = vec![remain[0] - b[0], remain[1] - b[1]];
                return push_button(&r, a, b, (pushed.0, pushed.1 + 1));
            }
            if remain[0] >= a[0] && remain[1] >= a[1] {
                let r = vec![remain[0] - a[0], remain[1] - a[1]];
                return push_button(&r, a, b, (pushed.0 + 1, pushed.1));
            }
            None
        }

        if let Some(pushed) = push_button(&prize, &a, &b, (0, 0)) {
            tokens.push(pushed.0 * 3 + pushed.1);
        }
    }

    tokens.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_file;

    #[test]
    fn part1_case1() {
        let inputs = read_file("./src/day13/test1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 480);
    }

    #[test]
    fn part1() {
        let inputs = read_file("./src/day13/input1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 30973);
    }

    // #[test]
    // fn part2_case1() {
    //     let inputs = read_file("./src/day13/test1.txt");
    //     let result = solve_part2(&inputs);
    //     assert_eq!(result, 480); // stack over flow
    // }

    // #[test]
    // fn part2() {
    //     let inputs = read_file("./src/day13/input1.txt");
    //     let result = solve_part2(&inputs);
    //     assert_eq!(result, 272673043446478);
    // }
}
