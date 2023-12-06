fn lookup(src: usize, dst_map: &[(usize, usize, usize)]) -> usize {
  let mut ret= src;

  for m in dst_map {
    if src >= m.1 && src <= m.1 + m.2 {
      let d = src - m.1;
      ret = m.0 + d;
      break;
    }
  }

  ret
}

pub fn solve_part1(inputs: &[String]) -> usize {
  let mut seeds: Vec<usize> = vec![];
  let mut seed2soil: Vec<(usize, usize, usize)> = vec![];
  let mut soil2fertilizer: Vec<(usize, usize, usize)> = vec![];
  let mut fertilizer2water: Vec<(usize, usize, usize)> = vec![];
  let mut water2light: Vec<(usize, usize, usize)> = vec![];
  let mut light2temperature: Vec<(usize, usize, usize)> = vec![];
  let mut temperature2humidity: Vec<(usize, usize, usize)> = vec![];
  let mut humidity2location: Vec<(usize, usize, usize)> = vec![];

  let mut i = 0;
  while i < inputs.len() {
    let input = &inputs[i];

    if input.starts_with("seeds:") {
      seeds = input.split(": ").last().unwrap().split_whitespace().filter_map(|x| x.parse::<usize>().ok()).collect();
    } else if input.starts_with("seed-to-soil map:") {
      for j in i+1..inputs.len() {
        if inputs[j] == "" {
          break
        }
        let m = inputs[j].split_whitespace().filter_map(|x| x.parse::<usize>().ok()).collect::<Vec<usize>>();
        seed2soil.push((m[0], m[1], m[2]));
      }
    } else if input.starts_with("soil-to-fertilizer map:") {
      for j in i+1..inputs.len() {
        if inputs[j] == "" {
          break
        }
        let m = inputs[j].split_whitespace().filter_map(|x| x.parse::<usize>().ok()).collect::<Vec<usize>>();
        soil2fertilizer.push((m[0], m[1], m[2]));
      }
    } else if input.starts_with("fertilizer-to-water map:") {
      for j in i+1..inputs.len() {
        if inputs[j] == "" {
          break
        }
        let m = inputs[j].split_whitespace().filter_map(|x| x.parse::<usize>().ok()).collect::<Vec<usize>>();
        fertilizer2water.push((m[0], m[1], m[2]));
      }
    } else if input.starts_with("water-to-light map:") {
      for j in i+1..inputs.len() {
        if inputs[j] == "" {
          break
        }
        let m = inputs[j].split_whitespace().filter_map(|x| x.parse::<usize>().ok()).collect::<Vec<usize>>();
        water2light.push((m[0], m[1], m[2]));
      }
    } else if input.starts_with("light-to-temperature map:") {
      for j in i+1..inputs.len() {
        if inputs[j] == "" {
          break
        }
        let m = inputs[j].split_whitespace().filter_map(|x| x.parse::<usize>().ok()).collect::<Vec<usize>>();
        light2temperature.push((m[0], m[1], m[2]));
      }
    } else if input.starts_with("temperature-to-humidity map:") {
      for j in i+1..inputs.len() {
        if inputs[j] == "" {
          break
        }
        let m = inputs[j].split_whitespace().filter_map(|x| x.parse::<usize>().ok()).collect::<Vec<usize>>();
        temperature2humidity.push((m[0], m[1], m[2]));
      }
    } else if input.starts_with("humidity-to-location map:") {
      for j in i+1..inputs.len() {
        if inputs[j] == "" {
          break
        }
        let m = inputs[j].split_whitespace().filter_map(|x| x.parse::<usize>().ok()).collect::<Vec<usize>>();
        humidity2location.push((m[0], m[1], m[2]));
      }
    }

    i += 1;
  }

  let mut min_location = usize::MAX;
  for seed in seeds {
    let soil = lookup(seed, &seed2soil);
    let fertilizer = lookup(soil, &soil2fertilizer);
    let water = lookup(fertilizer, &fertilizer2water);
    let light = lookup(water, &water2light);
    let temperature = lookup(light, &light2temperature);
    let humidity = lookup(temperature, &temperature2humidity);
    let location = lookup(humidity, &humidity2location);
    if location < min_location {
      min_location = location;
    }
  }

  min_location
}

pub fn solve_part2(inputs: &[String]) -> usize {
  let mut seeds: Vec<(usize, usize)> = vec![];
  let mut seed2soil: Vec<(usize, usize, usize)> = vec![];
  let mut soil2fertilizer: Vec<(usize, usize, usize)> = vec![];
  let mut fertilizer2water: Vec<(usize, usize, usize)> = vec![];
  let mut water2light: Vec<(usize, usize, usize)> = vec![];
  let mut light2temperature: Vec<(usize, usize, usize)> = vec![];
  let mut temperature2humidity: Vec<(usize, usize, usize)> = vec![];
  let mut humidity2location: Vec<(usize, usize, usize)> = vec![];

  let mut i = 0;
  while i < inputs.len() {
    let input = &inputs[i];

    if input.starts_with("seeds:") {
      let s = input.split(": ").last().unwrap().split_whitespace().filter_map(|x| x.parse::<usize>().ok()).collect::<Vec<usize>>();
      for j in (0..s.len()).step_by(2) {
        seeds.push((s[j], s[j+1]));
      }
    } else if input.starts_with("seed-to-soil map:") {
      for j in i+1..inputs.len() {
        if inputs[j] == "" {
          break
        }
        let m = inputs[j].split_whitespace().filter_map(|x| x.parse::<usize>().ok()).collect::<Vec<usize>>();
        seed2soil.push((m[0], m[1], m[2]));
      }
    } else if input.starts_with("soil-to-fertilizer map:") {
      for j in i+1..inputs.len() {
        if inputs[j] == "" {
          break
        }
        let m = inputs[j].split_whitespace().filter_map(|x| x.parse::<usize>().ok()).collect::<Vec<usize>>();
        soil2fertilizer.push((m[0], m[1], m[2]));
      }
    } else if input.starts_with("fertilizer-to-water map:") {
      for j in i+1..inputs.len() {
        if inputs[j] == "" {
          break
        }
        let m = inputs[j].split_whitespace().filter_map(|x| x.parse::<usize>().ok()).collect::<Vec<usize>>();
        fertilizer2water.push((m[0], m[1], m[2]));
      }
    } else if input.starts_with("water-to-light map:") {
      for j in i+1..inputs.len() {
        if inputs[j] == "" {
          break
        }
        let m = inputs[j].split_whitespace().filter_map(|x| x.parse::<usize>().ok()).collect::<Vec<usize>>();
        water2light.push((m[0], m[1], m[2]));
      }
    } else if input.starts_with("light-to-temperature map:") {
      for j in i+1..inputs.len() {
        if inputs[j] == "" {
          break
        }
        let m = inputs[j].split_whitespace().filter_map(|x| x.parse::<usize>().ok()).collect::<Vec<usize>>();
        light2temperature.push((m[0], m[1], m[2]));
      }
    } else if input.starts_with("temperature-to-humidity map:") {
      for j in i+1..inputs.len() {
        if inputs[j] == "" {
          break
        }
        let m = inputs[j].split_whitespace().filter_map(|x| x.parse::<usize>().ok()).collect::<Vec<usize>>();
        temperature2humidity.push((m[0], m[1], m[2]));
      }
    } else if input.starts_with("humidity-to-location map:") {
      for j in i+1..inputs.len() {
        if inputs[j] == "" {
          break
        }
        let m = inputs[j].split_whitespace().filter_map(|x| x.parse::<usize>().ok()).collect::<Vec<usize>>();
        humidity2location.push((m[0], m[1], m[2]));
      }
    }

    i += 1;
  }

  let mut min_location = usize::MAX;
  for (start, length) in seeds {
    for seed in start..start+length {
      let soil = lookup(seed, &seed2soil);
      let fertilizer = lookup(soil, &soil2fertilizer);
      let water = lookup(fertilizer, &fertilizer2water);
      let light = lookup(water, &water2light);
      let temperature = lookup(light, &light2temperature);
      let humidity = lookup(temperature, &temperature2humidity);
      let location = lookup(humidity, &humidity2location);
      if location < min_location {
        min_location = location;
      }
    }
  }

  min_location
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn part1_case1() {
      let inputs = read_file("./src/test1.txt");
      let result = solve_part1(&inputs);
      assert_eq!(result, 35);
    }

    #[test]
    fn part1() {
      let inputs = read_file("./src/input1.txt");
      let result = solve_part1(&inputs);
      assert_eq!(result, 227653707);
    }

    #[test]
    fn part2_case1() {
      let inputs = read_file("./src/test1.txt");
      let result = solve_part2(&inputs);
      assert_eq!(result, 46);
    }

    #[test]
    fn part2() {
      let inputs = read_file("./src/input1.txt");
      let result = solve_part2(&inputs);
      assert_eq!(result, 78775051);
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
