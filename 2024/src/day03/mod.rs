pub fn solve_part1(inputs: &[String]) -> isize {
    let mut ret = 0;

    for input in inputs {
        let mut i = 0;
        while i < input.len() - 8 {
            if &input[i..=i + 3] == "mul(" {
                i += 4;
                if let Some(j) = input[i..].find(')') {
                    let xy: Result<Vec<isize>, _> = input[i..i + j]
                        .split(',')
                        .map(|n| n.parse::<isize>())
                        .collect();
                    if let Ok(xy) = xy {
                        if xy.len() == 2 && xy[0] > 0 && xy[0] <= 999 && xy[1] > 0 && xy[1] <= 999 {
                            ret += xy[0] * xy[1];
                        }
                    }
                } else {
                    break;
                }
            }
            i += 1;
        }
    }

    ret
}

pub fn solve_part2(inputs: &[String]) -> isize {
    let mut ret = 0;

    let mut is_dont = false;
    for input in inputs {
        let mut i = 0;
        while i < input.len() - 8 {
            if &input[i..=i + 3] == "mul(" {
                i += 4;
                if let Some(j) = input[i..].find(')') {
                    let xy: Result<Vec<isize>, _> = input[i..i + j]
                        .split(',')
                        .map(|n| n.parse::<isize>())
                        .collect();
                    if let Ok(xy) = xy {
                        if !is_dont
                            && xy.len() == 2
                            && xy[0] > 0
                            && xy[0] <= 999
                            && xy[1] > 0
                            && xy[1] <= 999
                        {
                            ret += xy[0] * xy[1];
                        }
                    }
                } else {
                    break;
                }
            } else if &input[i..=i + 3] == "do()" {
                i += 3;
                is_dont = false;
            } else if &input[i..=i + 6] == "don't()" {
                i += 6;
                is_dont = true;
            }
            i += 1;
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_file;

    #[test]
    fn part1_case1() {
        let inputs = read_file("./src/day03/test1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 161);
    }

    #[test]
    fn part1() {
        let inputs = read_file("./src/day03/input1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 170068701);
    }

    #[test]
    fn part2_case1() {
        let inputs = read_file("./src/day03/test2.txt");
        let result = solve_part2(&inputs);
        assert_eq!(result, 48);
    }

    #[test]
    fn part2() {
        let inputs = read_file("./src/day03/input1.txt");
        let result = solve_part2(&inputs);
        assert_eq!(result, 78683433);
    }
}
