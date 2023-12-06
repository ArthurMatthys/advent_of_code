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

        (2023, 1, 0) => year2023::day01::print_sol_1(content),
        (2023, 2, 0) => year2023::day02::print_sol_1(content),
        (2023, 2, 1) => year2023::day02::print_sol_2(content),
        (2023, 4, 0) => year2023::day04::print_sol_1(content),
        (2023, 4, 1) => year2023::day04::print_sol_2(content),
        (2023, 5, 0) => year2023::day05::print_sol_1(content),
        (2023, 5, 1) => year2023::day05::print_sol_2(content),
        (2023, 6, 0) => year2023::day06::print_sol_1(content),
        (2023, 6, 1) => year2023::day06::print_sol_2(content),
        _ => unreachable!(),
    };
}
