
#[derive(Debug)]
struct Pos(isize, isize, isize);

#[derive(Debug)]
struct Velocity(isize, isize, isize);

#[derive(Debug)]
struct Hail {
  pos: Pos,
  velocity: Velocity,
  a: f64,
  b: f64,
}

impl Hail {
  fn dx(&self, x: f64) -> f64 {
    self.a * x + self.b
  }

  fn is_future(&self, x: f64) -> bool {
    (self.velocity.0 >= 0 && self.pos.0 as f64 <= x) || (self.velocity.0 < 0 && x < self.pos.0 as f64)
  }
}

pub fn solve_part1(inputs: &[String], least: isize, most: isize) -> usize {
  let mut ret = 0;
  let mut hails: Vec<Hail> = vec![];
  for input in inputs {
    let (p, v) = input.split_once(" @ ").unwrap();
    let pos: Vec<isize> = p.split(',').filter_map(|x| x.trim().parse().ok()).collect();
    let velocity: Vec<isize> = v.split(',').filter_map(|x| x.trim().parse().ok()).collect();
    let a = velocity[1] as f64 / velocity[0] as f64;
    let hail = Hail {
      pos: Pos(pos[0], pos[1], pos[2]),
      velocity: Velocity(velocity[0], velocity[1], velocity[2]),
      a: a,
      b: pos[1] as f64 - (a * pos[0] as f64),
    };
    hails.push(hail);
  }

  for i in 0..hails.len()-1 {
    for j in i+1..hails.len() {
      let (hail_a, hail_b) = (hails.get(i).unwrap(), hails.get(j).unwrap());
      let x = (hail_b.b - hail_a.b) / (hail_a.a - hail_b.a);
      if !hail_a.is_future(x) || !hail_b.is_future(x) {
        continue;
      }

      let y = hail_a.dx(x);
      if x >= least as f64 && x <= most as f64 && y >= least as f64 && y <= most as f64 {
        ret += 1;
      }
    }
  }

  ret
}

pub fn solve_part2(inputs: &[String]) -> isize {
  let mut ret = Hail {
    pos: Pos(0, 0, 0),
    velocity: Velocity(0, 0, 0),
    a: 0_f64,
    b: 0_f64,
  };
  let mut hails: Vec<Hail> = vec![];
  for input in inputs {
    let (p, v) = input.split_once(" @ ").unwrap();
    let pos: Vec<isize> = p.split(',').filter_map(|x| x.trim().parse().ok()).collect();
    let velocity: Vec<isize> = v.split(',').filter_map(|x| x.trim().parse().ok()).collect();
    let a = velocity[1] as f64 / velocity[0] as f64;
    let hail = Hail {
      pos: Pos(pos[0], pos[1], pos[2]),
      velocity: Velocity(velocity[0], velocity[1], velocity[2]),
      a: a,
      b: pos[1] as f64 - (a * pos[0] as f64),
    };
    hails.push(hail);
  }

  for i in 0..hails.len()-1 {
    for j in i+1..hails.len() {
      let (hail_a, hail_b) = (hails.get(i).unwrap(), hails.get(j).unwrap());
      let x = (hail_b.b - hail_a.b) / (hail_a.a - hail_b.a);
      if !hail_a.is_future(x) || !hail_b.is_future(x) {
        continue;
      }

      let y = hail_a.dx(x);
    }
  }

  ret.pos.0 + ret.pos.1 + ret.pos.2
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1_case1() {
      let inputs = read_file("./src/test1.txt");
      let result = solve_part1(&inputs, 7, 27);
      assert_eq!(result, 2);
    }

    #[test]
    fn part1() {
      let inputs = read_file("./src/input1.txt");
      let result = solve_part1(&inputs, 200000000000000, 400000000000000);
      assert_eq!(result, 25433);
    }

    #[test]
    fn part2_case1() {
      let inputs = read_file("./src/test1.txt");
      let result = solve_part2(&inputs);
      assert_eq!(result, 47);
    }

    // #[test]
    // fn part2() {
    //   let inputs = read_file("./src/input1.txt");
    //   let result = solve_part2(&inputs);
    //   assert_eq!(result, 70702);
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
