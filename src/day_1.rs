pub mod day_1 {
    use itertools::Zip;
    use crate::input::read_input::AocBufReader;


    pub fn part_1(aoc_reader: AocBufReader) -> usize {
        let inputs: Vec<usize> = aoc_reader.map(
            |line| {line.parse::<usize>().unwrap()}
        ).collect();

        Zip::new((&inputs, &inputs[1..])).map(
            |(element, next_element)| {
                match next_element > element {
                    true => 1,
                    false => 0
                }
            }).sum()
    }
}