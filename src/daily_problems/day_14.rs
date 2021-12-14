pub mod solutions {
    use crate::{AocBufReader, input::read_input};


    struct Polymer {
        template: String,
    }


    struct InsertionRule {
        pair: String,
        new_element: char,
    }


    fn read_input(mut aoc_reader: AocBufReader) -> (Polymer, Vec<InsertionRule>) {
        let polymer = Polymer {
            template: aoc_reader.next().unwrap()
        };

        aoc_reader.next().unwrap(); // blank line
        let mut insertion_rules: Vec<InsertionRule> = vec![];
        while let Some(line) = aoc_reader.next() {
            let inputs: Vec<&str> = line.split(" -> ").collect();
            let pair = inputs[0];
            assert_eq!(pair.len(), 2);

            let new_element = inputs[1];
            assert_eq!(new_element.len(), 1);

            insertion_rules.push(
                InsertionRule {
                    pair: pair.to_string(),
                    new_element: new_element.as_bytes()[0] as char
                }
            )
        }
        (polymer, insertion_rules)
    }


    pub fn part_1(aoc_reader: AocBufReader) -> usize {
        read_input(aoc_reader);
        1
    }
}