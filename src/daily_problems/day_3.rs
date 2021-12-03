pub mod solutions {
    use std::collections::HashMap;

    use crate::input::read_input::AocBufReader;

    fn _get_one_bit_counts(binary_strings: &Vec<String>) -> (HashMap<usize, usize>, usize) {
        let mut n_lines: usize = 0;
        let mut one_bit_counts: HashMap<usize, usize> = HashMap::new();
        for binary_string in binary_strings {
            n_lines += 1;
            for (idx, c) in binary_string.chars().enumerate() {
                match c {
                    '1' => *one_bit_counts.entry(idx).or_insert(0usize) += 1,
                    '0' => (),
                    _ => panic!("Unexpected char!")
                }
            }
        }

        (one_bit_counts, n_lines)
    }

    fn read_input_pt_1(aoc_reader: AocBufReader) -> (String, String) {
        let (one_bit_counts, n_lines) = _get_one_bit_counts(&aoc_reader.collect());

        let mut gamma: String = "".to_string();
        let mut epsilon: String = "".to_string();

        let mut idx: usize = 0;
        while let Some(one_bit_count) = one_bit_counts.get(&idx) {
            if one_bit_count * 2 > n_lines {
                gamma.push_str("1");
                epsilon.push_str("0");
            } else {
                gamma.push_str("0");
                epsilon.push_str("1");
            }
            idx += 1
        }

        (gamma, epsilon)
    }

    fn read_input_pt_2(aoc_reader: AocBufReader) -> (String, String) {
        let binary_strings: Vec<String> = aoc_reader.collect();
        let mut oxygen_strings: Vec<String> = binary_strings.iter().map(|x| x.to_string()).collect();
        let mut co2_strings: Vec<String> = binary_strings;

        let mut idx: usize = 0;
        while oxygen_strings.len() > 1 {
            let (one_bit_counts, n_strings) = _get_one_bit_counts(&oxygen_strings);
            if one_bit_counts.get(&idx).unwrap() * 2 >= n_strings {
                oxygen_strings = oxygen_strings.into_iter().filter(|binary_string| binary_string.as_bytes()[idx] as char == '1').collect::<Vec<String>>();
            } else {
                oxygen_strings = oxygen_strings.into_iter().filter(|binary_string| binary_string.as_bytes()[idx] as char == '0').collect::<Vec<String>>();
            }
            idx += 1
        }

        idx = 0;
        while co2_strings.len() > 1 {
            let (one_bit_counts, n_strings) = _get_one_bit_counts(&co2_strings);
            if one_bit_counts.get(&idx).unwrap() * 2 >= n_strings { // ones are more common, zeros are less or equal
                co2_strings = co2_strings.into_iter().filter(|binary_string| binary_string.as_bytes()[idx] as char == '0').collect::<Vec<String>>();
            } else {
                co2_strings = co2_strings.into_iter().filter(|binary_string| binary_string.as_bytes()[idx] as char == '1').collect::<Vec<String>>();
            }
            idx += 1;
        }
        (oxygen_strings[0].to_string(), co2_strings[0].to_string())
    }

    fn binary_string_to_usize(binary: String) -> usize {
        binary.chars().rev().enumerate().map(|(idx, c)| {
                match c {
                    '0' => 0,
                    '1' => 2usize.pow(idx as u32),
                    _ => panic!("unexpected char parsing binary")
                }
            }
        ).sum::<usize>()
    }

    pub fn part_1(aoc_reader: AocBufReader) -> usize {
        let (gamma, epsilon): (String, String) = read_input_pt_1(aoc_reader);
        let gamma = binary_string_to_usize(gamma);
        let epsilon = binary_string_to_usize(epsilon);
        gamma * epsilon
    }

    pub fn part_2(aoc_reader: AocBufReader) -> usize {
        let (oxygen_generator_rating, co2_scrubber_rating): (String, String) = read_input_pt_2(aoc_reader);
        println!("oxygen {}", oxygen_generator_rating);
        println!("co2 {}", co2_scrubber_rating);

        let oxygen_generator_rating = binary_string_to_usize(oxygen_generator_rating);
        let co2_scrubber_rating = binary_string_to_usize(co2_scrubber_rating);
        oxygen_generator_rating * co2_scrubber_rating
    }


}