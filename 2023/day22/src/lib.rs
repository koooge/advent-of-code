use std::collections::HashSet;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Pos(usize, usize, usize);

fn is_fallable(bricks: &[Vec<Pos>], brick: &[Pos]) -> bool {
  for pos in brick {
    if pos.2 == 1 {
      return false;
    }
    for b in bricks {
      if b == brick {
        continue;
      }
      if b.iter().find(|&x| x.0 == pos.0 && x.1 == pos.1 && x.2 == pos.2 - 1).is_some() {
        return false;
      }
    }
  }
  true
}

fn wait_settled(bricks: &[Vec<Pos>]) -> Vec<Vec<Pos>> {
  let mut ret = bricks.to_vec();
  let mut is_settled = false;
  while !is_settled {
    is_settled = true;
    for (i, brick) in ret.clone().iter().enumerate() {
      while is_fallable(&ret, &brick) {
        ret[i] = brick.iter().map(|x| Pos(x.0, x.1, x.2 - 1)).collect();
        is_settled = false;
      }
    }
  }
  ret
}

fn is_disintegratable(bricks: &[Vec<Pos>], brick: &[Pos]) -> bool {
  let mut disintegrated = bricks.to_vec();
  if let Some(index) = disintegrated.iter().position(|x| x == brick) {
    disintegrated.remove(index);
  }

  for pos in brick {
    for maybe_above in &disintegrated {
      if maybe_above.iter().find(|&x| x.0 == pos.0 && x.1 == pos.1 && x.2 == pos.2 + 1).is_some() {
        if is_fallable(&disintegrated, maybe_above) {
          return false
        }
      }
    }
  }
  true
}

pub fn solve_part1(inputs: &[String]) -> usize {
  let mut bricks: Vec<Vec<Pos>> = vec![];

  for input in inputs {
    let (begin, end) = input.split_once('~').unwrap();
    let b_pos: Vec<usize> = begin.split(',').filter_map(|x| x.parse().ok()).collect();
    let e_pos: Vec<usize> = end.split(',').filter_map(|x| x.parse().ok()).collect();
    let mut uniq: HashSet<Pos> = HashSet::new();
    for x in b_pos[0].min(e_pos[0])..=b_pos[0].max(e_pos[0]) {
      uniq.insert(Pos(x, b_pos[1], b_pos[2]));
    }
    for y in b_pos[1].min(e_pos[1])..=b_pos[1].max(e_pos[1]) {
      uniq.insert(Pos(b_pos[0], y, b_pos[2]));
    }
    for z in b_pos[2].min(e_pos[2])..=b_pos[2].max(e_pos[2]) {
      uniq.insert(Pos(b_pos[0], b_pos[1], z));
    }
    bricks.push(uniq.into_iter().collect());
  }

  bricks = wait_settled(&bricks);

  let mut ret = 0;
  for b in &bricks {
    if is_disintegratable(&bricks, &b) {
      ret += 1;
    }
  }
  ret
}

// pub fn solve_part2(inputs: &[String], step: usize) -> usize {
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
      assert_eq!(result, 5);
    }

    #[test]
    fn part1_case2() {
      let inputs = read_file("./src/test2.txt");
      let result = solve_part1(&inputs);
      assert_eq!(result, 1);
    }

    #[test]
    fn part1_case3() {
      let inputs = read_file("./src/test3.txt");
      let result = solve_part1(&inputs);
      assert_eq!(result, 5);
    }

    #[test]
    fn part1_case4() {
      let inputs = read_file("./src/test4.txt");
      let result = solve_part1(&inputs);
      assert_eq!(result, 1);
    }

    #[test]
    fn part1() {
      let inputs = read_file("./src/input1.txt");
      let result = solve_part1(&inputs);
      assert_eq!(result, 418);
    }

    // #[test]
    // fn part2_case1() {
    //   let inputs = read_file("./src/test1.txt");
    //   let result = solve_part2(&inputs);
    //   assert_eq!(result, 16);
    // }

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
