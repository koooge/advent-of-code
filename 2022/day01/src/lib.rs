pub fn task1(inputs: Vec<String>) -> u32 {
  let mut ret: u32 = 0;
  let mut calories: u32 = 0;

  for input in inputs {
    if input.len() == 0 {
      ret = if calories > ret { calories} else { ret };
      calories = 0;
      continue;
    }

    let calory: u32 = input.parse().unwrap();
    calories += calory;
  }

  ret
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn case1() {
      let inputs = read_file("./src/test1.txt");
      let result = task1(inputs);
      println!("{}", result);
      assert_eq!(result, 24000);
    }

    #[test]
    fn input1() {
      let inputs = read_file("./src/input1.txt");
      let result = task1(inputs);
      println!("{}", result);
      assert_eq!(result, 71934);
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
