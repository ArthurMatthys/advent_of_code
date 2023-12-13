use std::ops::{Deref, DerefMut};

struct Map(Vec<Vec<char>>);
#[derive(PartialEq, Eq, Clone, Debug, Hash)]
struct Coord((i32, i32));

impl Coord {
    fn x(&self) -> i32 {
        self.0 .0
    }
    fn y(&self) -> i32 {
        self.0 .1
    }

    fn new_coord(&self, d_x: i32, d_y: i32, h: i32, w: i32) -> Option<Self> {
        let x = d_x + self.x();
        let y = d_y + self.y();
        if x < 0 || x >= h {
            None
        } else if y < 0 || y >= w {
            None
        } else {
            Some(Self((x, y)))
        }
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

fn explore_loop(
    map: &Map,
    previous_pos: Coord,
    actual_pos: Coord,
    h: i32,
    w: i32,
    visited: &mut Vec<Coord>,
) -> bool {
    let new_pos = map[actual_pos.x() as usize][actual_pos.y() as usize];

    let (p1, p2) = match new_pos {
        '|' => ((-1, 0), (1, 0)),
        'L' => ((-1, 0), (0, 1)),
        'J' => ((-1, 0), (0, -1)),
        '-' => ((0, 1), (0, -1)),
        'F' => ((0, 1), (1, 0)),
        '7' => ((0, -1), (1, 0)),
        '.' => return false,
        'S' => return true,
        _ => unreachable!(),
    };
    if visited.contains(&actual_pos) {
        return false;
    } else {
        visited.push(actual_pos.clone());
    }
    let new_coord_1 = actual_pos.new_coord(p1.0, p1.1, h, w);
    let new_coord_2 = actual_pos.new_coord(p2.0, p2.1, h, w);
    if new_coord_1.as_ref() != Some(&previous_pos) && new_coord_2.as_ref() != Some(&previous_pos) {
        return false;
    } else if new_coord_1.is_some() && new_coord_1 != Some(previous_pos) {
        explore_loop(
            map,
            actual_pos,
            new_coord_1.expect("Is some"),
            h,
            w,
            visited,
        )
    } else {
        explore_loop(
            map,
            actual_pos,
            new_coord_2.expect("Is some"),
            h,
            w,
            visited,
        )
    }
}

fn explore_map(map: &Map, starting_pos: Coord) -> Vec<Coord> {
    let h = map.len() as i32;
    let w = map[0].len() as i32;

    let visited = Vec::from([starting_pos.clone()]);
    if let Some(new_coord) = starting_pos.new_coord(1, 0, h, w) {
        let mut cpy = visited.clone();
        if explore_loop(&map, starting_pos.clone(), new_coord, h, w, &mut cpy) {
            return cpy;
        }
    }
    if let Some(new_coord) = starting_pos.new_coord(-1, 0, h, w) {
        let mut cpy = visited.clone();
        if explore_loop(&map, starting_pos.clone(), new_coord, h, w, &mut cpy) {
            return cpy;
        }
    }
    if let Some(new_coord) = starting_pos.new_coord(0, 1, h, w) {
        let mut cpy = visited.clone();
        if explore_loop(&map, starting_pos.clone(), new_coord, h, w, &mut cpy) {
            return cpy;
        }
    }
    unreachable!()
}

pub(crate) fn eval_file(file: &str) -> usize {
    let map: Map = Map(file
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| line.chars().collect())
        .collect());
    let s_pos = Coord(
        map.iter()
            .enumerate()
            .find_map(|(i, line)| {
                if let Some(pos) = line.iter().position(|c| c == &'S') {
                    Some((i as i32, pos as i32))
                } else {
                    None
                }
            })
            .expect("Should have starting pos"),
    );
    explore_map(&map, s_pos).len() / 2
}

pub(crate) fn eval_file_2(file: &str) -> usize {
    let map: Map = Map(file
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| line.chars().collect())
        .collect());
    let s_pos = Coord(
        map.iter()
            .enumerate()
            .find_map(|(i, line)| {
                if let Some(pos) = line.iter().position(|c| c == &'S') {
                    Some((i as i32, pos as i32))
                } else {
                    None
                }
            })
            .expect("Should have starting pos"),
    );
    let mut loop_coords = explore_map(&map, s_pos.clone());
    loop_coords.push(s_pos);
    let len_loop = loop_coords.len();
    let area = loop_coords
        .iter()
        .collect::<Vec<_>>()
        .windows(2)
        .fold(0, |acc, w| {
            acc + (w[0].x() * w[1].y()) - (w[0].y() * w[1].x())
        })
        .abs()
        / 2;
    area as usize - len_loop / 2 + 1
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

    fn data_1() -> &'static str {
        r#"
-L|F7
7S-7|
L|7||
-L-J|
L|-JF"#
    }
    fn data_2() -> &'static str {
        r#"
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ"#
    }
    fn data_3() -> &'static str {
        r#"
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."#
    }
    fn data_4() -> &'static str {
        r#"
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."#
    }
    fn data_5() -> &'static str {
        r#"
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJIF7FJ-
L---JF-JLJIIIIFJLJJ7
|F|F-JF---7IIIL7L|7|
|FFJF7L7F-JF7IIL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"#
    }
    #[test]
    fn test_0() {
        assert_eq!(4, eval_file(data_1()));
        assert_eq!(8, eval_file(data_2()));
    }
    #[test]
    fn test_1() {
        assert_eq!(4, eval_file_2(data_3()));
    }
    #[test]
    fn test_2() {
        assert_eq!(8, eval_file_2(data_4()));
    }
    #[test]
    fn test_3() {
        assert_eq!(10, eval_file_2(data_5()));
    }
}
