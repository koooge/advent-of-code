pub fn solve_part1(inputs: &[String]) -> usize {
    let mut ret = 0;
    let mut rules: Vec<Vec<usize>> = vec![];

    fn is_ordered(rules: &[Vec<usize>], pages: &[usize]) -> bool {
        for (i, page1) in pages.iter().enumerate() {
            for page2 in pages.iter().skip(i + 1) {
                for rule in rules {
                    if *page1 == rule[1] && *page2 == rule[0] {
                        return false;
                    }
                }
            }
        }

        true
    }

    for input in inputs {
        if input.contains('|') {
            let rule: Vec<usize> = input.split('|').filter_map(|s| s.parse().ok()).collect();
            rules.push(rule);
        } else if input.contains(',') {
            let pages: Vec<usize> = input.split(',').filter_map(|s| s.parse().ok()).collect();
            if is_ordered(&rules, &pages) {
                let middle = (pages.len() - 1) / 2;
                ret += pages[middle];
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
        let inputs = read_file("./src/day05/test1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 143);
    }

    #[test]
    fn part1() {
        let inputs = read_file("./src/day05/input1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 5108);
    }

    // #[test]
    // fn part2_case1() {
    //     let inputs = read_file("./src/day05/test2.txt");
    //     let result = solve_part2(&inputs);
    //     assert_eq!(result, 9);
    // }

    // #[test]
    // fn part2() {
    //     let inputs = read_file("./src/day05/input1.txt");
    //     let result = solve_part2(&inputs);
    //     assert_eq!(result, 2000);
    // }
}
