use std::ops::Deref;

#[derive(Debug)]
struct Map(Vec<Vec<char>>);

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        Self(
            value
                .lines()
                .map(|l| {
                    let trimmed = l.trim();
                    trimmed.chars().collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
        )
    }
}

impl Deref for Map {
    type Target = Vec<Vec<char>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Map {
    fn find_mirrored_col(&self) -> Option<usize> {
        let col_nbr = self[0].len();
        let v = (0..(col_nbr - 1)).find(|idx| {
            let diff = col_nbr - idx;
            let max_dst = (idx + 1).min(diff - 1);
            (0..self.len()).all(|x| (0..max_dst).all(|y| self[x][idx - y] == self[x][idx + y + 1]))
        });
        v.map(|v| v + 1)
    }
    fn find_mirrored_line(&self) -> Option<usize> {
        let line_nbr = self.len();
        let v = (0..(line_nbr - 1)).find(|idx| {
            let diff = line_nbr - idx;
            let max_dst = (idx + 1).min(diff - 1);
            (0..self[0].len())
                .all(|y| (0..max_dst).all(|x| self[idx - x][y] == self[idx + x + 1][y]))
        });
        v.map(|v| (v + 1) * 100)
    }

    fn find_col_with_one_error(&self) -> Option<usize> {
        let col_nbr = self[0].len();
        let v = (0..(col_nbr - 1)).find(|idx| {
            let diff = col_nbr - idx;
            let max_dst = (idx + 1).min(diff - 1);
            let mut to_keep = false;
            for x in 0..self.len() {
                for y in 0..max_dst {
                    if self[x][idx - y] == self[x][idx + y + 1] {
                        continue;
                    } else if !to_keep {
                        to_keep = true
                    } else {
                        return false;
                    }
                }
            }
            to_keep
        });
        v.map(|v| v + 1)
    }
    fn find_row_with_one_error(&self) -> Option<usize> {
        let row_nbr = self.len();
        let v = (0..(row_nbr - 1)).find(|idx| {
            let diff = row_nbr - idx;
            let max_dst = (idx + 1).min(diff - 1);
            let mut to_keep = false;
            for y in 0..self[0].len() {
                for x in 0..max_dst {
                    if self[idx - x][y] == self[idx + x + 1][y] {
                        continue;
                    } else if !to_keep {
                        to_keep = true
                    } else {
                        return false;
                    }
                }
            }
            to_keep
        });
        v.map(|v| (v + 1) * 100)
    }
}

#[derive(Debug)]
struct Maps(Vec<Map>);

impl Deref for Maps {
    type Target = Vec<Map>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&str> for Maps {
    fn from(value: &str) -> Self {
        Self(
            value
                .replace('\r', "")
                .split("\n\n")
                .map(|map| map.into())
                .collect::<Vec<_>>(),
        )
    }
}

pub(crate) fn eval_file(file: &str) -> usize {
    let maps: Maps = file.into();
    maps.iter().fold(0, |acc, map| {
        if let Some(v) = map.find_mirrored_col() {
            acc + v
        } else if let Some(v) = map.find_mirrored_line() {
            acc + v
        } else {
            unreachable!()
        }
    })
}

pub(crate) fn eval_file_2(file: &str) -> usize {
    let maps: Maps = file.into();
    maps.iter().fold(0, |acc, map| {
        if let Some(v) = map.find_col_with_one_error() {
            acc + v
        } else if let Some(v) = map.find_row_with_one_error() {
            acc + v
        } else {
            unreachable!()
        }
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

    fn data() -> &'static str {
        r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#
    }

    #[test]
    fn test_0() {
        assert_eq!(405, eval_file(data()));
    }
    #[test]
    fn test_1() {
        assert_eq!(400, eval_file_2(data()));
    }
}
