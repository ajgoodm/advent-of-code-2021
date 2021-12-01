pub mod day_1 {
    use itertools::Zip;
    use crate::input::read_input::AocBufReader;


    fn read_input(aoc_reader: AocBufReader) -> Vec<usize> {
        aoc_reader.map(
            |line| {line.parse::<usize>().unwrap()}
        ).collect::<Vec<usize>>()
    }


    pub fn part_1(aoc_reader: AocBufReader) -> usize {
        let inputs = read_input(aoc_reader);
        Zip::new((&inputs, &inputs[1..])).map(
            |(element, next_element)| {
                match next_element > element {
                    true => 1,
                    false => 0
                }
            }).sum()
    }


    pub fn part_2(aoc_reader: AocBufReader) -> usize {
        let inputs = read_input(aoc_reader);
        Zip::new((&inputs, &inputs[1..], &inputs[2..], &inputs[3..])).map(
            |(first, second, third, fourth)| {
                match (second + third + fourth) > (first + second + third) {
                    true => 1,
                    false => 0
                }
            }).sum()
    }

}