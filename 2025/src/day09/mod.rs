type Pos = (usize, usize);

pub fn solve_part1(inputs: &[String]) -> usize {
    let mut reds: Vec<Pos> = vec![];
    for line in inputs {
        let p: Pos = line
            .split_once(',')
            .map(|(s1, s2)| (s1.parse().unwrap(), s2.parse().unwrap()))
            .unwrap();
        reds.push(p);
    }

    println!("{:?}", reds);
    let mut largest: (usize, Pos, Pos) = (0, (0, 0), (0, 0));
    for i in 0..reds.len() - 1 {
        for j in i + 1..reds.len() {
            let dx = usize::abs_diff(reds[i].0, reds[j].0);
            let dy = usize::abs_diff(reds[i].1, reds[j].1);
            let area_size = (dx + 1) * (dy + 1);
            if area_size > largest.0 {
                largest = (area_size, reds[i], reds[j]);
            }
        }
    }

    largest.0
}

// pub fn solve_part2(inputs: &[String]) -> usize {
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_file;

    #[test]
    fn day08_part1_case1() {
        let inputs = read_file("./src/day09/test1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 50);
    }

    #[test]
    fn part1() {
        let inputs = read_file("./src/day09/input1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 4759930955);
    }
}
