fn dig(lagoon: &[Vec<bool>], outsides: &[Vec<Option<bool>>], x: usize, y: usize) -> Vec<Vec<Option<bool>>> {
  let mut ret = outsides.to_vec();
  if y > 0 && ret[y-1][x] == None && !lagoon[y-1][x] {
    ret[y-1][x] = Some(true);
    ret = dig(&lagoon, &ret, x, y - 1);
  }
  if x < ret[y].len() - 1 && ret[y][x+1] == None && !lagoon[y][x+1] {
    ret[y][x+1] = Some(true);
    ret = dig(&lagoon, &ret, x + 1, y);
  }
  if y < ret.len() - 1 && ret[y+1][x] == None && !lagoon[y+1][x] {
    ret[y+1][x] = Some(true);
    ret = dig(&lagoon, &ret, x, y + 1);
  }
  if x > 0 && ret[y][x-1] == None && !lagoon[y][x-1] {
    ret[y][x-1] = Some(true);
    ret = dig(&lagoon, &ret, x - 1, y);
  }

  ret
}

pub fn solve_part1(inputs: &[String]) -> usize {
  let mut lagoon: Vec<Vec<bool>> = vec![vec![false; 100]; 100];
  let mut pos: (usize, usize) = (50, 50);
  for input in inputs {
    let l: Vec<&str> = input.split_whitespace().collect::<Vec<&str>>();
    let to = l[0];
    let meter: usize = l[1].parse::<usize>().unwrap();
    match to {
      "U" => {
        for i in pos.1-meter..pos.1 {
          lagoon[i][pos.0] = true;
        }
        pos = (pos.0, pos.1 - meter);
      },
      "R" => {
        for i in pos.0+1..=pos.0+meter {
          lagoon[pos.1][i] = true;
        }
        pos = (pos.0 + meter, pos.1);
      },
      "D" => {
        for i in pos.1+1..=pos.1+meter {
          lagoon[i][pos.0] = true;
        }
        pos = (pos.0, pos.1 + meter);
      },
      "L" => {
        for i in pos.0-meter..pos.0 {
          lagoon[pos.1][i] = true;
        }
        pos = (pos.0 - meter, pos.1);
      },
      _ => unreachable!(),
    }
  }

  let mut outsides = vec![vec![None; lagoon[0].len()]; lagoon.len()];
  outsides[0][0] = Some(true);
  let outsides = dig(&lagoon, &outsides, 0, 0);
  let mut ret = 40 * 40;
  for row in outsides {
    for b in row {
      if b == Some(true) {
        ret -= 1;
      }
    }
  }
  ret
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
      assert_eq!(result, 62);
    }

    #[test]
    fn part1() {
      let inputs = read_file("./src/input1.txt");
      let result = solve_part1(&inputs);
      assert_eq!(result, 8551);
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
