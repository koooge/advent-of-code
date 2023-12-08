pub fn solve_part1(inputs: &[String]) -> usize {
  let times = &inputs[0].split(":").last().unwrap().split_whitespace().filter_map(|x| x.parse::<usize>().ok()).collect::<Vec<usize>>();
  let distances = &inputs[1].split(":").last().unwrap().split_whitespace().filter_map(|x| x.parse::<usize>().ok()).collect::<Vec<usize>>();
  let mut ret: Vec<usize> = vec![0; times.len()];

  for i in 0..times.len() {
    for j in 1..times[i] {
      let traveled = j * (times[i] - j);
      if traveled > distances[i] {
        ret[i] += 1;
      }

    }
  }

  return ret.into_iter().reduce(|acc, cur| acc * cur).unwrap();
}

pub fn solve_part2(inputs: &[String]) -> usize {
  let time = inputs[0].split(":").last().unwrap().chars().filter(|c| !c.is_whitespace()).collect::<String>().parse::<usize>().unwrap();
  let distance = inputs[1].split(":").last().unwrap().chars().filter(|c| !c.is_whitespace()).collect::<String>().parse::<usize>().unwrap();
  let mut ret = 0;

  for i in 1..time {
    let traveled = i * (time - i);
    if traveled > distance {
      ret += 1;
    }
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
      assert_eq!(result, 288);
    }

    #[test]
    fn part1() {
      let inputs = read_file("./src/input1.txt");
      let result = solve_part1(&inputs);
      assert_eq!(result, 2756160);
    }

    #[test]
    fn part2_case1() {
      let inputs = read_file("./src/test1.txt");
      let result = solve_part2(&inputs);
      assert_eq!(result, 71503);
    }

    #[test]
    fn part2() {
      let inputs = read_file("./src/input1.txt");
      let result = solve_part2(&inputs);
      assert_eq!(result, 34788142);
    }

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
