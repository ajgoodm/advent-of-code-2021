pub mod solutions {
    use std::collections::{HashMap, HashSet};
    use lazy_static::lazy_static;

    use crate::AocBufReader;

    lazy_static! {
        static ref UNIQUE_LENS: HashSet<usize> = vec![2, 3, 4, 7].into_iter().collect();
    }

    pub fn part_1(aoc_reader: AocBufReader) -> usize {
        aoc_reader.map(|line| {
            line.split("|").collect::<Vec<&str>>()[1]
                .split(" ").map(|word| {
                    match UNIQUE_LENS.contains(&word.len()) {
                        true => 1,
                        false => 0
                    }
                }).sum::<usize>()
        }).sum::<usize>()
    }


    lazy_static! {
        static ref NUM_TO_WIRES: HashMap<usize, HashSet<char>> = vec![
            (0, vec!['a', 'b', 'c', 'e', 'f', 'g'].into_iter().collect()),
            (1, vec!['c', 'f'].into_iter().collect()),
            (2, vec!['a', 'c', 'd', 'e', 'g'].into_iter().collect()),
            (3, vec!['a', 'c', 'd', 'f', 'g'].into_iter().collect()),
            (4, vec!['b', 'c', 'd', 'f'].into_iter().collect()),
            (5, vec!['a', 'b', 'd', 'f', 'g'].into_iter().collect()),
            (6, vec!['a', 'b', 'd', 'e', 'f' ,'g'].into_iter().collect()),
            (7, vec!['a', 'c', 'f'].into_iter().collect()),
            (8, vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'].into_iter().collect()),
            (9, vec!['a', 'b', 'c', 'd', 'f', 'g'].into_iter().collect()),
        ].into_iter().collect();
    }


    struct WireMapping {
        mapping: HashMap<char, Option<char>>
    }


    impl WireMapping {
        fn new() -> WireMapping {
            WireMapping {
                mapping: vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'].into_iter().map(|c| (c, None)).collect()
            }
        }

        fn build(&mut self, patterns: Vec<&str>) {

        }

        fn decode(&self, digits: Vec<&str>) -> usize {
            1
        }

    }


    pub fn part_2(aoc_reader: AocBufReader) -> usize {
        let mut sum: usize = 0;
        for line in aoc_reader {
            let input_output: Vec<&str> = line.split("|").collect();
            let inputs: Vec<&str> = input_output[0].split(" ").collect();
            let digits: Vec<&str> = input_output[1].split(" ").collect();

            let mut wire_map = WireMapping::new();
            wire_map.build(inputs);
            sum += wire_map.decode(digits);
        }

        sum
    }
}