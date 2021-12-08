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


    struct WireMapping {
        str_to_val: HashMap<String, usize>,
        val_to_str: HashMap<usize, String>
    }


    impl WireMapping {
        fn new() -> WireMapping {
            WireMapping {
                str_to_val: HashMap::new(),
                val_to_str: HashMap::new()
            }
        }

        fn get_patterns_of_len(patterns: &[String], n: usize) -> Vec<&String> {
            patterns.iter().filter(|pattern| pattern.len() == n).collect()
        }


        fn get_pattern_of_len(patterns: &[String], n: usize) -> &String {
            let patterns = WireMapping::get_patterns_of_len(&patterns[..], n);
            assert_eq!(patterns.len(), 1);

            patterns[0]
        }


        fn add_entry(&mut self, str: &String, val: usize) {
            self.str_to_val.insert(str.to_owned(), val);
            self.val_to_str.insert(val, str.to_owned());
        }


        fn get_chars_for_num(&self, n: usize) -> Option<HashSet<char>> {
            match self.val_to_str.get(&n) {
                Some(str) => Some(str.chars().collect::<HashSet<char>>()),
                None => None
            }

        }


        fn build(&mut self, patterns: Vec<String>) {
            let one_str: &String = WireMapping::get_pattern_of_len(&patterns, 2);
            self.add_entry(one_str, 1usize);

            let seven_str: &String = WireMapping::get_pattern_of_len(&patterns, 3);
            self.add_entry(seven_str, 7usize);

            let four_str: &String = WireMapping::get_pattern_of_len(&patterns, 4);
            self.add_entry(four_str, 4usize);

            let eight_str: &String = WireMapping::get_pattern_of_len(&patterns, 7);
            self.add_entry(eight_str, 8usize);

            let strs_of_len_5: Vec<&String> = WireMapping::get_patterns_of_len(&patterns, 5);
            let strs_of_len_6: Vec<&String> = WireMapping::get_patterns_of_len(&patterns, 6);
            for str in strs_of_len_5.iter() {
                let str_chars: HashSet<char> = str.chars().collect();
                if self.get_chars_for_num(1usize).unwrap().is_subset(&str_chars) {
                    self.add_entry(str, 3usize);
                }
            }

            for str in strs_of_len_6.iter() {
                let str_chars: HashSet<char> = str.chars().collect();
                if !self.get_chars_for_num(1usize).unwrap().is_subset(&str_chars) {
                    self.add_entry(str, 6usize);
                }
            }

            for str in strs_of_len_5.iter() {
                let str_chars: HashSet<char> = str.chars().collect();
                if str_chars == self.get_chars_for_num(3).unwrap() {
                    continue
                } else if str_chars.is_subset(&self.get_chars_for_num(6usize).unwrap()) {
                    self.add_entry(str, 5usize);
                } else {
                    self.add_entry(str, 2usize);
                }
            }

            for str in strs_of_len_6.iter() {
                let str_chars: HashSet<char> = str.chars().collect();
                if str_chars == self.get_chars_for_num(6).unwrap() {
                    continue
                } else if self.get_chars_for_num(5).unwrap().is_subset(&str_chars) {
                    self.add_entry(str, 9usize);
                } else {
                    self.add_entry(str, 0usize);
                }
            }
        }

        fn get_num_for_digit(&self, digit: &String) -> usize {
            let digit_chars: HashSet<char> = digit.chars().collect();

            for val in 0..10 {
                if self.get_chars_for_num(val).unwrap() == digit_chars {
                    return val
                }
            }
            panic!("Unmapped digit {}", digit);
        }

        fn decode(&self, digits: Vec<String>) -> usize {
            digits.iter().rev().enumerate().map(|(place, digit)| {
                self.get_num_for_digit(digit) * 10usize.pow(place as u32)
            }).sum()
        }
    }


    pub fn part_2(aoc_reader: AocBufReader) -> usize {
        let mut sum: usize = 0;
        for line in aoc_reader {
            let input_output: Vec<&str> = line.split("|").collect();
            let inputs: Vec<String> = input_output[0].split_whitespace().map(|s| s.to_string()).collect();
            let digits: Vec<String> = input_output[1].split_whitespace().map(|s| s.to_string()).collect();

            let mut wire_map = WireMapping::new();
            wire_map.build(inputs);
            sum += wire_map.decode(digits);
        }

        sum
    }
}