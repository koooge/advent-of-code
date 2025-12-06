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

// pub fn solve_part2(inputs: &[String]) -> usize {
// }

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
}
