use std::collections::{HashMap, HashSet};

#[derive(Debug)]
enum MirroDirection {
    /// /
    Slash,
    /// \
    Backslash,
}

#[derive(Debug)]
enum SplitterDirection {
    Horizontal,
    Vertical,
}

#[derive(Debug)]
enum Obstacle {
    Mirror(MirroDirection),
    Splitter(SplitterDirection),
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
enum Direction {
    NorthToSouth,
    SouthToNorth,
    EastToWest,
    WestToEast,
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct Ray {
    pub(crate) direction: Direction,
    pub(crate) starting_pos: Coord,
}

impl Ray {
    fn new(direction: Direction, starting_pos: Coord) -> Self {
        Self {
            direction,
            starting_pos,
        }
    }

    fn encounter_obstacle(self, position: Coord, obstacle: &Obstacle) -> Vec<Self> {
        match obstacle {
            Obstacle::Mirror(MirroDirection::Slash) => vec![match self.direction {
                Direction::NorthToSouth => Self {
                    direction: Direction::EastToWest,
                    starting_pos: position,
                },
                Direction::SouthToNorth => Self {
                    direction: Direction::WestToEast,
                    starting_pos: position,
                },
                Direction::EastToWest => Self {
                    direction: Direction::NorthToSouth,
                    starting_pos: position,
                },
                Direction::WestToEast => Self {
                    direction: Direction::SouthToNorth,
                    starting_pos: position,
                },
            }],
            Obstacle::Mirror(MirroDirection::Backslash) => vec![match self.direction {
                Direction::NorthToSouth => Self {
                    direction: Direction::WestToEast,
                    starting_pos: position,
                },
                Direction::SouthToNorth => Self {
                    direction: Direction::EastToWest,
                    starting_pos: position,
                },
                Direction::EastToWest => Self {
                    direction: Direction::SouthToNorth,
                    starting_pos: position,
                },
                Direction::WestToEast => Self {
                    direction: Direction::NorthToSouth,
                    starting_pos: position,
                },
            }],
            Obstacle::Splitter(SplitterDirection::Horizontal) => match self.direction {
                Direction::NorthToSouth | Direction::SouthToNorth => vec![
                    Self {
                        direction: Direction::WestToEast,
                        starting_pos: position.clone(),
                    },
                    Self {
                        direction: Direction::EastToWest,
                        starting_pos: position,
                    },
                ],
                Direction::EastToWest => vec![Self {
                    direction: Direction::EastToWest,
                    starting_pos: position,
                }],
                Direction::WestToEast => vec![Self {
                    direction: Direction::WestToEast,
                    starting_pos: position,
                }],
            },
            Obstacle::Splitter(SplitterDirection::Vertical) => match self.direction {
                Direction::EastToWest | Direction::WestToEast => vec![
                    Self {
                        direction: Direction::SouthToNorth,
                        starting_pos: position.clone(),
                    },
                    Self {
                        direction: Direction::NorthToSouth,
                        starting_pos: position,
                    },
                ],
                Direction::NorthToSouth => vec![Self {
                    direction: Direction::NorthToSouth,
                    starting_pos: position,
                }],
                Direction::SouthToNorth => vec![Self {
                    direction: Direction::SouthToNorth,
                    starting_pos: position,
                }],
            },
        }
    }
}

#[derive(Hash, PartialEq, Eq, Debug, Default, Clone)]
struct Coord {
    pub(crate) x: usize,
    pub(crate) y: usize,
}

impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn add_direction(self, dir: &Direction, max_h: usize, max_w: usize) -> Option<Self> {
        match dir {
            Direction::NorthToSouth => {
                if self.x < max_h {
                    Some(Self {
                        x: self.x + 1,
                        y: self.y,
                    })
                } else {
                    None
                }
            }
            Direction::SouthToNorth => {
                if self.x > 0 {
                    Some(Self {
                        x: self.x - 1,
                        y: self.y,
                    })
                } else {
                    None
                }
            }
            Direction::WestToEast => {
                if self.y < max_w {
                    Some(Self {
                        x: self.x,
                        y: self.y + 1,
                    })
                } else {
                    None
                }
            }
            Direction::EastToWest => {
                if self.y > 0 {
                    Some(Self {
                        x: self.x,
                        y: self.y - 1,
                    })
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Debug)]
struct Map {
    pub(crate) map: HashMap<Coord, Obstacle>,
    pub(crate) height: usize,
    pub(crate) width: usize,
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let mut ret = HashMap::new();
        let mut height = 0;
        let mut width = 0;
        value
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .enumerate()
            .for_each(|(x, l)| {
                height = height.max(x);

                l.chars().enumerate().for_each(|(y, c)| {
                    match c {
                        '-' => {
                            ret.insert(
                                Coord { x, y },
                                Obstacle::Splitter(SplitterDirection::Horizontal),
                            );
                        }
                        '|' => {
                            ret.insert(
                                Coord { x, y },
                                Obstacle::Splitter(SplitterDirection::Vertical),
                            );
                        }
                        '/' => {
                            ret.insert(Coord { x, y }, Obstacle::Mirror(MirroDirection::Slash));
                        }
                        '\\' => {
                            ret.insert(Coord { x, y }, Obstacle::Mirror(MirroDirection::Backslash));
                        }
                        '.' => (),
                        _ => unreachable!(),
                    };
                    if x == 0 {
                        width = width.max(y);
                    }
                })
            });

        Self {
            map: ret,
            height,
            width,
        }
    }
}

impl Map {
    fn resolve_ray(&self, seen: &mut HashSet<Ray>, ray: Ray, coords: &mut HashSet<Coord>) {
        let mut pos = ray.starting_pos.clone();
        let dir = ray.direction.clone();
        loop {
            if let Some(p) = pos.add_direction(&dir, self.height, self.width) {
                coords.insert(p.clone());
                if let Some(obs) = self.map.get(&p) {
                    ray.encounter_obstacle(p, obs).into_iter().for_each(|r| {
                        if !seen.contains(&r) {
                            seen.insert(r.clone());
                            self.resolve_ray(seen, r, coords)
                        }
                    });
                    break;
                }
                pos = p;
            } else {
                break;
            }
        }
    }
}

pub(crate) fn eval_file(file: &str) -> usize {
    let map: Map = file.into();
    let ray = Ray::new(Direction::WestToEast, Default::default());

    let mut coords = HashSet::new();
    let mut seen = HashSet::new();

    let starting_coord = Coord::default();
    coords.insert(Coord::default());
    seen.insert(ray.clone());

    if let Some(obs) = map.map.get(&starting_coord) {
        ray.encounter_obstacle(starting_coord, obs)
            .into_iter()
            .for_each(|r| {
                if !seen.contains(&r) {
                    seen.insert(r.clone());
                    map.resolve_ray(&mut seen, r, &mut coords)
                }
            });
    } else {
        map.resolve_ray(&mut seen, ray, &mut coords);
    }

    coords.len()
}

pub(crate) fn eval_file_2(file: &str) -> usize {
    let map: Map = file.into();
    let mut max_len = 0;

    for x in 0..(map.height) {
        {
            let starting_coord = Coord::new(x, map.width - 1);
            let ray = Ray::new(Direction::EastToWest, starting_coord.clone());

            let mut coords = HashSet::new();
            let mut seen = HashSet::new();

            coords.insert(starting_coord.clone());
            seen.insert(ray.clone());

            if let Some(obs) = map.map.get(&starting_coord) {
                ray.encounter_obstacle(starting_coord, obs)
                    .into_iter()
                    .for_each(|r| {
                        if !seen.contains(&r) {
                            seen.insert(r.clone());
                            map.resolve_ray(&mut seen, r, &mut coords)
                        }
                    });
            } else {
                map.resolve_ray(&mut seen, ray, &mut coords);
            }
            max_len = max_len.max(coords.len())
        }
        {
            let starting_coord = Coord::new(x, 0);
            let ray = Ray::new(Direction::WestToEast, starting_coord.clone());

            let mut coords = HashSet::new();
            let mut seen = HashSet::new();

            coords.insert(starting_coord.clone());
            seen.insert(ray.clone());

            if let Some(obs) = map.map.get(&starting_coord) {
                ray.encounter_obstacle(starting_coord, obs)
                    .into_iter()
                    .for_each(|r| {
                        if !seen.contains(&r) {
                            seen.insert(r.clone());
                            map.resolve_ray(&mut seen, r, &mut coords)
                        }
                    });
            } else {
                map.resolve_ray(&mut seen, ray, &mut coords);
            }
            max_len = max_len.max(coords.len())
        }
    }
    for y in 0..(map.width) {
        {
            let starting_coord = Coord::new(map.height - 1, y);
            let ray = Ray::new(Direction::SouthToNorth, starting_coord.clone());

            let mut coords = HashSet::new();
            let mut seen = HashSet::new();

            coords.insert(starting_coord.clone());
            seen.insert(ray.clone());

            if let Some(obs) = map.map.get(&starting_coord) {
                ray.encounter_obstacle(starting_coord, obs)
                    .into_iter()
                    .for_each(|r| {
                        if !seen.contains(&r) {
                            seen.insert(r.clone());
                            map.resolve_ray(&mut seen, r, &mut coords)
                        }
                    });
            } else {
                map.resolve_ray(&mut seen, ray, &mut coords);
            }
            max_len = max_len.max(coords.len())
        }
        {
            let starting_coord = Coord::new(0, y);
            let ray = Ray::new(Direction::NorthToSouth, starting_coord.clone());

            let mut coords = HashSet::new();
            let mut seen = HashSet::new();

            coords.insert(starting_coord.clone());
            seen.insert(ray.clone());

            if let Some(obs) = map.map.get(&starting_coord) {
                ray.encounter_obstacle(starting_coord, obs)
                    .into_iter()
                    .for_each(|r| {
                        if !seen.contains(&r) {
                            seen.insert(r.clone());
                            map.resolve_ray(&mut seen, r, &mut coords)
                        }
                    });
            } else {
                map.resolve_ray(&mut seen, ray, &mut coords);
            }
            max_len = max_len.max(coords.len())
        }
    }
    max_len
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
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
        "#
    }
    #[test]
    fn test_0() {
        assert_eq!(46, eval_file(data()));
    }
    #[test]
    fn test_1() {
        assert_eq!(51, eval_file_2(data()));
    }
}
