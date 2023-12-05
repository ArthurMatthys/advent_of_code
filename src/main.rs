mod year2023;

use std::fs;

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Year of the program
    #[arg(short, long)]
    pub year: u32,

    /// Day of the program
    #[arg(short, long)]
    pub day: u8,

    /// Exercice number
    #[arg(short, long)]
    pub exercice: u8,

    /// path to the file
    #[arg(short, long)]
    pub filename: String,
}

fn main() {
    let args = Args::parse();
    let file = fs::read(args.filename).expect("cannot open file");

    let content = std::str::from_utf8(&file).expect("cannot convert to utf8");

    println!(
        "result : {}",
        match (args.year, args.day, args.exercice) {
            (2023, 0, 0) => year2023::day01::eval_file(content),
            (2023, 1, 0) => year2023::day02::eval_file(content),
            (2023, 1, 1) => year2023::day02::eval_file_2(content),
            (2023, 3, 0) => year2023::day04::eval_file(content),
            (2023, 3, 1) => year2023::day04::eval_file_2(content),
            // (0, 1) => day_01::top_three(content),
            // (1, 0) => day_02::count_points(content),
            // (1, 1) => day_02::count_points_with_result(content),
            // (2, 0) => day_03::get_priorities(content),
            // (2, 1) => day_03::get_badges(content),
            // (3, 0) => day_04::get_pairs_number(content),
            // (3, 1) => day_04::get_overlaps(content),
            _ => unreachable!(),
        }
    );
}
