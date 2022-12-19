use std::cmp;

struct Blueprint {
    ore: u32,
    clay: u32,
    obsidian: (u32, u32), // ore, clay
    geode: (u32, u32),    // ore, obsidian
}

fn tick_minute(blueprint: &Blueprint, minute: u32, resources: [u32; 4], robots: [u32; 4]) -> u32 {
    if minute == 24 {
        return resources[3];
    }

    let mut max_geode: u32 = 0;
    let mut does_spent_ore: bool = false;

    // geode
    if resources[0] >= blueprint.geode.0 && resources[2] >= blueprint.geode.1 {
        return cmp::max(
            tick_minute(
                blueprint,
                minute + 1,
                [
                    resources[0] + robots[0] - blueprint.geode.0,
                    resources[1] + robots[1],
                    resources[2] + robots[2] - blueprint.geode.1,
                    resources[3] + robots[3],
                ],
                [robots[0], robots[1], robots[2], robots[3] + 1],
            ),
            max_geode,
        );
    }

    // obsidian
    if resources[0] >= blueprint.obsidian.0 && resources[1] >= blueprint.obsidian.1 {
        return cmp::max(
            tick_minute(
                blueprint,
                minute + 1,
                [
                    resources[0] + robots[0] - blueprint.obsidian.0,
                    resources[1] + robots[1] - blueprint.obsidian.1,
                    resources[2] + robots[2],
                    resources[3] + robots[3],
                ],
                [robots[0], robots[1], robots[2] + 1, robots[3]],
            ),
            max_geode,
        );
    }

    // clay
    if resources[0] >= blueprint.clay {
        max_geode = cmp::max(
            tick_minute(
                blueprint,
                minute + 1,
                [
                    resources[0] + robots[0] - blueprint.clay,
                    resources[1] + robots[1],
                    resources[2] + robots[2],
                    resources[3] + robots[3],
                ],
                [robots[0], robots[1] + 1, robots[2], robots[3]],
            ),
            max_geode,
        );
    }

    // ore
    if resources[0] >= blueprint.ore {
        max_geode = cmp::max(
            tick_minute(
                blueprint,
                minute + 1,
                [
                    resources[0] + robots[0] - blueprint.ore,
                    resources[1] + robots[1],
                    resources[2] + robots[2],
                    resources[3] + robots[3],
                ],
                [robots[0] + 1, robots[1], robots[2], robots[3]],
            ),
            max_geode,
        );
    }

    max_geode = cmp::max(
        tick_minute(
            blueprint,
            minute + 1,
            [
                resources[0] + robots[0],
                resources[1] + robots[1],
                resources[2] + robots[2],
                resources[3] + robots[3],
            ],
            robots,
        ),
        max_geode,
    );

    max_geode
}

pub fn solve_part1(inputs: &[String]) -> u32 {
    let mut ret: u32 = 0;
    let mut level: u32 = 0;

    for input in inputs {
        level += 1;
        let line: Vec<&str> = input.split('.').collect();
        let ore: u32 = line[0].split(' ').collect::<Vec<&str>>()[6]
            .parse()
            .unwrap();
        let clay: u32 = line[1].split(' ').collect::<Vec<&str>>()[5]
            .parse()
            .unwrap();
        let obsidian: Vec<&str> = line[2].split(' ').collect();
        let obsidian_ore: u32 = obsidian[5].parse().unwrap();
        let obsidian_clay: u32 = obsidian[8].parse().unwrap();
        let geode: Vec<&str> = line[3].split(' ').collect();
        let geode_ore: u32 = geode[5].parse().unwrap();
        let geode_obsidian: u32 = geode[8].parse().unwrap();
        let blueprint: Blueprint = Blueprint {
            ore,
            clay,
            obsidian: (obsidian_ore, obsidian_clay),
            geode: (geode_ore, geode_obsidian),
        };

        let max_geode = tick_minute(&blueprint, 0, [0; 4], [1, 0, 0, 0]);
        ret = ret + (level * max_geode);
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
        assert_eq!(result, 33);
    }

    #[test]
    fn part1() {
        let inputs = read_file("./src/input1.txt");
        let result = solve_part1(&inputs);
        println!("{}", result);
        assert_eq!(result, 1624);
    }

    // #[test]
    // fn part2_case1() {
    //     let inputs = read_file("./src/test1.txt");
    //     let result = solve_part2(&inputs);
    //     assert_eq!(result, 58);
    // }

    // #[test]
    // fn part2() {
    //     let inputs = read_file("./src/input1.txt");
    //     let result = solve_part2(&inputs);
    //     assert_eq!(result, 2018);
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
