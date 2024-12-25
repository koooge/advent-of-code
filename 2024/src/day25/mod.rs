pub fn solve_part1(inputs: &[String]) -> usize {
    let mut locks: Vec<Vec<usize>> = vec![];
    let mut keys: Vec<Vec<usize>> = vec![];
    let mut chunk: Vec<Vec<char>> = vec![];
    for input in inputs {
        if input.is_empty() {
            continue;
        }

        chunk.push(input.chars().collect());
        if chunk.len() == 7 {
            match chunk[0][0] {
                '#' => {
                    let mut lock: Vec<usize> = vec![0; chunk[0].len()];
                    for row in chunk.iter().take(5 + 1).skip(1) {
                        for (j, c) in row.iter().enumerate() {
                            if *c == '#' {
                                lock[j] += 1;
                            }
                        }
                    }
                    locks.push(lock);
                }
                '.' => {
                    let mut key: Vec<usize> = vec![0; chunk[0].len()];
                    for row in chunk.iter().take(5 + 1).skip(1).rev() {
                        for (j, c) in row.iter().enumerate() {
                            if *c == '#' {
                                key[j] += 1;
                            }
                        }
                    }
                    keys.push(key);
                }
                _ => unreachable!(),
            }
            chunk = vec![];
        }
    }

    let mut ret = 0;
    for lock in locks {
        for key in &keys {
            let mut is_overlap = false;
            for i in 0..lock.len() {
                if lock[i] + key[i] > 5 {
                    is_overlap = true;
                    break;
                }
            }
            if !is_overlap {
                ret += 1;
            }
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
        let inputs = read_file("./src/day25/test1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 3);
    }

    #[test]
    fn part1() {
        let inputs = read_file("./src/day25/input1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 3671);
    }
}
