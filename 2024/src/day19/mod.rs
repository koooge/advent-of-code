pub fn solve_part1(inputs: &[String]) -> usize {
    let patterns: Vec<String> = inputs[0].split(", ").map(|s| s.to_string()).collect();

    let mut ret = 0;
    for input in inputs.iter().skip(2) {
        let mut designs: Vec<String> = vec![input.to_string()];
        while let Some(design) = designs.pop() {
            if patterns.iter().any(|pat| design == *pat) {
                ret += 1;
                break;
            }
            let matched: Vec<_> = patterns
                .iter()
                .filter(|&pat| design.starts_with(pat))
                .collect();
            for pat in matched {
                designs.push(design.strip_prefix(pat).unwrap().to_string());
            }
        }
    }
    ret
}

// pub fn solve_part2(inputs: &[String]) -> String {
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_file;

    #[test]
    fn part1_case1() {
        let inputs = read_file("./src/day19/test1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 6);
    }

    // #[test]
    // fn part1() {
    //     let inputs = read_file("./src/day19/input1.txt");
    //     let result = solve_part1(&inputs);
    //     assert_eq!(result, 286);
    // }

    // #[test]
    // fn part2_case1() {
    //     let inputs = read_file("./src/day19/test1.txt");
    //     let result = solve_part2(&inputs, 7);
    //     assert_eq!(result, String::from("6,1"));
    // }

    // #[test]
    // fn part2() {
    //     let inputs = read_file("./src/day19/input1.txt");
    //     let result = solve_part2(&inputs, 71);
    //     assert_eq!(result, String::from("20,64"));
    // }
}
