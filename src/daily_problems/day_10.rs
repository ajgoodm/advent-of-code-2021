pub mod solutions {
    use std::collections::{HashMap, HashSet};

    use lazy_static::lazy_static;

    use crate::AocBufReader;

    lazy_static! {
        static ref OPENING_CHARS: HashSet<char> = vec!['(', '[', '{', '<'].into_iter().collect();
        static ref CLOSING_CHARS: HashSet<char> = vec![')', ']', '}', '>'].into_iter().collect();
        static ref OPEN_TO_CLOSE: HashMap<char, char> = vec![('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')].into_iter().collect();
        static ref CLOSE_TO_OPEN: HashMap<char, char> = vec![(')', '('), (']', '['), ('}', '{'), ('>', '<')].into_iter().collect();
        static ref CHAR_TO_SCORE_PT1: HashMap<char, usize> = vec![(')', 3), (']', 57), ('}', 1197), ('>', 25137)].into_iter().collect();
        static ref CHAR_TO_SCORE_PT2: HashMap<char, usize> = vec![(')', 1), (']', 2), ('}', 3), ('>', 4)].into_iter().collect();        
    }


    struct CharLine {
        line: String,
        opening_char_queue: Vec<char>,
    }

    
    impl CharLine {
        fn new(line: String) -> CharLine {
            CharLine {line: line, opening_char_queue: vec![]}
        }

        fn next_expected_closing_char(&self) -> Option<char> {
            if let Some(opening_char) = self.opening_char_queue.last() {
                return Some(*OPEN_TO_CLOSE.get(opening_char).unwrap())
            }
            None
        }

        fn validate(mut self) -> Option<char> {
            for next_char in self.line.chars() {
                if OPENING_CHARS.contains(&next_char) {
                    self.opening_char_queue.push(next_char);
                } else {
                    if let Some(next_expected_closing_char) = self.next_expected_closing_char() {
                        if next_char == next_expected_closing_char  {
                            self.opening_char_queue.pop().unwrap();
                            continue
                        }
                    }
                    return Some(next_char)
                }
            }
            None
        }

        fn complete(mut self) -> String {
            for next_char in self.line.chars() {
                if OPENING_CHARS.contains(&next_char) {
                    self.opening_char_queue.push(next_char);
                } else {
                    if let Some(next_expected_closing_char) = self.next_expected_closing_char() {
                        if next_char == next_expected_closing_char  {
                            self.opening_char_queue.pop().unwrap();
                            continue
                        }
                    }
                    return String::new()
                }
            }
            self.opening_char_queue.iter().rev().map(|opening_char| {
                OPEN_TO_CLOSE.get(opening_char).unwrap()
            }).collect::<String>()
        }
    }


    pub fn part_1(aoc_reader: AocBufReader) -> usize {
        aoc_reader.into_iter().map(|line| {
            match CharLine::new(line).validate() {
                Some(invalid_char) => *CHAR_TO_SCORE_PT1.get(&invalid_char).unwrap(),
                None => 0,
            }
        }).sum()
    }


    fn score_string(s: String) -> usize {
        let mut score: usize = 0;
        for c in s.chars() {
            score = score * 5 + CHAR_TO_SCORE_PT2.get(&c).unwrap()
        }
        score
    }


    pub fn part_2(aoc_reader: AocBufReader) -> usize {
        let mut scores: Vec<usize> = aoc_reader.into_iter().map(|line| {
            score_string(CharLine::new(line).complete())
        }).filter(|score| *score != 0).collect();
        scores.sort();
        scores[scores.len() / 2]
    }
}