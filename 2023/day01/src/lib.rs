pub fn solve_part1(inputs: &[String]) -> usize {
  let mut ret = 0;

  for input in inputs {
    let nv: Vec<&str> = input.matches(char::is_numeric).collect();
    let num = format!("{}{}", nv[0], nv[nv.len() - 1]).parse::<usize>().unwrap();
    ret += num;
  }

  ret
}

pub fn solve_part2(inputs: &[String]) -> usize {
  let mut ret = 0;

  for input in inputs {
    let (mut first, mut last): (usize, usize) = (0, 0);

    for (i, c) in input.char_indices() {
       if c.is_numeric() {
        first = usize::try_from(c.to_digit(10).unwrap()).unwrap();
        break
      } else if input[i..].starts_with("one") {
        first = 1;
        break
      } else if input[i..].starts_with("two") {
        first = 2;
        break
      } else if input[i..].starts_with("three") {
        first = 3;
        break
      } else if input[i..].starts_with("four") {
        first = 4;
        break
      } else if input[i..].starts_with("five") {
        first = 5;
        break
      } else if input[i..].starts_with("six") {
        first = 6;
        break
      } else if input[i..].starts_with("seven") {
        first = 7;
        break
      } else if input[i..].starts_with("eight") {
        first = 8;
        break
      } else if input[i..].starts_with("nine") {
        first = 9;
        break
      }
    }

    let rev = input.chars().rev().collect::<String>();
    for (i, c) in rev.char_indices() {
      if c.is_numeric() {
        last = usize::try_from(c.to_digit(10).unwrap()).unwrap();
        break
      } else if rev[i..].starts_with("eno") {
        last = 1;
        break
      } else if rev[i..].starts_with("owt") {
        last = 2;
        break
      } else if rev[i..].starts_with("eerht") {
        last = 3;
        break
      } else if rev[i..].starts_with("ruof") {
        last = 4;
        break
      } else if rev[i..].starts_with("evif") {
        last = 5;
        break
      } else if rev[i..].starts_with("xis") {
        last = 6;
        break
      } else if rev[i..].starts_with("neves") {
        last = 7;
        break
      } else if rev[i..].starts_with("thgie") {
        last = 8;
        break
      } else if rev[i..].starts_with("enin") {
        last = 9;
        break
      }
    }

    let num = first * 10 + last;
    ret += num
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
      assert_eq!(result, 142);
    }

    #[test]
    fn part1() {
      let inputs = read_file("./src/input1.txt");
      let result = solve_part1(&inputs);
      assert_eq!(result, 55607);
    }

    #[test]
    fn part2_case1() {
      let inputs = read_file("./src/test2.txt");
      let result = solve_part2(&inputs);
      assert_eq!(result, 281);
    }

    #[test]
    fn part2() {
      let inputs = read_file("./src/input1.txt");
      let result = solve_part2(&inputs);
      assert_eq!(result, 55291);
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
