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

fn find_largest(batteries: &[usize], begin: usize, end: usize) -> (usize, usize) {
    let mut ret = (0, begin);
    for (i, _) in batteries.iter().enumerate().take(end + 1).skip(begin) {
        if batteries[i] > ret.0 {
            ret = (batteries[i], i + 1);
        }
    }
    ret
}

pub fn solve_part2(inputs: &[String]) -> usize {
    let mut ret = 0;

    for line in inputs {
        let batteries: Vec<usize> = line
            .chars()
            .map(|c| c.to_string().parse::<usize>().unwrap())
            .collect();
        let mut largest = [0; 12];

        let mut ptr = 0;
        for (i, item) in largest.iter_mut().enumerate() {
            let (joltage, new_ptr) = find_largest(&batteries, ptr, batteries.len() - (12 - i));
            *item = joltage;
            ptr = new_ptr;
        }

        let n = largest[0] * 100000000000
            + largest[1] * 10000000000
            + largest[2] * 1000000000
            + largest[3] * 100000000
            + largest[4] * 10000000
            + largest[5] * 1000000
            + largest[6] * 100000
            + largest[7] * 10000
            + largest[8] * 1000
            + largest[9] * 100
            + largest[10] * 10
            + largest[11];
        ret += n;
    }

    ret
}

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

    #[test]
    fn day03_part2_case1() {
        let inputs = read_file("./src/day03/test1.txt");
        let result = solve_part2(&inputs);
        assert_eq!(result, 3121910778619);
    }

    #[test]
    fn part2() {
        let inputs = read_file("./src/day03/input1.txt");
        let result = solve_part2(&inputs);
        assert_eq!(result, 167549941654721);
    }
}
