const NUMBERS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn line_value(line: &str) -> i32 {
    let first = line.find(char::is_numeric).unwrap_or(usize::MAX);
    let first_nbr_idx = NUMBERS
        .iter()
        .enumerate()
        .filter_map(|(nbr, repr)| line.find(repr).map(|idx_find| (nbr, idx_find)))
        .min_by_key(|x| x.1)
        .unwrap_or((0, usize::MAX));
    let first_nbr = if first_nbr_idx.1 < first {
        first_nbr_idx.0 as i32
    } else {
        line.get(first..first + 1)
            .expect("index of str")
            .parse::<i32>()
            .expect("figure can be parsed")
    };
    let last = line.rfind(char::is_numeric).unwrap_or(0);
    let last_nbr_idx = NUMBERS
        .iter()
        .enumerate()
        .filter_map(|(nbr, repr)| line.rfind(repr).map(|idx_find| (nbr, idx_find)))
        .max_by_key(|x| x.1)
        .unwrap_or((0, 0));
    let last_nbr = if last_nbr_idx.1 > last {
        last_nbr_idx.0 as i32
    } else {
        line.get(last..last + 1)
            .expect("index of str")
            .parse::<i32>()
            .expect("figure can be parsed")
    };

    first_nbr * 10 + last_nbr
}

pub(crate) fn eval_file(file: &str) -> i32 {
    file.split_whitespace()
        .fold(0, |acc, line| acc + line_value(line))
}

pub(crate) fn print_sol_1(file: &str) {
    print!("res : {}", eval_file(file));
}

#[cfg(test)]
mod tests {
    use super::eval_file;

    #[test]
    fn first_test() {
        let v = r#"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
        assert_eq!(142, eval_file(v));
    }

    #[test]
    fn second_test() {
        let v = r#"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"#;
        assert_eq!(281, eval_file(v));
    }
}
