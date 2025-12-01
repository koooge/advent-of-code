pub fn solve_part1(inputs: &[String]) -> usize {
    let mut ret = 0;
    let mut ptr: usize = 50;

    for line in inputs {
        let (lr, rest) = line.split_at(1);
        let num: usize = rest.trim().parse().unwrap();
        ptr = match lr {
            "L" => {
                if ptr >= num {
                    ptr - num
                } else {
                    (100 + ptr - (num % 100)) % 100
                }
            }
            "R" => (ptr + num) % 100,
            _ => unreachable!("invalid input"),
        };
        if ptr == 0 {
            ret += 1;
        }
    }

    ret
}

// pub fn solve_part2(inputs: &[String]) -> isize {

// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_file;

    #[test]
    fn day01_part1_case1() {
        let inputs = read_file("./src/day01/test1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 3);
    }

    #[test]
    fn part1() {
        let inputs = read_file("./src/day01/input1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 1129);
    }
}
