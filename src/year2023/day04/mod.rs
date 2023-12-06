fn eval_win(line: &str) -> usize {
    let mut card_content = line.split(": ");
    let _ = card_content.next();
    let content = card_content.next().expect("should have content");
    let mut winning_nbrs_and_my_nbrs = content.split(" | ");
    let winning_cards = winning_nbrs_and_my_nbrs.next().expect("first of two parts");
    let my_cards = winning_nbrs_and_my_nbrs
        .next()
        .expect("second of two parts");
    let winning_cards = winning_cards
        .split(' ')
        .filter(|nbr| !nbr.is_empty())
        .map(|nbr| nbr.trim().parse::<i32>().expect("should be nbr"))
        .collect::<Vec<_>>();
    let my_cards = my_cards
        .split(' ')
        .filter(|nbr| !nbr.is_empty())
        .map(|nbr| nbr.trim().parse::<i32>().expect("should be nbr"))
        .collect::<Vec<_>>();
    let count = my_cards
        .iter()
        .filter(|nbr| winning_cards.contains(&nbr))
        .count();
    count
}

fn eval_file(file: &str) -> i32 {
    file.lines()
        .filter(|line| !line.is_empty())
        .fold(0, |acc, line| {
            let nbr_match = eval_win(line);
            acc + if nbr_match == 0 {
                0
            } else {
                1 << nbr_match - 1
            }
        })
}

fn eval_file_2(file: &str) -> i32 {
    let matching_by_line = file
        .lines()
        .filter(|line| !line.is_empty())
        .map(eval_win)
        .collect::<Vec<_>>();
    let nbr_lines = matching_by_line.len();
    let mut card_count = vec![1; nbr_lines];
    (0..nbr_lines).for_each(|idx| {
        let count = card_count.get(idx).expect("In table").clone();
        let matching_card = matching_by_line.get(idx).expect("In table");
        for card_idx in 0..*matching_card {
            let to_add = card_count.get_mut(idx + card_idx + 1).expect("In table");
            *to_add = *to_add + count;
        }
    });
    card_count.iter().sum()
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
        r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#
    }
    #[test]
    fn test_0() {
        assert_eq!(13, eval_file(data()));
    }
    #[test]
    fn test_1() {
        assert_eq!(30, eval_file_2(data()));
    }
    //     #[test]
    //     fn test_1() {
    //         let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    // Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    // Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    // Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    // Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
    //         assert_eq!(2286, eval_file_2(input));
    //     }
}
