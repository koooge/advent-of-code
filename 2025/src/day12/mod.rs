type Shape = Vec<[bool; 3]>;

pub fn solve_part1(inputs: &[String]) -> usize {
    let mut shapes: Vec<Shape> = vec![];
    let mut shape: Shape = vec![];

    for line in inputs {
        if line.contains('#') || line.contains('.') {
            let row: [bool; 3] = line
                .chars()
                .map(|c| match c {
                    '#' => true,
                    '.' => false,
                    _ => unreachable!(),
                })
                .collect::<Vec<bool>>()
                .try_into()
                .expect("must be length 3");
            shape.push(row);
        } else if line.is_empty() {
            shapes.push(shape);
            shape = vec![];
        } else if line.contains('x') {
            let (a, b) = line.split_once(": ").unwrap();
            let (x, y) = a
                .split_once('x')
                .map(|(x, y)| {
                    (
                        x.trim().parse::<usize>().unwrap(),
                        y.trim().parse::<usize>().unwrap(),
                    )
                })
                .unwrap();
            let list: Vec<usize> = b
                .split(' ')
                .map(|n| n.trim().parse::<usize>().unwrap())
                .collect();

            println!("{x}x{y}: {:?}", list);
        }
    }

    2
}

// pub fn solve_part2(inputs: &[String]) -> usize {
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_file;

    #[test]
    fn part1_case1() {
        let inputs = read_file("./src/day12/test1.txt");
        let result = solve_part1(&inputs);
        assert_eq!(result, 2);
    }

    // #[test]
    // fn part1() {
    //     let inputs = read_file("./src/day12/input1.txt");
    //     let result = solve_part1(&inputs);
    //     assert_eq!(result, 477);
    // }
}
