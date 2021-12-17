mod daily_problems;
mod input;
mod utils;

pub use input::read_input::AocBufReader;
pub use daily_problems::day_16::solutions::{part_1, part_2};


fn main() {
    let aoc_reader = AocBufReader::from_str("src/data/day_16_pt_1.txt");
    let result: usize = part_2(aoc_reader);
    println!("{}", result);
}