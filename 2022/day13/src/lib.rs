// TODO: parse String to Vec<u32>
fn parse_str(vect: &mut Vec<Vec<u32>>, str: &String) {
  for i in 1..str.len()-1 {
    let c: char = str.chars().nth(i).unwrap();
    if c == '[' {

    } else {
      vect.push(vec![c.to_digit(10).unwrap()]);
    }
  }
}

pub fn solve_part1(inputs: &[String]) -> u32 {
    let mut ret: u32 = 0;
    let n: usize = inputs.len() / 3;
    for i in 0..=n {
        let mut left: Vec<Vec<u32>> = Vec::new();
        let mut right: Vec<Vec<u32>> = Vec::new();
        let left_str = &inputs[i * 3];
        let right_str = &inputs[i * 3 + 1];
        parse_str(&mut left, &left_str);
        parse_str(&mut right, &right_str);
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1_case1() {
        let inputs = read_file("./src/test1.txt");
        let result = solve_part1(&inputs);
        println!("{}", result);
        assert_eq!(result, 13);
    }

    // #[test]
    // fn part1() {
    //     let inputs = read_file("./src/input1.txt");
    //     let result = solve_part1(inputs);
    //     println!("{}", result);
    //     assert_eq!(result, 361);
    // }

    // #[test]
    // fn part2_case1() {
    //     let inputs = read_file("./src/test1.txt");
    //     let result = solve_part2(inputs);
    //     assert_eq!(result, 29);
    // }

    // #[test]
    // fn part2() {
    //     let inputs = read_file("./src/input1.txt");
    //     let result = solve_part2(inputs);
    //     assert_eq!(result, 354);
    // }

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
