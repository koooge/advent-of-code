fn step(pipes: &[String], map: &[Vec<usize>], x: usize, y: usize) -> Vec<Vec<usize>> {
  let mut ret = map.to_vec();

  let distance =  ret[y][x];
  if y > 0 && ret[y-1][x] > distance {
    match pipes[y-1].chars().nth(x) {
      Some('|') | Some('7') | Some('F') => {
        ret[y-1][x] = distance + 1;
        ret = step(&pipes, &ret, x, y - 1);
      },
      Some(_) => (),
      None => unreachable!(),
    }
  }
  if x < ret[0].len() - 1 && ret[y][x+1] > distance {
    match pipes[y].chars().nth(x+1) {
      Some('-') | Some('J') | Some('7') => {
        ret[y][x+1] = distance + 1;
        ret = step(&pipes, &ret, x + 1, y);
      },
      Some(_) => (),
      None => unreachable!(),
    }
  }
  if y < ret.len() - 1 && ret[y+1][x] > distance {
    match pipes[y+1].chars().nth(x) {
      Some('|') | Some('L') | Some('J') => {
        ret[y+1][x] = distance + 1;
        ret = step(&pipes, &ret, x, y + 1);
      },
      Some(_) => (),
      None => unreachable!(),
    }
  }
  if x > 0 && ret[y][x-1] > distance {
    match pipes[y].chars().nth(x-1) {
      Some('-') | Some('L') | Some('F') => {
        ret[y][x-1] = distance + 1;
        ret = step(&pipes, &ret, x - 1, y);
      },
      Some(_) => (),
      None => unreachable!(),
    }
  }

  ret
}


pub fn solve_part1(inputs: &[String]) -> usize {
  let ylen = inputs.len();
  let xlen = inputs[0].len();
  let (mut pos_x, mut pos_y) = (usize::MAX, usize::MAX);

  for (i, input) in inputs.iter().enumerate() {
    let x = input.chars().position(|x| x == 'S');
    if x != None {
      (pos_x, pos_y) = (x.unwrap(), i);
      break;
    }
  }

  let mut m = vec![vec![usize::MAX; xlen]; ylen];
  m[pos_y][pos_x] = 0;
  let steps = step(&inputs, &m, pos_x, pos_y);

  let mut ret = 0;
  for step in steps {
    let far = step.into_iter().filter(|&x| x != usize::MAX).max();
    if far != None {
      ret = std::cmp::max(ret, far.unwrap())
    }
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
      assert_eq!(result, 4);
    }

    #[test]
    fn part1_case2() {
      let inputs = read_file("./src/test2.txt");
      let result = solve_part1(&inputs);
      assert_eq!(result, 8);
    }

    #[test]
    fn part1() {
      let inputs = read_file("./src/input1.txt");
      let result = solve_part1(&inputs);
      assert_eq!(result, 0);
    }

    // #[test]
    // fn part2_case1() {
    //   let inputs = read_file("./src/test1.txt");
    //   let result = solve_part2(&inputs);
    //   assert_eq!(result, 2);
    // }

    // #[test]
    // fn part2() {
    //   let inputs = read_file("./src/input1.txt");
    //   let result = solve_part2(&inputs);
    //   assert_eq!(result, 977);
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
