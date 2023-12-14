use std::{
    collections::HashMap,
    fmt::Debug,
    ops::{Deref, DerefMut},
};

#[derive(Eq, Hash, PartialEq, Clone)]
struct Map(Vec<Vec<char>>);

const NBR_CYCLES: usize = 1_000_000_000;

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.iter().for_each(|line| {
            line.iter().for_each(|c| {
                let _ = write!(f, "{}", c);
            });

            let _ = write!(f, "\n");
        });
        Ok(())
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        Map(value
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| l.chars().collect())
            .collect())
    }
}

impl Deref for Map {
    type Target = Vec<Vec<char>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Map {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Map {
    fn push_north(&mut self) {
        let h = self.len();
        let w = self[0].len();
        for y in 0..w {
            let mut last_block = 0;
            for x in 0..h {
                let v = self[x][y];
                if v == 'O' {
                    if last_block != x {
                        self[last_block][y] = 'O';
                        self[x][y] = '.';
                    }
                    last_block += 1;
                } else if v == '#' {
                    last_block = x + 1;
                }
            }
        }
    }

    fn push_west(&mut self) {
        let h = self.len();
        let w = self[0].len();
        for x in 0..h {
            let mut last_block = 0;
            for y in 0..w {
                let v = self[x][y];
                if v == 'O' {
                    if last_block != y {
                        self[x][last_block] = 'O';
                        self[x][y] = '.';
                    }
                    last_block += 1;
                } else if v == '#' {
                    last_block = y + 1;
                }
            }
        }
    }

    fn push_south(&mut self) {
        let h = self.len();
        let w = self[0].len();
        for y in 0..w {
            let mut last_block = h - 1;
            for x in (0..h).rev() {
                let v = self[x][y];
                if v == 'O' {
                    if last_block != x {
                        self[last_block][y] = 'O';
                        self[x][y] = '.';
                    }
                    if last_block != 0 {
                        last_block -= 1;
                    }
                } else if v == '#' {
                    if x != 0 {
                        last_block = x - 1;
                    }
                }
            }
        }
    }

    fn push_east(&mut self) {
        let h = self.len();
        let w = self[0].len();
        for x in 0..h {
            let mut last_block = w - 1;
            for y in (0..w).rev() {
                let v = self[x][y];
                if v == 'O' {
                    if last_block != y {
                        self[x][last_block] = 'O';
                        self[x][y] = '.';
                    }
                    if last_block != 0 {
                        last_block -= 1;
                    }
                } else if v == '#' {
                    if y != 0 {
                        last_block = y - 1;
                    }
                }
            }
        }
    }

    fn get_weight(&self) -> usize {
        let h = self.len();
        let w = self[0].len();
        (0..w).fold(0, |acc, y| {
            let mut tot = 0;
            let mut last_block = 0;
            (0..h).for_each(|x| {
                match self[x][y] {
                    'O' => {
                        tot += h - last_block;
                        last_block += 1
                    }
                    '#' => last_block = x + 1,
                    '.' => (),
                    _ => unreachable!(),
                };
            });
            acc + tot
        })
    }
    fn get_weight_no_move(&self) -> usize {
        let h = self.len();
        self.iter().enumerate().fold(0, |acc, (x, line)| {
            acc + line
                .iter()
                .fold(0, |tot, c| tot + if c == &'O' { h - x } else { 0 })
        })
    }

    fn do_cycle(&mut self) {
        self.push_north();
        self.push_west();
        self.push_south();
        self.push_east();
    }

    fn excecute_rotations(&mut self) -> usize {
        let mut memo = HashMap::new();
        let mut cycle = 0;

        while cycle < NBR_CYCLES {
            if let Some(past_cycle) = memo.insert(self.clone(), cycle) {
                let remaining = NBR_CYCLES - cycle - 1;
                let to_find = remaining % (cycle - past_cycle);
                let seen = memo
                    .iter()
                    .find(|(_, idx)| **idx == to_find + past_cycle + 1)
                    .expect("Should be present")
                    .0;

                return seen.get_weight_no_move();
            }
            self.do_cycle();
            cycle += 1;
        }
        unreachable!()
    }
}

pub(crate) fn eval_file(file: &str) -> usize {
    let map: Map = file.into();
    map.get_weight()
}

pub(crate) fn eval_file_2(file: &str) -> usize {
    let mut map: Map = file.into();
    map.excecute_rotations()
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
        r#"
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#
    }
    #[test]
    fn test_0() {
        assert_eq!(136, eval_file(data()));
    }
    #[test]
    fn test_1() {
        assert_eq!(64, eval_file_2(data()));
    }
}
