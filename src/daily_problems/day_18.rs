pub mod solutions {
    use lazy_static::lazy_static;
    use regex::Regex;

    use crate::AocBufReader;

    lazy_static! {
        static ref ANY_INTERNAL_PAIR: Regex = Regex::new(
            r"^(.*)\[([0-9]*),([0-9]*)\](.*)$"
        ).unwrap();

        static ref LEFT_MOST_INTERNAL_PAIR: Regex = Regex::new(
            r"^\[([0-9]*),([0-9]*)\](.*)$"
        ).unwrap();

        static ref RIGHT_MOST_NUM: Regex = Regex::new(
            r"^(.*[^0-9])([0-9]+)([^0-9]*)$"
        ).unwrap();

        static ref LEFT_MOST_NUM: Regex = Regex::new(
            r"^([^0-9]*)([0-9]+)([^0-9].*)$"
        ).unwrap();

        static ref VAL_GREATER_THAN_TEN:Regex = Regex::new(
            r"^(.*[^0-9])([0-9]+[0-9]+)([^0-9].*)$"
        ).unwrap();

        static ref LEFT_MOST_GREATER_THAN_TEN :Regex = Regex::new(
            r"^([0-9]+[0-9]+)([,\]].*)$"
        ).unwrap();
    }


    #[derive(PartialEq, Eq, Debug)]
    struct SnailFishNumber {
        s: String
    }


    impl SnailFishNumber {
        fn new(s: String) -> SnailFishNumber {
            SnailFishNumber { s }
        }

        fn _get_deep_pair(s: &str) -> Option<Vec<String>> {
            let mut depth: usize = 0;
            for (idx, c) in s.chars().enumerate() {
                if depth >= 4 {
                    if let Some(capture) = LEFT_MOST_INTERNAL_PAIR.captures(&s[idx..]) {
                        return Some(vec![
                            s[..idx].to_string(),
                            capture.get(1).unwrap().as_str().to_string(),
                            capture.get(2).unwrap().as_str().to_string(),
                            capture.get(3).unwrap().as_str().to_string()
                        ])
                    }
                }
                if c == '[' { depth += 1; }
                if c == ']' { depth -= 1; }
            }
            return None
        }

        fn _maybe_add_at_right(s: &str, addendum: usize) -> String {
            if let Some(capture) = LEFT_MOST_NUM.captures(s) {
                let left_sub_str = capture.get(1).unwrap().as_str();
                let val = capture.get(2).unwrap().as_str().parse::<usize>().unwrap();
                let right_sub_str = capture.get(3).unwrap().as_str();
                return vec![left_sub_str, &(val + addendum).to_string(), right_sub_str].into_iter().collect::<String>()
            }
            s.to_string()
        }

        fn _maybe_add_left(s: &str, addendum: usize) -> String {
            if let Some(capture) = RIGHT_MOST_NUM.captures(s) {
                let left_sub_str = capture.get(1).unwrap().as_str();
                let val = capture.get(2).unwrap().as_str().parse::<usize>().unwrap();
                let right_sub_str = capture.get(3).unwrap().as_str();
                return vec![left_sub_str, &(val + addendum).to_string(), right_sub_str].into_iter().collect::<String>()
            }
            s.to_string()
        }

        fn explode_deeper_than_4(&mut self) {
            if let Some(str_vec) = SnailFishNumber::_get_deep_pair(&self.s) {
                let deep_left = str_vec[1].parse::<usize>().unwrap();
                let deep_right = str_vec[2].parse::<usize>().unwrap();

                self.s = vec![
                    SnailFishNumber::_maybe_add_left(&str_vec[0], deep_left),
                    "0".to_string(),
                    SnailFishNumber::_maybe_add_at_right(&str_vec[3], deep_right)
                ].into_iter().collect::<String>();
            }
        }

        fn split_greater_than_9(&mut self) {
            let mut left_sub_str: &str = "";
            let mut val: usize = 0;
            let mut right_sub_str: &str = "";

            for (idx, c) in self.s.chars().enumerate() {
                if let Some(capture) = LEFT_MOST_GREATER_THAN_TEN.captures(&self.s[idx..]) {
                    left_sub_str = &self.s[..idx];
                    val = capture.get(1).unwrap().as_str().parse::<usize>().unwrap();
                    right_sub_str = capture.get(2).unwrap().as_str();
                    break
                }
            }

            self.s = vec![
                left_sub_str.to_string(),
                "[".to_string(),
                (val / 2).to_string(),
                ",".to_string(),
                (val - (val / 2)).to_string(),
                "]".to_string(),
                right_sub_str.to_string()
            ].into_iter().collect::<String>();
        }

        fn reduce(&mut self) {
            loop {
                if let Some(_) = SnailFishNumber::_get_deep_pair(&self.s) {
                    self.explode_deeper_than_4();
                } else if let Some(_) = VAL_GREATER_THAN_TEN.captures(&self.s) {
                    self.split_greater_than_9();
                } else {
                    break
                }
            }
        }

        fn add(&mut self, other: SnailFishNumber) {
            let new_str = vec![
                "[".to_string(),
                self.s.clone(),
                ",".to_string(),
                other.s,
                "]".to_string()
            ].into_iter().collect::<String>();

            self.s = new_str;
            self.reduce()
        }

        fn magnitude(&self) -> usize {
            let mut result: String = self.s.clone();
            while let Some(capture) = ANY_INTERNAL_PAIR.captures(&result) {
                let left_sub_str = capture.get(1).unwrap().as_str();
                let left_val = capture.get(2).unwrap().as_str().parse::<usize>().unwrap();
                let right_val = capture.get(3).unwrap().as_str().parse::<usize>().unwrap();
                let right_sub_str = capture.get(4).unwrap().as_str();
                result = vec![
                    left_sub_str.to_string(),
                    (3 * left_val + 2 * right_val).to_string(),
                    right_sub_str.to_string()
                ].into_iter().collect()

            }
            result.parse::<usize>().unwrap()
        }
    }


    pub fn part_1(mut aoc_reader: AocBufReader) -> usize {
       let mut sum = SnailFishNumber::new(aoc_reader.next().unwrap());
       while let Some(line) = aoc_reader.next() {
           sum.add(SnailFishNumber::new(line));
       }

        sum.magnitude()
    }



    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_explode_deeper_than_4() {
            let mut s = SnailFishNumber::new("[7,[6,[5,[4,[3,2]]]]]".to_string());
            s.explode_deeper_than_4();
            assert_eq!(s.s, "[7,[6,[5,[7,0]]]]".to_string());
        }

        #[test]
        fn test_split_greater_than_9() {
            let mut s = SnailFishNumber::new("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]".to_string());
            s.split_greater_than_9();
            assert_eq!(s.s, "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]".to_string());
        }

        #[test]
        fn test_reduce() {
            let mut s = SnailFishNumber::new("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]".to_string());
            s.reduce();
            assert_eq!(s.s, "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".to_string())
        }

        #[test]
        fn test_magnitude() {
            let s = SnailFishNumber::new("[[1,2],[[3,4],5]]".to_string());
            assert_eq!(s.magnitude(), 143);
        }
    }
}