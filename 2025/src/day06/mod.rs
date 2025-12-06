pub fn solve_part1(inputs: &[String]) -> usize {
    let mut ret = 0;
    let mut numbers: Vec<Vec<usize>> = vec![];

    for line in inputs {
        let elements: Vec<&str> = line.split_whitespace().collect();
        if elements[0] == "+" || elements[0] == "*" {
            for (i, &el) in elements.iter().enumerate() {
                if el == "+" {
                    let sum: usize = numbers[i].iter().sum();
                    ret += sum;
                } else if el == "*" {
                    let product: usize = numbers[i].iter().product();
                    ret += product;
                } else {
                    unreachable!();
                }
            }
            continue;
        }

        for (i, el) in elements.iter().enumerate() {
            let num: usize = el.parse().unwrap();
            numbers
                .get_mut(i)
                .map(|r| r.push(num))
                .unwrap_or_else(|| numbers.push(vec![num]));
        }
    }

    ret
}

pub fn solve_part2(inputs: &[String]) -> usize {
    let mut ret = 0;
    let mut sheet: Vec<[char; 10000]> = vec![];

    for line in inputs {
        let mut s: [char; 10000] = [' '; 10000];
        for (x, c) in line.chars().enumerate() {
            s[x] = c;
        }
        sheet.push(s);
    }

    let mut nums: Vec<usize> = vec![];
    for x in (0..sheet[sheet.len() - 1].len()).rev() {
        let mut s = String::from("");
        for row in sheet.iter().take(sheet.len() - 1) {
            if row[x] == ' ' {
                continue;
            }
            s.push(row[x]);
        }
        if s.is_empty() {
            continue;
        }
        let num: usize = s.parse().unwrap();
        nums.push(num);

        let el = sheet[sheet.len() - 1][x];
        if el == '+' {
            let sum: usize = nums.iter().sum();
            ret += sum;
            nums = vec![];
        } else if el == '*' {
            let product: usize = nums.iter().product();
            ret += product;
            nums = vec![];
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_file;

    #[test]
    fn day06_part1_case1() {
        let inputs = read_file("./src/day06/test1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 4277556);
    }

    #[test]
    fn part1() {
        let inputs = read_file("./src/day06/input1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 5595593539811);
    }

    #[test]
    fn day06_part2_case1() {
        let inputs = read_file("./src/day06/test1.txt");
        let result = solve_part2(&inputs);
        assert_eq!(result, 3263827);
    }

    #[test]
    fn part2() {
        let inputs = read_file("./src/day06/input1.txt");
        let result = solve_part2(&inputs);
        assert_eq!(result, 10153315705125);
    }
}
