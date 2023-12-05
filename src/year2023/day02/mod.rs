use std::collections::HashMap;

fn dict() -> HashMap<&'static str, i32> {
    let mut hash = HashMap::new();
    hash.insert("red", 12);
    hash.insert("green", 13);
    hash.insert("blue", 14);
    hash
}

fn get_line_value(line: &str) -> Option<i32> {
    if line.is_empty() {
        return None;
    }
    // eprintln!("line : {}", line);
    let dict = dict();
    let mut split = line.split(':');
    let game_and_nbr = split.next().expect("at leat 2 part");
    let rest = split.next().expect("at leat 2 part");
    if rest.split(';').all(|sets| {
        sets.split(',').all(|color_and_nbr| {
            // eprintln!("color_and_nbr : {}", color_and_nbr);
            let mut color_and_nbr = color_and_nbr.split_whitespace();
            let nbr = color_and_nbr
                .next()
                .expect("Need nbr")
                .parse::<i32>()
                .expect("Should be number");
            let color = color_and_nbr.next().expect("Need color");
            // eprintln!("expected : {:?}, got {}", dict.get(color), nbr);
            dict.get(color).expect("Should be a valid color") >= &nbr
        })
    }) {
        // eprintln!("final : {}", game_and_nbr);
        let mut split = game_and_nbr.split_whitespace();
        let _ = split.next();
        Some(
            split
                .next()
                .expect("Should have a nbr")
                .parse::<i32>()
                .expect("Should be a nbr"),
        )
    } else {
        None
    }
}

#[derive(Default)]
struct Color {
    red: i32,
    blue: i32,
    green: i32,
}

impl Color {
    fn to_res(&self) -> i32 {
        self.red * self.blue * self.green
    }

    fn to_max(&mut self, color: &str, nbr: i32) {
        match color {
            "red" => self.red = i32::max(self.red, nbr),
            "blue" => self.blue = i32::max(self.blue, nbr),
            "green" => self.green = i32::max(self.green, nbr),
            _ => unreachable!(),
        };
    }
}

fn get_min_requiered(line: &str) -> i32 {
    if line.is_empty() {
        return 0;
    }
    let mut colors = Color::default();

    let mut split = line.split(':');
    let _ = split.next().expect("at leat 2 part");
    let rest = split.next().expect("at leat 2 part");
    rest.split(';').for_each(|sets| {
        sets.split(',').for_each(|color_and_nbr| {
            // eprintln!("color_and_nbr : {}", color_and_nbr);
            let mut color_and_nbr = color_and_nbr.split_whitespace();
            let nbr = color_and_nbr
                .next()
                .expect("Need nbr")
                .parse::<i32>()
                .expect("Should be number");
            let color = color_and_nbr.next().expect("Need color");
            colors.to_max(color, nbr);
        });
    });
    colors.to_res()
}

pub(crate) fn eval_file(file: &str) -> i32 {
    file.lines()
        .filter_map(|line| get_line_value(line))
        .fold(0, |acc, v| acc + v)
}

pub(crate) fn eval_file_2(file: &str) -> i32 {
    file.lines()
        .map(|line| get_min_requiered(line))
        .fold(0, |acc, v| acc + v)
}

#[cfg(test)]
mod tests {
    use crate::year2023::day02::{eval_file, eval_file_2};

    #[test]
    fn test_0() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        assert_eq!(8, eval_file(input));
    }
    #[test]
    fn test_1() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        assert_eq!(2286, eval_file_2(input));
    }
}
