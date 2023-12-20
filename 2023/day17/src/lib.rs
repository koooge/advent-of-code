fn dijkstra(map: &[Vec<u8>], minmap: &[Vec<u8>], x: usize, y: usize, from: i8, count: u8) -> Vec<Vec<u8>> {
  let cur = minmap[y][x];
  let goal = minmap.last().unwrap().last().unwrap();
  if &cur > goal {
    return minmap.to_vec();
  }
  let mut ret = minmap.to_vec();

  // top
  if y > 0 && from != 0 {
    if from != 2 || count < 3 {
      let top = cur + map[y-1][x];
      if top < ret[y-1][x] {
        let cnt = if from == -1 || from == 2 { count + 1 } else { 1 };
        ret[y-1][x] = top;
        ret = dijkstra(&map, &ret, x, y - 1, 2, cnt);
      }
    }
  }

  // right
  if x < map[y].len() - 1 && from != 1 {
    if from != 3 || count < 3 {
      let right = cur + map[y][x+1];
      if right < ret[y][x+1] {
        let cnt = if from == -1 || from == 3 { count + 1 } else { 1 };
        ret[y][x+1] = right;
        ret = dijkstra(&map, &ret, x + 1, y, 3, cnt);
      }
    }
  }

  // bottom
  if y < map.len() - 1 && from != 2 {
    if from != 0 || count < 3 {
      let bottom = cur + map[y+1][x];
      if bottom < ret[y+1][x] {
        let cnt = if from == -1 || from == 0 { count + 1 } else { 1 };
        ret[y+1][x] = bottom;
        ret = dijkstra(&map, &ret, x, y + 1, 0, cnt);
      }
    }
  }

  // left
  if x > 0 && from != 3 {
    if from != 1 || count < 3 {
      let left = cur + map[y][x-1];
      if left < ret[y][x-1] {
        let cnt = if from == -1 || from == 1 { count + 1 } else { 1 };
        ret[y][x-1] = left;
        ret = dijkstra(&map, &ret, x - 1, y, 1, cnt);
      }
    }
  }

  ret
}

pub fn solve_part1(inputs: &[String]) -> usize {
  let mut map: Vec<Vec<u8>> = vec![];

  for input in inputs {
    let row: Vec<u8> = input.chars().filter_map(|x| x.to_string().parse::<u8>().ok()).collect();
    map.push(row);
  }
  let mut minmap: Vec<Vec<u8>> = vec![vec![u8::MAX; map[0].len()]; map.len()];
  minmap[0][0] = map[0][0];
  let ret: Vec<Vec<u8>> = dijkstra(&map, &minmap, 0, 0, -1, 1);

  println!("{:?}", ret);
  usize::from(ret.last().unwrap().last().unwrap().clone())
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
      assert_eq!(result, 102);
    }

    // #[test]
    // fn part1() {
    //   let inputs = read_file("./src/input1.txt");
    //   let result = solve_part1(&inputs);
    //   assert_eq!(result, 8551);
    // }

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
