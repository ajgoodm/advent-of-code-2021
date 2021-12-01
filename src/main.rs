mod input;
mod day_1;

pub use input::read_input::AocBufReader;
pub use day_1::day_1::{part_1, part_2};


fn main() {
    let aoc_reader = AocBufReader::from_str("src/data/day_1_pt_1.txt");
    let result: usize = part_2(aoc_reader);
    println!("{}", result);
}
