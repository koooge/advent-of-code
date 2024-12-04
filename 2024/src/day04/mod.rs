pub fn solve_part1(inputs: &[String]) -> isize {
    let mut ret = 0;
    let words: Vec<Vec<char>> = inputs.iter().map(|row| row.chars().collect()).collect();

    for i in 0..words.len() {
        for j in 0..words[i].len() {
            if words[i][j] == 'X' {
                // top
                if i > 2
                    && words[i - 1][j] == 'M'
                    && words[i - 2][j] == 'A'
                    && words[i - 3][j] == 'S'
                {
                    ret += 1;
                }
                // top-right
                if i > 2
                    && j < words[i].len() - 3
                    && words[i - 1][j + 1] == 'M'
                    && words[i - 2][j + 2] == 'A'
                    && words[i - 3][j + 3] == 'S'
                {
                    ret += 1;
                }
                // right
                if j < words[i].len() - 3
                    && words[i][j + 1] == 'M'
                    && words[i][j + 2] == 'A'
                    && words[i][j + 3] == 'S'
                {
                    ret += 1;
                }
                // bottom-right
                if i < words.len() - 3
                    && j < words[i].len() - 3
                    && words[i + 1][j + 1] == 'M'
                    && words[i + 2][j + 2] == 'A'
                    && words[i + 3][j + 3] == 'S'
                {
                    ret += 1;
                }
                // bottom
                if i < words.len() - 3
                    && words[i + 1][j] == 'M'
                    && words[i + 2][j] == 'A'
                    && words[i + 3][j] == 'S'
                {
                    ret += 1;
                }
                // bottom-left
                if i < words.len() - 3
                    && j > 2
                    && words[i + 1][j - 1] == 'M'
                    && words[i + 2][j - 2] == 'A'
                    && words[i + 3][j - 3] == 'S'
                {
                    ret += 1;
                }
                // left
                if j > 2
                    && words[i][j - 1] == 'M'
                    && words[i][j - 2] == 'A'
                    && words[i][j - 3] == 'S'
                {
                    ret += 1;
                }
                // top-left
                if i > 2
                    && j > 2
                    && words[i - 1][j - 1] == 'M'
                    && words[i - 2][j - 2] == 'A'
                    && words[i - 3][j - 3] == 'S'
                {
                    ret += 1;
                }
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
        let inputs = read_file("./src/day04/test1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 18);
    }

    #[test]
    fn part1() {
        let inputs = read_file("./src/day04/input1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 2646);
    }

    // #[test]
    // fn part2_case1() {
    //     let inputs = read_file("./src/day04/test2.txt");
    //     let result = solve_part2(&inputs);
    //     assert_eq!(result, 48);
    // }

    // #[test]
    // fn part2() {
    //     let inputs = read_file("./src/day04/input1.txt");
    //     let result = solve_part2(&inputs);
    //     assert_eq!(result, 78683433);
    // }
}
