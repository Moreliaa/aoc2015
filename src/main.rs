use aoc_lib::input_reader;
mod day1;

fn main() {
    let year = "2015";
    let path_to_cookie = "./cookie.txt";
    day1::run(input_reader::get_input(year, "1", path_to_cookie));
}
