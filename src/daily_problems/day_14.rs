pub mod solutions {
    use std::{collections::HashMap, hash::Hash};

    use itertools::Itertools;

    use crate::{AocBufReader, input::read_input};



    struct QueuedInsertion {
        idx: usize,
        char: char
    }

    pub struct Polymer {
        pub template: String,
        pub insertions: HashMap<String, char>
    }


    impl Polymer {
        pub fn len(&self) -> usize {
            self.template.len()
        }

        pub fn insert_char_at(&mut self, idx: usize, c: char) {
            if idx > self.len() { panic!("Cannot insert char there!") }
            self.template = vec![&self.template[..idx], &c.to_string(), &self.template[idx..]].iter().join("")
        }

        pub fn run_insertions(&mut self) {
            let mut insertion_queue: Vec<QueuedInsertion> = vec![];
            for idx in 0usize..self.template.len() - 1 {
                match self.insertions.get(&self.template[idx..idx + 2]) {
                    Some(c) => insertion_queue.push(
                        QueuedInsertion {
                            idx: idx + 1 + insertion_queue.len(),
                            char: *c
                        }
                    ),
                    None => ()
                }
            }

            for queued_insertion in insertion_queue.iter() {
                self.insert_char_at(queued_insertion.idx, queued_insertion.char)
            }
        }


        pub fn score_part_1(&self) -> usize {
            let mut occurence_count: HashMap<char, usize> = HashMap::new();
            for c in self.template.chars() {
                *occurence_count.entry(c).or_insert(0) += 1
            }

            let mut max_char_val: usize = usize::MIN;
            let mut min_char_val: usize = usize::MAX;

            for (c, cts) in occurence_count {
                if cts > max_char_val {max_char_val = cts}
                if cts < min_char_val {min_char_val = cts}
            }
            max_char_val - min_char_val
        }
    }


    fn read_input(mut aoc_reader: AocBufReader) -> Polymer {
        let mut polymer = Polymer {
            template: aoc_reader.next().unwrap(),
            insertions: HashMap::new()
        };

        aoc_reader.next().unwrap(); // blank line
        let mut insertion_rules: HashMap<String, char> = HashMap::new();
        while let Some(line) = aoc_reader.next() {
            let inputs: Vec<&str> = line.split(" -> ").collect();
            let pair = inputs[0];
            assert_eq!(pair.len(), 2);

            let new_element = inputs[1];
            assert_eq!(new_element.len(), 1);

            insertion_rules.insert(pair.to_string(), new_element.as_bytes()[0] as char);
        }
        polymer.insertions = insertion_rules;
        polymer
    }


    pub fn part_1(aoc_reader: AocBufReader) -> usize {
        let mut polymer = read_input(aoc_reader);
        for _ in 0..10 {
            polymer.run_insertions();
        }
        polymer.score_part_1()
    }
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::solutions::*;

    #[test]
    fn test_insert_char_at() {
        let mut test_seq: Polymer = Polymer {template: "ABC".to_string(), insertions: HashMap::new()};
        test_seq.insert_char_at(0, 'D');
        assert_eq!(test_seq.template, "DABC".to_string());
        test_seq.insert_char_at(1, 'E');
        assert_eq!(test_seq.template, "DEABC".to_string());
        test_seq.insert_char_at(test_seq.len(), 'F');
        assert_eq!(test_seq.template, "DEABCF".to_string());
    }

    #[test]
    fn test_run_insertions() {
        let mut test_seq: Polymer = Polymer {
            template: "NNCB".to_string(),
            insertions: vec![
                ("NN".to_string(), 'C'),
                ("NC".to_string(), 'B'),
                ("CB".to_string(), 'H')
            ].into_iter().collect()
        };
        test_seq.run_insertions();
        assert_eq!(test_seq.template, "NCNBCHB".to_string());
    }
}