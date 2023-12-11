use std::ops::Deref;

use itertools::Itertools;

struct Map(Vec<Vec<char>>);

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        Self(
            value
                .lines()
                .filter(|l| !l.is_empty())
                .map(|line| line.chars().collect::<Vec<_>>())
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

#[derive(Clone)]
struct Coord {
    x: usize,
    y: usize,
}

impl Map {
    fn get_stars(&self) -> Vec<Coord> {
        self.iter()
            .enumerate()
            .flat_map(|(x, line)| {
                line.iter()
                    .enumerate()
                    .filter_map(move |(y, c)| {
                        if c == &'#' {
                            Some(Coord { x, y })
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }
    fn get_empty_rows(&self) -> Vec<usize> {
        self.iter()
            .enumerate()
            .filter_map(|(i, line)| {
                if line.iter().all(|c| c == &'.') {
                    Some(i)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }
    fn get_empty_cols(&self) -> Vec<usize> {
        (0..self[0].len())
            .filter_map(|j| {
                if (0..self.len()).all(|i| self[i][j] == '.') {
                    Some(j)
                } else {
                    None
                }
            })
            .collect()
    }

    fn eval_min_distances(&self, size_empty_galaxies: usize) -> usize {
        let stars_coord = self.get_stars();

        let empty_rows = self.get_empty_rows();
        let empty_cols = self.get_empty_cols();

        stars_coord
            .into_iter()
            .tuple_combinations::<(Coord, Coord)>()
            .fold(0, |acc, (c1, c2)| {
                let max_x = c1.x.max(c2.x);
                let min_x = c1.x.min(c2.x);
                let max_y = c1.y.max(c2.y);
                let min_y = c1.y.min(c2.y);
                let dx = max_x - min_x;
                let dy = max_y - min_y;
                let dx_to_add = empty_rows
                    .iter()
                    .filter(|r| min_x <= **r && **r <= max_x)
                    .count();
                let dy_to_add = empty_cols
                    .iter()
                    .filter(|c| min_y <= **c && **c <= max_y)
                    .count();
                acc + dx + dy + (dx_to_add + dy_to_add) * (size_empty_galaxies - 1)
            })
    }
}

// fn stars_coord(map: )

pub(crate) fn eval_file(file: &str) -> usize {
    let map: Map = file.into();
    map.eval_min_distances(2)
}

pub(crate) fn eval_file_2(file: &str) -> usize {
    let map: Map = file.into();
    map.eval_min_distances(1000000)
}
pub(crate) fn print_sol_1(file: &str) {
    print!("res : {}", eval_file(file));
}
pub(crate) fn print_sol_2(file: &str) {
    print!("res : {}", eval_file_2(file));
}

#[cfg(test)]
mod tests {
    use super::{eval_file, Map};

    fn data() -> &'static str {
        r#"
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#
    }
    #[test]
    fn test_0() {
        assert_eq!(374, eval_file(data()));
    }
    #[test]
    fn test_1() {
        let data = data();
        let map: Map = data.into();

        assert_eq!(1030, map.eval_min_distances(10));
        assert_eq!(8410, map.eval_min_distances(100));
    }
}
