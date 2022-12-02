pub fn solve_part1(mut inputs: Vec<String>) -> u32 {
  let mut ret: u32 = 0;
  let mut calories: u32 = 0;
  inputs.push("".to_string());

  for input in inputs {
    if input.len() == 0 {
      ret = if calories > ret { calories } else { ret };
      calories = 0;
      continue;
    }

    let calory: u32 = input.parse().unwrap();
    calories += calory;
  }

  ret
}

pub fn solve_part2(mut inputs: Vec<String>) -> u32 {
  let mut calories: Vec<u32> = vec![];
  let mut calory: u32 = 0;
  inputs.push("".to_string());

  for input in inputs {
    if input.len() == 0 {
      calories.push(calory);
      calory = 0;
      continue;
    }

    let c: u32 = input.parse().unwrap();
    calory += c;
  }

  calories.sort();
  calories.iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn part1_case1() {
      let inputs = read_file("./src/test1.txt");
      let result = solve_part1(inputs);
      println!("{}", result);
      assert_eq!(result, 24000);
    }

    #[test]
    fn part1() {
      let inputs = read_file("./src/input1.txt");
      let result = solve_part1(inputs);
      println!("{}", result);
      assert_eq!(result, 71934);
    }

    #[test]
    fn part2_case1() {
      let inputs = read_file("./src/test1.txt");
      let result = solve_part2(inputs);
      println!("{}", result);
      assert_eq!(result, 45000);
    }

    #[test]
    fn part2() {
      let inputs = read_file("./src/input1.txt");
      let result = solve_part2(inputs);
      println!("{}", result);
      assert_eq!(result, 211447);
    }

    fn read_file(file_path: &str) -> Vec<String> {
      let contents = fs::read_to_string(file_path);
      let mut ret: Vec<String> = vec![];
      match contents {
        Ok(contents) => {
          for line in contents.lines() {
            ret.push(line.trim().to_string());
          }
        },
        Err(why) => eprintln!("{}", why),
      }
      ret
    }
}
