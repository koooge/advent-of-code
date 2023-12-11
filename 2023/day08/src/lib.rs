use std::collections::HashMap;

pub fn solve_part1(inputs: &[String]) -> usize {
  let mut ret = 0;
  let instructions = inputs.get(0).unwrap().chars().collect::<Vec<char>>();
  let mut network: HashMap<&str, (String, String)> = HashMap::new();

  for input in &inputs[2..] {
    let (src, v) = input.split_once(" = ").unwrap();
    let d = v.replace(&['(', ')'], "");
    let dests = d.split_once(", ").unwrap();
    network.insert(src, (dests.0.to_string(), dests.1.to_string()));
  }

  let mut src = "AAA";
  loop {
    let dests = network.get(src).unwrap();
    let instruction = instructions.get(ret % instructions.len());
    let dest = match instruction {
      Some('L') => &dests.0,
      Some('R') => &dests.1,
      _ => unreachable!(),
    };
    ret += 1;

    if dest == "ZZZ" {
      break;
    }
    src = dest;
  }

  ret
}

// pub fn solve_part2(inputs: &[String]) -> usize {
// }

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1_case1() {
      let inputs = read_file("./src/test1.txt");
      let result = solve_part1(&inputs);
      assert_eq!(result, 2);
    }

    #[test]
    fn part1_case2() {
      let inputs = read_file("./src/test2.txt");
      let result = solve_part1(&inputs);
      assert_eq!(result, 6);
    }

    #[test]
    fn part1() {
      let inputs = read_file("./src/input1.txt");
      let result = solve_part1(&inputs);
      assert_eq!(result, 15989);
    }

    // #[test]
    // fn part2_case1() {
    //   let inputs = read_file("./src/test1.txt");
    //   let result = solve_part2(&inputs);
    //   assert_eq!(result, 5905);
    // }

    // #[test]
    // fn part2() {
    //   let inputs = read_file("./src/input1.txt");
    //   let result = solve_part2(&inputs);
    //   assert_eq!(result, 248750699);
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
