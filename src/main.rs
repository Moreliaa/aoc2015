use aoc_lib::input_reader;
use std::env;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut run_all = false;

    if args.len() > 1 && (args[1] == "a" || args[1] == "all") {
        run_all = true;
    }
    if run_all {
        day1::run(input("1"));
        day2::run(input("2"));
        day3::run(input("3"));
        day4::run();
        day5::run(input("5"));
        day6::run(input("6"));
        day7::run(input("7"));
    }
    day8::run(input("8"));
}

fn input(day: &str) -> String {
    input_reader::get_input("2015", day, "./cookie.txt")
}
