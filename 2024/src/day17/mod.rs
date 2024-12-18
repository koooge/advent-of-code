pub fn solve_part1(inputs: &[String]) -> String {
    let mut regs: Vec<usize> = vec![0; 3];
    regs[0] = inputs[0].split(": ").last().unwrap().parse().unwrap();
    regs[1] = inputs[1].split(": ").last().unwrap().parse().unwrap();
    regs[2] = inputs[2].split(": ").last().unwrap().parse().unwrap();
    let program: Vec<usize> = inputs[4]
        .split(": ")
        .last()
        .unwrap()
        .split(",")
        .filter_map(|c| c.parse().ok())
        .collect();

    let mut counter: usize = 0;
    let mut ret: Vec<usize> = vec![];
    while let Some(opcode) = program.get(counter) {
        let literal = program.get(counter + 1).unwrap();
        let combo = match literal {
            0..=3 => literal,
            4..=6 => &regs[literal - 4],
            _ => unreachable!(),
        };

        match opcode {
            0 => regs[0] /= 2usize.pow(*combo as u32),
            1 => regs[1] ^= *literal,
            2 => regs[1] = combo % 8,
            3 => {
                if regs[0] != 0 {
                    counter = *literal
                } else {
                    counter += 2;
                }
            }
            4 => regs[1] ^= regs[2],
            5 => ret.push(combo % 8),
            6 => regs[1] = regs[0] / 2usize.pow(*combo as u32),
            7 => regs[2] = regs[0] / 2usize.pow(*combo as u32),
            _ => unreachable!(),
        }

        if *opcode != 3 {
            counter += 2;
        }
    }

    ret.iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

// pub fn solve_part2(inputs: &[String]) -> usize {
//   0
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_file;

    #[test]
    fn part1_case1() {
        let inputs = read_file("./src/day17/test1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, String::from("4,6,3,5,6,3,5,2,1,0"));
    }

    #[test]
    fn part1() {
        let inputs = read_file("./src/day17/input1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, String::from("6,4,6,0,4,5,7,2,7"));
    }

    // #[test]
    // fn part2_case1() {
    //     let inputs = read_file("./src/day17/test1.txt");
    //     let result = solve_part2(&inputs);
    //     assert_eq!(result, 45);
    // }

    // #[test]
    // fn part2() {
    //     let inputs = read_file("./src/day17/input1.txt");
    //     let result = solve_part2(&inputs);
    //     assert_eq!(result, 533);
    // }
}
