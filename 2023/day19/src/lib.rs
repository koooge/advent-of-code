use std::collections::HashMap;

pub fn solve_part1(inputs: &[String]) -> usize {
  let mut ret: Vec<usize> = vec![];
  let mut rules: HashMap<String, Vec<(char, String, usize, String)>> = HashMap::new();

  let mut blanked = false;
  for input in inputs {
    if input == "" {
      blanked = true;
    } else if !blanked {
      let (name, rest) = input.split_once('{').unwrap();
      let mut rule: Vec<(char, String, usize, String)> = vec![];
      let r: Vec<&str> = rest.split('}').next().unwrap().split(',').collect();
      for cond in &r[0..r.len()-1] {
        let (s, dest) = cond.split_once(':').unwrap();
        let a = s.chars().nth(0).unwrap();
        let cmp = s.chars().nth(1).unwrap();
        let b = s[2..].parse::<usize>().unwrap();
        rule.push((a, cmp.to_string(), b, dest.to_string()));
      }
      rule.push(('-', String::from("else"), 0, String::from(*r.last().unwrap())));
      rules.insert(name.to_string(), rule);
    } else {
      let s = &input[1..input.len()-1];
      let xmas: Vec<usize> = s.split(',').map(|x| x.split_once('=').unwrap().1.parse::<usize>().unwrap()).collect();
      let hm: HashMap<char, usize> = HashMap::from([
        ('x', xmas[0]),
        ('m', xmas[1]),
        ('a', xmas[2]),
        ('s', xmas[3]),
      ]);

      let mut cur = "in";
      while cur != "A" && cur != "R" {
        let rule = rules.get(cur).unwrap();
        for cond in rule {
          let cmp: &str = &cond.1;
          match cmp {
            ">" => {
              if hm.get(&cond.0).unwrap() > &cond.2 {
                cur = &cond.3;
                break;
              }
            },
            "<" => {
              if hm.get(&cond.0).unwrap() < &cond.2 {
                cur = &cond.3;
                break;
              }
            },
            "else" => {
              cur = &cond.3;
              break;
            },
            _ => unreachable!(),
          }
        }
      }
      if cur == "A" {
        ret.push(xmas.iter().sum());
      }
    }
  }

  ret.iter().sum()
}

// pub fn solve_part2(inputs: &[String]) -> usize {
//   0
// }

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1_case1() {
      let inputs = read_file("./src/test1.txt");
      let result = solve_part1(&inputs);
      assert_eq!(result, 19114);
    }

    #[test]
    fn part1() {
      let inputs = read_file("./src/input1.txt");
      let result = solve_part1(&inputs);
      assert_eq!(result, 330820);
    }

    // #[test]
    // fn part2_case1() {
    //   let inputs = read_file("./src/test1.txt");
    //   let result = solve_part2(&inputs);
    //   assert_eq!(result, 51);
    // }

    // #[test]
    // fn part2() {
    //   let inputs = read_file("./src/input1.txt");
    //   let result = solve_part2(&inputs);
    //   assert_eq!(result, 8754);
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
