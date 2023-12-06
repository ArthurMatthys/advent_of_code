pub(crate) fn eval_file(file: &str) -> i32 {
    0
}

pub(crate) fn eval_file_2(file: &str) -> i32 {
    0
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
        r#""#
    }
    #[test]
    fn test_0() {
        assert_eq!(0, eval_file(data()));
    }
    #[test]
    fn test_1() {
        assert_eq!(0, eval_file_2(data()));
    }
}
