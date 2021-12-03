pub mod solutions {
    use std::collections::HashMap;

    use crate::input::read_input::AocBufReader;

    fn read_input(aoc_reader: AocBufReader) -> (String, String) {
        let mut n_lines: usize = 0;
        let mut one_bit_counts: HashMap<usize, usize> = HashMap::new();
        for line in aoc_reader {
            n_lines += 1;
            for (idx, c) in line.chars().enumerate() {
                match c {
                    '1' => *one_bit_counts.entry(idx).or_insert(0usize) += 1,
                    '0' => (),
                    _ => panic!("Unexpected char!")
                }
            }
        }

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
        let (gamma, epsilon): (String, String) = read_input(aoc_reader);
        let gamma = binary_string_to_usize(gamma);
        let epsilon = binary_string_to_usize(epsilon);
        gamma * epsilon
    }


}