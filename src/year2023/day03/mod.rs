use std::collections::{HashMap, HashSet};

const DIRS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub(crate) fn eval_file(file: &str) -> u32 {
    let mut numbers = vec![];
    let mut to_fetch = HashSet::new();
    file.lines().enumerate().for_each(|(line_idx, line)| {
        let line = line.trim();
        let chars = line.chars().collect::<Vec<_>>();
        let mut i = 0;
        while let Some(v) = chars.get(i) {
            let mut j = 1;
            if v.is_numeric() {
                let mut nbr = v.to_digit(10).expect("v is a number");
                while let Some(c) = chars.get(i + j) {
                    if c.is_numeric() {
                        nbr = nbr * 10 + c.to_digit(10).expect("c is a number");
                    } else {
                        break;
                    }
                    j += 1;
                }
                numbers.push((
                    nbr,
                    (0..j)
                        .map(|idx| (line_idx as i32, (i + idx) as i32))
                        .collect::<Vec<_>>(),
                ));
            } else if v != &'.' {
                to_fetch.insert((line_idx as i32, i as i32));
            }
            i += j;
        }
    });
    numbers.iter().fold(0, |acc, (nbr, coords)| {
        acc + if coords.iter().any(|coord| {
            DIRS.iter()
                .any(|(dir_x, dir_y)| to_fetch.get(&(dir_x + coord.0, dir_y + coord.1)).is_some())
        }) {
            nbr
        } else {
            &0
        }
    })
}

#[derive(Hash, PartialEq, Eq, Debug)]
struct Coord {
    x: i32,
    y: i32,
}

pub(crate) fn eval_file_2(file: &str) -> u32 {
    let mut origin_number = HashMap::new();
    let mut mapping = HashMap::new();
    let mut to_fetch = HashMap::new();
    file.lines().enumerate().for_each(|(line_idx, line)| {
        let line = line.trim();
        let chars = line.chars().collect::<Vec<_>>();
        let mut i = 0;
        while let Some(v) = chars.get(i) {
            let mut j = 1;
            if v.is_numeric() {
                let mut nbr = v.to_digit(10).expect("v is a number");
                while let Some(c) = chars.get(i + j) {
                    if c.is_numeric() {
                        nbr = nbr * 10 + c.to_digit(10).expect("c is a number");
                    } else {
                        break;
                    }
                    j += 1;
                }
                origin_number.insert(
                    Coord {
                        x: line_idx as i32,
                        y: i as i32,
                    },
                    nbr,
                );
                (0..j).for_each(|idx| {
                    mapping.insert(
                        Coord {
                            x: line_idx as i32,
                            y: (i + idx) as i32,
                        },
                        Coord {
                            x: line_idx as i32,
                            y: i as i32,
                        },
                    );
                })
            } else if v != &'.' {
                to_fetch.insert(
                    Coord {
                        x: line_idx as i32,
                        y: i as i32,
                    },
                    (v.clone(), HashSet::new()),
                );
            }
            i += j;
        }
    });

    to_fetch.iter_mut().for_each(|(coord, (_, nbrs))| {
        DIRS.iter().for_each(|(dir_x, dir_y)| {
            if let Some(origin) = mapping.get(&Coord {
                x: dir_x + coord.x,
                y: dir_y + coord.y,
            }) {
                nbrs.insert(origin);
            }
        })
    });

    to_fetch
        .iter()
        .filter(|(_, (c, nbrs))| c == &'*' && nbrs.len() == 2)
        .fold(0, |acc, (_, (_, nbrs))| {
            acc + nbrs.iter().fold(1, |acc, coord| {
                acc * origin_number.get(coord).expect("Number mapped")
            })
        })
}

pub(crate) fn print_sol_1(file: &str) {
    print!("res : {}", eval_file(file));
}
pub(crate) fn print_sol_2(file: &str) {
    print!("res : {}", eval_file_2(file));
}

#[cfg(test)]
mod tests {
    use super::{eval_file, eval_file_2};

    fn data_0() -> &'static str {
        r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#
    }
    fn data_1() -> &'static str {
        r#"12.......*..
+.........34
.......-12..
..78........
..*....60...
78.........9
.5.....23..$
8...90*12...
............
2.2......12.
.*.........*
1.1..503+.56"#
    }

    fn data_2() -> &'static str {
        r#"........
.24..4..
......*."#
    }
    fn data_3() -> &'static str {
        r#"....................
..-52..52-..52..52..
..................-."#
    }
    fn data_4() -> &'static str {
        r#"
        12.......*..
        +.........34
        .......-12..
        ..78........
        ..*....60...
        78.........9
        15.....23..$
        8...90*12...
        ............
        2.2......12.
        .*.........*
        1.1..503+.56"#
    }

    fn data_5() -> &'static str {
        r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#
    }
    #[test]
    fn test_0() {
        assert_eq!(4361, eval_file(data_0()));
    }

    #[test]
    fn test_1() {
        assert_eq!(925, eval_file(data_1()));
    }
    #[test]
    fn test_2() {
        assert_eq!(4, eval_file(data_2()));
    }
    #[test]
    fn test_3() {
        assert_eq!(156, eval_file(data_3()));
    }
    #[test]
    fn test_4() {
        assert_eq!(925, eval_file(data_4()));
    }
    #[test]
    fn test_5() {
        assert_eq!(467835, eval_file_2(data_5()));
    }
    //     #[test]
    //     fn test_1() {
    //         let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    // Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    // Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    // Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    // Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
    //         assert_eq!(2286, eval_file_2(input));
    //     }
}
