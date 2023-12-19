use std::{fmt::Debug, marker::PhantomData};

#[derive(Debug)]
enum Direction {
    East,
    North,
    West,
    South,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "R" => Self::East,
            "D" => Self::South,
            "L" => Self::West,
            "U" => Self::North,
            _ => unreachable!(),
        }
    }
}

enum NormalParse {}
enum ColoredParse {}

#[derive(Debug)]
struct Row<T> {
    pub(crate) dir: Direction,
    pub(crate) length: u32,
    data: PhantomData<T>,
}

impl From<&str> for Row<NormalParse> {
    fn from(value: &str) -> Self {
        let mut split = value.split(' ');
        let dir: Direction = split.next().expect("Have a direction").into();
        let length = split
            .next()
            .expect("Should have length")
            .parse()
            .expect("Is a nbr");
        Self {
            dir,
            length,
            data: PhantomData::default(),
        }
    }
}
impl From<&str> for Row<ColoredParse> {
    fn from(value: &str) -> Self {
        let mut split = value.split(' ').skip(2);
        let mut color = split
            .next()
            .expect("Should have color")
            .trim_matches('(')
            .trim_matches(')')
            .chars()
            .skip(1);

        let tot = (0..5).fold(0, |acc, _| {
            acc * 16
                + color
                    .next()
                    .expect("should be present")
                    .to_digit(16)
                    .expect("Should be nbr")
        });
        let dir = match color.next() {
            Some('0') => Direction::East,
            Some('1') => Direction::South,
            Some('2') => Direction::West,
            Some('3') => Direction::North,
            _ => unreachable!(),
        };
        Self {
            dir,
            length: tot,
            data: Default::default(),
        }
    }
}

#[derive(Debug)]
struct Map {
    pub(crate) map: Vec<(i64, i64)>,
    pub(crate) area: u32,
}

impl<T> From<Vec<Row<T>>> for Map {
    fn from(rows: Vec<Row<T>>) -> Self {
        let mut x = 0;
        let mut y = 0;
        let mut map = vec![(0, 0)];
        let mut area = 2;
        for r in rows.iter() {
            let dir: (i64, i64) = match r.dir {
                Direction::East => (0, 1),
                Direction::North => (-1, 0),
                Direction::West => (0, -1),
                Direction::South => (1, 0),
            };
            x += dir.0 * r.length as i64;
            y += dir.1 * r.length as i64;
            area += r.length;
            map.push((x, y))
        }
        Self {
            map,
            area: area / 2,
        }
    }
}

impl Map {
    fn eval_shoelace(&self) -> i64 {
        self.map
            .windows(2)
            .fold(0, |acc, coords| {
                acc + coords[0].0 * coords[1].1 - coords[0].1 * coords[1].0
            })
            .abs()
            / 2
    }
}

pub(crate) fn eval_file(file: &str) -> i64 {
    let rows = file
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.into())
        .collect::<Vec<Row<NormalParse>>>();
    let map: Map = rows.into();
    // eprintln!("h : {} // w : {}", map.height, map.width);
    // map.print();
    map.eval_shoelace() + map.area as i64
    // eprintln!("{map:?}");
    // 0
}

pub(crate) fn eval_file_2(file: &str) -> i64 {
    let rows = file
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.into())
        .collect::<Vec<Row<ColoredParse>>>();
    let map: Map = rows.into();
    map.eval_shoelace() + map.area as i64
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
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#
    }
    #[test]
    fn test_0() {
        assert_eq!(62, eval_file(data()));
    }
    #[test]
    fn test_1() {
        assert_eq!(952408144115, eval_file_2(data()));
    }
}
