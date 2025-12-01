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

pub fn solve_part2(inputs: &[String], p: usize) -> usize {
    let mut ret = 0;
    let mut ptr = p;

    for line in inputs {
        let (lr, rest) = line.split_at(1);
        let num: usize = rest.trim().parse().unwrap();
        ptr = match lr {
            "L" => {
                if ptr >= num {
                    let p = ptr - num;
                    if p == 0 {
                        ret += 1;
                    }
                    p
                } else {
                    ret += num / 100;
                    let n = num % 100;
                    let p = (100 + ptr - n) % 100;
                    if p == 0 || (ptr != 0 && n > ptr) {
                        ret += 1;
                    }
                    p
                }
            }
            "R" => {
                let sum = ptr + num;
                ret += sum / 100;
                sum % 100
            }
            _ => unreachable!("invalid input"),
        };
    }

    ret
}

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

    #[test]
    fn day01_part2_case1() {
        let inputs = read_file("./src/day01/test1.txt");
        let result = solve_part2(&inputs, 50);
        assert_eq!(result, 6);
    }

    #[test]
    fn day01_part2_case2() {
        let inputs: Vec<String> = vec![String::from("L687")];
        let result = solve_part2(&inputs, 94);
        assert_eq!(result, 6);
    }

    #[test]
    fn day01_part2_case3() {
        let inputs: Vec<String> = vec![String::from("L683")];
        let result = solve_part2(&inputs, 58);
        assert_eq!(result, 7);
    }

    #[test]
    fn part2() {
        let inputs = read_file("./src/day01/input1.txt");
        let result = solve_part2(&inputs, 50);
        assert_eq!(result, 6638);
    }
}
