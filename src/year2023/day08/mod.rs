use std::collections::HashMap;

use num::integer::lcm;

#[derive(Debug)]
struct Dest<'a> {
    left: &'a str,
    right: &'a str,
}

impl<'a> From<&'a str> for Dest<'a> {
    fn from(value: &'a str) -> Self {
        let trimmed = value.trim_matches(|c| c == '(' || c == ')');
        let mut splited = trimmed.split(", ");
        Self {
            left: splited.next().expect("left present"),
            right: splited.next().expect("right present"),
        }
    }
}

impl<'a> Dest<'a> {
    fn get_dest(&self, c: Option<char>) -> &'a str {
        match c {
            Some('L') => self.left,
            Some('R') => self.right,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Map<'a> {
    moves: &'a str,
    map: HashMap<&'a str, Dest<'a>>,
}

struct Solver<'a> {
    pub(crate) positions: Vec<&'a str>,
    pub(crate) map: Map<'a>,
}

impl<'a> Solver<'a> {
    fn new<P>(map: Map<'a>, mut filter_start: P) -> Solver
    where
        P: FnMut(&&'a str) -> bool,
    {
        let positions = map
            .map
            .keys()
            .filter_map(|p| if filter_start(p) { Some(*p) } else { None })
            .collect::<Vec<_>>();

        Solver { positions, map }
    }
    fn is_finished<P>(&mut self, filter_end: &mut P) -> bool
    where
        P: FnMut(&&'a str) -> bool,
    {
        self.positions.iter().all(|p| filter_end(p))
    }

    fn eval_first_steps(&mut self) {
        self.positions.iter_mut().for_each(|p| {
            self.map.moves.chars().for_each(|c| {
                *p = self
                    .map
                    .map
                    .get(*p)
                    .expect("location present")
                    .get_dest(Some(c));
            });
        })
    }

    fn solve<P>(&mut self, mut filter_end: P) -> u32
    where
        P: FnMut(&&'a str) -> bool,
    {
        let mut nbr_step = 0;
        self.eval_first_steps();
        let mut moves = self.map.moves.chars().cycle();
        while !self.is_finished(&mut filter_end) {
            self.positions.iter_mut().for_each(|p| {
                *p = self
                    .map
                    .map
                    .get(*p)
                    .expect("location present")
                    .get_dest(moves.next())
            });
            nbr_step += 1;
        }
        nbr_step + self.map.moves.len() as u32
    }

    fn get_loop_len(self) -> Vec<usize> {
        self.positions
            .into_iter()
            .map(|pos| {
                let mut cycle = 0;
                let mut moves = self.map.moves.chars().cycle();
                let mut current_pos = pos;
                while let Some(next_move) = moves.next() {
                    current_pos = self
                        .map
                        .map
                        .get(current_pos)
                        .expect("Location present")
                        .get_dest(Some(next_move));
                    cycle += 1;
                    if current_pos.ends_with('Z') {
                        break;
                    }
                }
                cycle
            })
            .collect()
        // eprintln!("pos : {:?}", self.positions);
        // while checked > 0 && cycle < 5000 {
        //     cycle += 1;
        //     let next_move = moves.next();
        //     self.positions.iter_mut().enumerate().for_each(|(i, p)| {
        //         if done.contains(&i) {
        //             return;
        //         }
        //         let v = self
        //             .map
        //             .map
        //             .get(*p)
        //             .expect("location present")
        //             .get_dest(next_move);
        //         eprintln!("from {p:?} to {v:?}");
        //         if v.ends_with('A') {
        //             checked -= 1;
        //             done.push(i);
        //             ret[i] = Some(cycle)
        //         } else if v == *p {
        //             checked -= 1;
        //             done.push(i);
        //             ret[i] = Some(0)
        //         }
        //         *p = v
        //     });
        // }
        // ret
    }
}

impl<'a> From<&'a str> for Map<'a> {
    fn from(value: &'a str) -> Self {
        let mut lines = value.lines().filter(|l| !l.is_empty());
        let moves = lines.next().expect("Should have moves");
        let mut map = HashMap::new();
        while let Some(from_and_dest) = lines.next() {
            let mut from_and_dest = from_and_dest.split(" = ");
            let from = from_and_dest.next().expect("From should be present");
            map.insert(
                from,
                from_and_dest.next().expect("dest should be present").into(),
            );
        }

        Self { moves, map }
    }
}

pub(crate) fn eval_file(file: &str) -> u32 {
    let map: Map = file.into();
    let mut solver = Solver::new(map, |v| v == &"AAA");
    solver.solve(|v| v == &"ZZZ")
}

pub(crate) fn eval_file_2(file: &str) -> usize {
    let map: Map = file.into();
    let solver = Solver::new(map, |v| v.ends_with('A'));
    solver
        .get_loop_len()
        .into_iter()
        .fold(1, |acc, v| lcm(acc, v))
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
        r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#
    }
    fn data_1() -> &'static str {
        r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#
    }
    fn data_2() -> &'static str {
        r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#
    }
    #[test]
    fn test_0() {
        assert_eq!(2, eval_file(data()));
        assert_eq!(6, eval_file(data_1()));
    }
    #[test]
    fn test_1() {
        assert_eq!(6, eval_file_2(data_2()));
    }
}
