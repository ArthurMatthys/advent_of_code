struct Reading(Vec<i32>);

impl From<&str> for Reading {
    fn from(value: &str) -> Self {
        Self(
            value
                .split(' ')
                .map(|s| s.parse::<i32>().expect("Should be nbr"))
                .collect(),
        )
    }
}

impl Reading {
    fn extrapolate_right(self) -> i32 {
        let mut current_nbrs = self.0;
        let mut lasts = vec![];
        while current_nbrs.iter().any(|v| *v != 0) {
            lasts.push(current_nbrs.last().expect("At least one number").clone());
            current_nbrs = current_nbrs
                .iter()
                .zip(current_nbrs.iter().skip(1))
                .map(|(a, b)| b - a)
                .collect();
        }
        lasts.iter().sum()
    }

    fn extrapolate_left(self) -> i32 {
        let mut current_nbrs = self.0;
        let mut firsts = vec![];
        while current_nbrs.iter().any(|v| *v != 0) {
            firsts.push(current_nbrs.first().expect("At least one number").clone());
            current_nbrs = current_nbrs
                .iter()
                .zip(current_nbrs.iter().skip(1))
                .map(|(a, b)| b - a)
                .collect();
        }
        firsts.iter().rev().fold(0, |acc, v| v - acc)
    }
}

struct Readings(Vec<Reading>);

impl From<&str> for Readings {
    fn from(value: &str) -> Self {
        Self(
            value
                .lines()
                .filter(|s| !s.is_empty())
                .map(|line| line.into())
                .collect(),
        )
    }
}

impl Readings {
    fn extrapolate_right(self) -> i32 {
        self.0.into_iter().map(|v| v.extrapolate_right()).sum()
    }
    fn extrapolate_left(self) -> i32 {
        self.0.into_iter().map(|v| v.extrapolate_left()).sum()
    }
}

pub(crate) fn eval_file(file: &str) -> i32 {
    let r: Readings = file.into();
    r.extrapolate_right()
}

pub(crate) fn eval_file_2(file: &str) -> i32 {
    let r: Readings = file.into();
    r.extrapolate_left()
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
        r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#
    }
    #[test]
    fn test_0() {
        assert_eq!(114, eval_file(data()));
    }
    #[test]
    fn test_1() {
        assert_eq!(2, eval_file_2(data()));
    }
}
