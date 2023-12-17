pub fn solve_part1(inputs: &[String]) -> usize {
  let mut ret: Vec<usize> = vec![];
  let mut rocks: Vec<Vec<char>> = vec![];
  for input in inputs {
    let row: Vec<char> = input.chars().collect();
    rocks.push(row);
  }

  for i in 0..rocks[0].len() {
    let mut next = 0;
    for j in 0..rocks.len() {
      match rocks[j][i] {
        'O' => {
          ret.push((rocks.len()) - next);
          next += 1;
        },
        '#' => {
          next = j + 1;
        },
        '.' => (),
        _ => unreachable!(),
      }
    }
  }

  ret.iter().sum()
}

pub fn solve_part2(inputs: &[String]) -> usize {
  let mut ret: Vec<usize> = vec![];
  let mut rocks: Vec<Vec<char>> = vec![];
  for input in inputs {
    let row: Vec<char> = input.chars().collect();
    rocks.push(row);
  }

  for i in 0..rocks[0].len() {
    let mut next = 0;
    for j in 0..rocks.len() {
      match rocks[j][i] {
        'O' => {
          ret.push((rocks.len()) - next);
          next += 1;
        },
        '#' => {
          next = j + 1;
        },
        '.' => (),
        _ => unreachable!(),
      }
    }
  }

  ret.iter().sum()
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1_case1() {
      let inputs = read_file("./src/test1.txt");
      let result = solve_part1(&inputs);
      assert_eq!(result, 136);
    }

    #[test]
    fn part1() {
      let inputs = read_file("./src/input1.txt");
      let result = solve_part1(&inputs);
      assert_eq!(result, 110128);
    }

    #[test]
    fn part2_case1() {
      let inputs = read_file("./src/test1.txt");
      let result = solve_part2(&inputs);
      assert_eq!(result, 64);
    }

    // #[test]
    // fn part2() {
    //   let inputs = read_file("./src/input1.txt");
    //   let result = solve_part2(&inputs);
    //   assert_eq!(result, 0);
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
