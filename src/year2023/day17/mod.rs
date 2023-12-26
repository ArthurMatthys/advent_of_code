use std::{collections::BinaryHeap, ops::Deref};

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
enum Dir {
    E,
    N,
    W,
    S,
}

impl Dir {
    fn index(&self) -> usize {
        match self {
            Dir::E => 0,
            Dir::N => 1,
            Dir::W => 2,
            Dir::S => 3,
        }
    }
    fn is_opp(&self, other: &Self) -> bool {
        match (self, other) {
            (Dir::N, Dir::S) | (Dir::E, Dir::W) | (Dir::W, Dir::E) | (Dir::S, Dir::N) => true,
            _ => false,
        }
    }
}

struct Map {
    pub(crate) map: Vec<u32>,
    pub(crate) rows: usize,
    pub(crate) cols: usize,
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let value = value.trim();
        let cols = value.find('\n').unwrap();
        let map = value
            .lines()
            .filter(|l| !l.is_empty())
            .flat_map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).expect("Is nbr"))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let rows = map.len() / cols;
        Self { map, rows, cols }
    }
}

impl Deref for Map {
    type Target = Vec<u32>;
    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Step {
    pub(crate) coord: usize,
    pub(crate) heat_loss: u32,
    pub(crate) movement: (Dir, usize),
    pub(crate) past: Vec<usize>,
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.heat_loss.partial_cmp(&self.heat_loss)
    }
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.heat_loss.cmp(&self.heat_loss)
    }
}

impl Map {
    fn solve<const MIN: usize, const MAX: usize>(&self) -> u32 {
        let size = self.len();
        let mut seen = vec![(false, u32::MAX); size * 4 * MAX as usize];
        let mut queue = BinaryHeap::from([
            Step {
                coord: self.rows,
                heat_loss: self.map[self.rows],
                movement: (Dir::S, 0),
                past: vec![self.rows],
            },
            Step {
                coord: 1,
                heat_loss: self.map[1],
                movement: (Dir::E, 0),
                past: vec![1],
            },
        ]);
        while let Some(step) = queue.pop() {
            if step.coord == size - 1 && step.movement.1 >= MIN {
                return step.heat_loss;
            }
            queue.extend(
                [Dir::N, Dir::E, Dir::W, Dir::S]
                    .into_iter()
                    .filter_map(|dir| {
                        if step.movement.0.is_opp(&dir)
                            || (dir == step.movement.0 && step.movement.1 + 1 >= MAX)
                            || (dir != step.movement.0 && step.movement.1 < MIN)
                            || match dir {
                                Dir::E => step.coord % self.cols == self.cols - 1,
                                Dir::N => step.coord < self.cols,
                                Dir::W => step.coord % self.cols == 0,
                                Dir::S => step.coord / self.cols == self.rows - 1,
                            }
                        {
                            return None;
                        }
                        let movement = if dir == step.movement.0 {
                            (dir.clone(), step.movement.1 + 1)
                        } else {
                            (dir.clone(), 0)
                        };
                        let coord = match dir {
                            Dir::E => step.coord + 1,
                            Dir::N => step.coord - self.cols,
                            Dir::W => step.coord - 1,
                            Dir::S => step.coord + self.cols,
                        };
                        let heat_loss = step.heat_loss + self.map[coord];
                        let idx = (coord) * 4 * MAX + movement.0.index() * MAX + (movement.1);
                        if !seen[idx as usize].0 {
                            seen[idx as usize] = (true, heat_loss);
                            let mut past = step.past.clone();
                            past.push(coord);
                            Some(Step {
                                coord,
                                heat_loss,
                                movement,
                                past,
                            })
                        } else {
                            None
                        }
                    }),
            );
        }
        0
        // *seen[(size - 1) * MAX * 4 + MIN..]
        //     .iter()
        //     .map(|(_, v)| v)
        //     .min()
        //     .unwrap()
    }
}

pub(crate) fn eval_file(file: &str) -> u32 {
    let map: Map = file.into();
    map.solve::<0, 3>()
}

pub(crate) fn eval_file_2(file: &str) -> u32 {
    let map: Map = file.into();
    map.solve::<3, 10>()
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
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#
    }

    fn data_1() -> &'static str {
        r#"
111111111111
999999999991
999999999991
999999999991
999999999991"#
    }
    #[test]
    fn test_0() {
        assert_eq!(102, eval_file(data()));
    }
    #[test]
    fn test_1() {
        assert_eq!(94, eval_file_2(data()));
    }
    #[test]
    fn test_2() {
        assert_eq!(71, eval_file_2(data_1()));
    }
}
