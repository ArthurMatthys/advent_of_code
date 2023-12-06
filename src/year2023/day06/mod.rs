fn analyse_function(time: i64, distance: i64) -> f64 {
    let sqrt_delta = ((time.pow(2) - 4 * distance) as f64).sqrt();
    let r1 = ((time as f64 + sqrt_delta) / 2. - 1.).ceil();
    let r2 = ((time as f64 - sqrt_delta) / 2. + 1.).floor();
    r1 - r2 + 1.
}

pub(crate) fn eval_file(file: &str) -> f64 {
    let nbrs = file
        .lines()
        .map(|line| {
            line.split(':')
                .skip(1)
                .flat_map(|nbrs| {
                    nbrs.split(' ')
                        .filter(|s| !s.is_empty())
                        .map(|s| s.parse::<i64>().expect("Should be nbr"))
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    nbrs[0]
        .iter()
        .zip(nbrs[1].clone())
        .fold(1., |acc, (time, distance)| {
            acc * analyse_function(*time, distance)
        })
}

pub(crate) fn eval_file_2(file: &str) -> f64 {
    let nbrs = file
        .lines()
        .map(|line| {
            line.split(':')
                .skip(1)
                .map(|nbrs| {
                    nbrs.split(' ').filter(|s| !s.is_empty()).fold(0, |acc, s| {
                        acc * (10_i64.pow(s.len() as u32))
                            + s.parse::<i64>().expect("Should be nbr")
                    })
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    nbrs[0]
        .iter()
        .zip(nbrs[1].clone())
        .fold(1., |acc, (time, distance)| {
            acc * analyse_function(*time, distance)
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
        r#"Time:      7  15   30
Distance:  9  40  200"#
    }
    #[test]
    fn test_0() {
        assert_eq!(288., eval_file(data()));
    }
    #[test]
    fn test_1() {
        assert_eq!(71503., eval_file_2(data()));
    }
}
