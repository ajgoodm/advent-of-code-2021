mod daily_problems;
mod input;
mod utils;

pub use input::read_input::AocBufReader;
pub use daily_problems::day_20::solutions::{part_1};


fn main() {
    let aoc_reader = AocBufReader::from_str("src/data/day_20_pt_1_test.txt");
    let result: usize = part_1(aoc_reader);
    println!("{}", result);
}