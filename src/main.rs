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

    match (args.year, args.day, args.exercice) {
        (2023, 1, 0) => year2023::day01::print_sol_1(content),
        (2023, 2, 0) => year2023::day02::print_sol_1(content),
        (2023, 2, 1) => year2023::day02::print_sol_2(content),
        (2023, 3, 0) => year2023::day03::print_sol_1(content),
        (2023, 3, 1) => year2023::day03::print_sol_2(content),
        (2023, 4, 0) => year2023::day04::print_sol_1(content),
        (2023, 4, 1) => year2023::day04::print_sol_2(content),
        (2023, 5, 0) => year2023::day05::print_sol_1(content),
        (2023, 5, 1) => year2023::day05::print_sol_2(content),
        (2023, 6, 0) => year2023::day06::print_sol_1(content),
        (2023, 6, 1) => year2023::day06::print_sol_2(content),
        (2023, 7, 0) => year2023::day07::print_sol_1(content),
        (2023, 7, 1) => year2023::day07::print_sol_2(content),
        (2023, 8, 0) => year2023::day08::print_sol_1(content),
        (2023, 8, 1) => year2023::day08::print_sol_2(content),
        (2023, 9, 0) => year2023::day09::print_sol_1(content),
        (2023, 9, 1) => year2023::day09::print_sol_2(content),
        (2023, 10, 0) => year2023::day10::print_sol_1(content),
        (2023, 10, 1) => year2023::day10::print_sol_2(content),
        (2023, 11, 0) => year2023::day11::print_sol_1(content),
        (2023, 11, 1) => year2023::day11::print_sol_2(content),
        (2023, 12, 0) => year2023::day12::print_sol_1(content),
        (2023, 12, 1) => year2023::day12::print_sol_2(content),
        (2023, 13, 0) => year2023::day13::print_sol_1(content),
        (2023, 13, 1) => year2023::day13::print_sol_2(content),
        (2023, 14, 0) => year2023::day14::print_sol_1(content),
        (2023, 14, 1) => year2023::day14::print_sol_2(content),
        (2023, 15, 0) => year2023::day15::print_sol_1(content),
        (2023, 15, 1) => year2023::day15::print_sol_2(content),
        _ => unreachable!(),
    };
}
