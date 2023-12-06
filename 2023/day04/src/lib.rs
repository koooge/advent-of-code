pub fn solve_part1(inputs: &[String]) -> usize {
  let mut ret = 0;

  for input in inputs {
    let mut winning_num = 0;
    let nums = input.split(": ").last().unwrap();
    let left = nums.split(" | ").next().unwrap().split_whitespace().collect::<Vec<&str>>();
    let right = nums.split(" | ").last().unwrap().split_whitespace().collect::<Vec<&str>>();
    for n in left {
      if right.iter().find(|&&x| x == n) != None {
        winning_num = if winning_num == 0 { 1 } else { winning_num * 2};
      }
    }
    ret += winning_num;
  }

  ret
}

// pub fn solve_part2(inputs: &[String]) -> usize {
//   let mut ret = 0;

//   ret
// }

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1_case1() {
      let inputs = read_file("./src/test1.txt");
      let result = solve_part1(&inputs);
      assert_eq!(result, 13);
    }

    #[test]
    fn part1() {
      let inputs = read_file("./src/input1.txt");
      let result = solve_part1(&inputs);
      assert_eq!(result, 26443);
    }

    // #[test]
    // fn part2_case1() {
    //   let inputs = read_file("./src/test1.txt");
    //   let result = solve_part2(&inputs);
    //   assert_eq!(result, 467835);
    // }

    // #[test]
    // fn part2() {
    //   let inputs = read_file("./src/input1.txt");
    //   let result = solve_part2(&inputs);
    //   assert_eq!(result, 81997870);
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
