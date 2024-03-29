pub fn solve_part1(inputs: &[String]) -> usize {
  let mut ret = 0;

  for (i, input) in inputs.iter().enumerate() {
    let sets = input.split(": ").collect::<Vec<&str>>()[1].split("; ").collect::<Vec<&str>>();
    let mut valid_game = true;
    for set in sets {
      let cubes = set.split(", ").collect::<Vec<&str>>();
      for cube in cubes {
        let (p, color) = cube.split_once(' ').unwrap();
        let point = p.parse::<usize>().unwrap();
        if (color == "red" && point > 12) || (color == "green" && point > 13) || (color == "blue" && point > 14) {
          valid_game = false;
          break
        }
      }
      if !valid_game {
        break
      }
    }
    if valid_game {
      ret += i + 1;
    }
  }

  ret
}

pub fn solve_part2(inputs: &[String]) -> usize {
  let mut ret = 0;

  for input in inputs {
    let (mut red, mut green, mut blue) = (0, 0, 0);
    let sets = input.split(": ").collect::<Vec<&str>>()[1].split("; ").collect::<Vec<&str>>();
    for set in sets {
      let cubes = set.split(", ").collect::<Vec<&str>>();
      for cube in cubes {
        let (p, color) = cube.split_once(' ').unwrap();
        let point = p.parse::<usize>().unwrap();
        if color == "red" {
          red = std::cmp::max(point, red)
        } else if color == "green" {
          green = std::cmp::max(point, green)
        } else if color == "blue" {
          blue = std::cmp::max(point, blue)
        }
      }
    }
    ret += red * green * blue;
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
      assert_eq!(result, 8);
    }

    #[test]
    fn part1() {
      let inputs = read_file("./src/input1.txt");
      let result = solve_part1(&inputs);
      assert_eq!(result, 2913);
    }

    #[test]
    fn part2_case1() {
      let inputs = read_file("./src/test1.txt");
      let result = solve_part2(&inputs);
      assert_eq!(result, 2286);
    }

    #[test]
    fn part2() {
      let inputs = read_file("./src/input1.txt");
      let result = solve_part2(&inputs);
      assert_eq!(result, 55593);
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
