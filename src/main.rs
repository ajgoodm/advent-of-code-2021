mod input;
mod day_2;

pub use input::read_input::AocBufReader;
pub use day_2::day_2::{part_1};


fn main() {
    let aoc_reader = AocBufReader::from_str("src/data/day_2_pt_1.txt");
    let result: isize = part_1(aoc_reader);
    println!("{}", result);
}
