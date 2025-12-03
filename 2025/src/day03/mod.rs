pub fn solve_part1(inputs: &[String]) -> usize {
    let mut ret = 0;

    for line in inputs {
        let batteries: Vec<usize> = line
            .chars()
            .map(|c| c.to_string().parse::<usize>().unwrap())
            .collect();
        let mut largest = 0;

        for i in 0..batteries.len() - 1 {
            for j in i + 1..batteries.len() {
                let n = batteries[i] * 10 + batteries[j];
                if n > largest {
                    largest = n;
                }
            }
        }

        ret += largest;
    }
    ret
}

// pub fn solve_part2(inputs: &[String]) -> usize {

// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_file;

    #[test]
    fn day03_part1_case1() {
        let inputs = read_file("./src/day03/test1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 357);
    }

    #[test]
    fn part1() {
        let inputs = read_file("./src/day03/input1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 16858);
    }
}
