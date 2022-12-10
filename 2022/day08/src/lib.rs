fn is_visible(r: usize, c: usize, inputs: &Vec<String>) -> bool {
    let row: Vec<u32> = inputs[r].chars().map(|c| c.to_digit(10).unwrap()).collect();
    let col: Vec<u32> = inputs
        .iter()
        .map(|r| r.chars().nth(c).unwrap().to_digit(10).unwrap())
        .collect();

    for i in 0..c {
        if row[i] >= row[c] {
            break;
        } else if i == c - 1 {
            return true;
        }
    }

    for i in (c + 1)..row.len() {
        if row[i] >= row[c] {
            break;
        } else if i == row.len() - 1 {
            return true;
        }
    }

    for i in 0..r {
        if col[i] >= row[c] {
            break;
        } else if i == r - 1 {
            return true;
        }
    }

    for i in (r + 1)..inputs.len() {
        if col[i] >= row[c] {
            break;
        } else if i == inputs.len() - 1 {
            return true;
        }
    }

    false
}

pub fn solve_part1(inputs: Vec<String>) -> u32 {
    let mut ret: u32 = (inputs.len() as u32 * 2) + ((inputs.len() as u32 - 2) * 2);

    for i in 1..(inputs.len() - 1) {
        for j in 1..(inputs[i].len() - 1) {
            if is_visible(i, j, &inputs) {
                ret += 1;
            }
        }
    }

    ret
}

use std::cmp;

fn calc_score(r: usize, c: usize, inputs: &Vec<String>) -> u32 {
    let row: Vec<u32> = inputs[r].chars().map(|c| c.to_digit(10).unwrap()).collect();
    let col: Vec<u32> = inputs
        .iter()
        .map(|r| r.chars().nth(c).unwrap().to_digit(10).unwrap())
        .collect();

    let mut base: u32 = row[c];
    let mut left: u32 = 0;
    for i in (0..c).rev() {
        left += 1;
        if row[i] >= base {
            break;
        }
        base = cmp::max(row[i], base);
    }

    base = row[c];
    let mut right: u32 = 0;
    for i in (c + 1)..row.len() {
        right += 1;
        if row[i] >= row[c] {
            break;
        }
        base = cmp::max(row[i], base);
    }

    base = row[c];
    let mut top: u32 = 0;
    for i in (0..r).rev() {
        top += 1;
        if col[i] >= row[c] {
            break;
        }
        base = cmp::max(col[i], base);
    }

    base = row[c];
    let mut bottom: u32 = 0;
    for i in (r + 1)..inputs.len() {
        bottom += 1;
        if col[i] >= base {
            break;
        }
        base = cmp::max(col[i], base);
    }

    top * right * bottom * left
}

pub fn solve_part2(inputs: Vec<String>) -> u32 {
    let mut max_score: u32 = 0;

    for i in 1..(inputs.len() - 1) {
        for j in 1..(inputs[i].len() - 1) {
            let score = calc_score(i, j, &inputs);
            max_score = cmp::max(score, max_score);
        }
    }

    max_score
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1_case1() {
        let inputs = read_file("./src/test1.txt");
        let result = solve_part1(inputs);
        println!("{}", result);
        assert_eq!(result, 21);
    }

    #[test]
    fn part1() {
        let inputs = read_file("./src/input1.txt");
        let result = solve_part1(inputs);
        println!("{}", result);
        assert_eq!(result, 1798);
    }

    #[test]
    fn part2_case1() {
        let inputs = read_file("./src/test1.txt");
        let result = solve_part2(inputs);
        println!("{}", result);
        assert_eq!(result, 8);
    }

    #[test]
    fn part2() {
        let inputs = read_file("./src/input1.txt");
        let result = solve_part2(inputs);
        println!("{}", result);
        assert_eq!(result, 259308);
    }

    fn read_file(file_path: &str) -> Vec<String> {
        let contents = fs::read_to_string(file_path);
        let mut ret: Vec<String> = vec![];
        match contents {
            Ok(contents) => {
                for line in contents.lines() {
                    ret.push(line.to_string());
                }
            }
            Err(why) => eprintln!("{}", why),
        }
        ret
    }
}
