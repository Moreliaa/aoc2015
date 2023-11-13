use aoc_lib::input_reader;
mod day1;
mod day2;
mod day3;

fn main() {
    day1::run(input("1"));
    day2::run(input("2"));
    day3::run(input("3"));
}

fn input(day: &str) -> String {
    input_reader::get_input("2015", day, "./cookie.txt")
}
