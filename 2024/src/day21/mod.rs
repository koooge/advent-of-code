// pub fn solve_part1(inputs: &[String]) -> usize {
//     fn num2dir(buttons: &str) -> Vec<String> {
//         fn get_pos(c: char) -> (usize, usize) {
//             match c {
//                 '7' => (0, 0),
//                 '8' => (0, 1),
//                 '9' => (0, 2),
//                 '4' => (1, 0),
//                 '5' => (1, 1),
//                 '6' => (1, 2),
//                 '1' => (2, 0),
//                 '2' => (2, 1),
//                 '3' => (2, 2),
//                 '0' => (3, 1),
//                 'A' => (3, 2),
//                 _ => unreachable!(),
//             }
//         }

//         fn get_dir(ptr: (usize, usize), dst: (usize, usize), dirs: &[String]) -> Vec<String> {
//             let mut d = dirs.to_vec();
//             if ptr == dst {
//                 for i in 0..d.len() {
//                     d[i] = format!("{}A", d[i]);
//                 }
//                 return d;
//             }

//             let mut dy = d.clone();
//             let mut dx = d.clone();
//             if ptr.0 < dst.0 {
//                 if d.is_empty() {
//                     dy = vec![String::from("v")];
//                 } else {
//                     for i in 0..d.len() {
//                         dy[i] = format!("{}v", d[i])
//                     }
//                 }
//                 dy = get_dir((ptr.0 + 1, ptr.1), dst, &dy);
//             } else if ptr.0 > dst.0 {
//                 if d.is_empty() {
//                     dy = vec![String::from("^")];
//                 } else {
//                     for i in 0..d.len() {
//                         dy[i] = format!("{}^", d[i])
//                     }
//                 }
//                 dy = get_dir((ptr.0 - 1, ptr.1), dst, &dy);
//             }
//             if ptr.1 < dst.1 {
//                 if d.is_empty() {
//                     dx = vec![String::from(">")];
//                 } else {
//                     for i in 0..d.len() {
//                         dx[i] = format!("{}>", d[i])
//                     }
//                 }
//                 dx = get_dir((ptr.0, ptr.1 + 1), dst, &dx);
//             } else if ptr.1 > dst.1 {
//                 if d.is_empty() {
//                     dx = vec![String::from("<")];
//                 } else {
//                     for i in 0..d.len() {
//                         dx[i] = format!("{}<", d[i])
//                     }
//                 }
//                 dx = get_dir((ptr.0, ptr.1 - 1), dst, &dx);
//             }
//             dy.extend(dx);
//             dy
//         }

//         let mut dir: Vec<Vec<String>> = vec![vec![]; 4];
//         let ptr = get_pos('A');
//         for (i, c) in buttons.chars().enumerate() {
//             println!("{} {}", i, c);
//             let dst = get_pos(c);
//             dir[i] = get_dir(ptr, dst, &dir[i]);
//             println!("{:?}", dir);
//         }

//         println!("{} {:?}", buttons, dir);

//         let mut ret: Vec<String> = vec![];
//         for d0 in &dir[0] {
//             for d1 in &dir[1] {
//                 for d2 in &dir[2] {
//                     for d3 in &dir[3] {
//                         ret.push(format!("{}{}{}{}", d0, d1, d2, d3));
//                     }
//                 }
//             }
//         }
//         ret
//     }

//     fn dir2dir(buttons: &str) -> String {
//         fn get_pos(c: char) -> (usize, usize) {
//             match c {
//                 '^' => (0, 1),
//                 'A' => (0, 2),
//                 '<' => (1, 0),
//                 'v' => (1, 1),
//                 '>' => (1, 2),
//                 _ => unreachable!(),
//             }
//         }
//         let mut ret: Vec<char> = vec![];
//         let mut ptr = get_pos('A');
//         for c in buttons.chars() {
//             let dst = get_pos(c);
//             while ptr != dst {
//                 if ptr.0 < dst.0 {
//                     ptr.0 += 1;
//                     ret.push('v');
//                 } else if ptr.1 < dst.1 {
//                     ptr.1 += 1;
//                     ret.push('>');
//                 } else if ptr.1 > dst.1 {
//                     ptr.1 -= 1;
//                     ret.push('<');
//                 } else if ptr.0 > dst.0 {
//                     ptr.0 -= 1;
//                     ret.push('^');
//                 }
//             }
//             ret.push('A');
//         }
//         ret.iter().collect()
//     }

//     let mut ret: Vec<usize> = vec![];
//     for input in inputs {
//         let keypad_as = num2dir(&input[0..=3]);
//         println!("{:?}", keypad_as);
//         let mut keypad_b: String = Default::default();
//         for a in keypad_as {
//             let b = dir2dir(&a);
//             if b.len() < keypad_b.len() {
//                 keypad_b = b
//             }
//         }
//         println!("{:?}", keypad_b);
//         let keypad_c = dir2dir(&keypad_b);
//         let num: usize = input[0..=2].parse().unwrap();
//         println!("{:?}", keypad_c);
//         println!("{:?} {}", keypad_c.len(), num);
//         ret.push(keypad_c.len() * num);
//     }
//     ret.iter().sum()
// }

// pub fn solve_part2(inputs: &[String]) -> usize {
//     0
// }

#[cfg(test)]
mod tests {
    // use super::*;
    // use crate::read_file;

    // #[test]
    // fn part1_case1() {
    //     let inputs = vec![String::from("029A")];
    //     let result = solve_part1(&inputs);
    //     assert_eq!(result, 68 * 29);
    // }

    // #[test]
    // fn part1_case2() {
    //     let inputs = vec![String::from("179A")];
    //     let result = solve_part1(&inputs);
    //     assert_eq!(result, 68 * 179);
    // }

    // #[test]
    // fn part1_case3() {
    //     let inputs = vec![String::from("379A")];
    //     let result = solve_part1(&inputs);
    //     assert_eq!(result, 64 * 379);
    // }

    // #[test]
    // fn part1_case4() {
    //     let inputs = read_file("./src/day21/test1.txt");
    //     let result = solve_part1(&inputs);
    //     assert_eq!(result, 126384);
    // }

    // #[test]
    // fn part1() {
    //     let inputs = read_file("./src/day21/input1.txt");
    //     let result = solve_part1(&inputs, 100);
    //     assert_eq!(result, 1286);
    // }

    // #[test]
    // fn part2_case1() {
    //     let inputs = read_file("./src/day21/test1.txt");
    //     let result = solve_part2(&inputs, 50);
    //     assert_eq!(
    //         result,
    //         32 + 31 + 29 + 39 + 25 + 23 + 20 + 19 + 12 + 14 + 12 + 22 + 4 + 3
    //     );
    // }

    // #[test]
    // fn part2() {
    //     let inputs = read_file("./src/day21/input1.txt");
    //     let result = solve_part2(&inputs, 100);
    //     assert_eq!(result, 0);
    // }
}
