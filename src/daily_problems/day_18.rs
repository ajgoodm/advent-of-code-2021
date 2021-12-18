pub mod solutions {
    use lazy_static::lazy_static;
    use regex::Regex;

    use crate::AocBufReader;

    lazy_static! {
        static ref RIGHT_MOST_NUM: Regex = Regex::new(
            r"^(.*[^0-9])([0-9]+)([^0-9]*)$"
        ).unwrap();

        static ref LEFT_MOST_NUM: Regex = Regex::new(
            r"^([^0-9]*)([0-9]+)([^0-9].*)$"
        ).unwrap();

        static ref VAL_GREATER_THAN_TEN:Regex = Regex::new(
            r"^(.*[^0-9])([0-9]+[0-9]+)([^0-9].*)$"
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

        fn _get_deep_comma_idx(s: &str) -> Option<usize> {
            let mut depth: usize = 0;
            for (idx, c) in s.chars() {
                if c == '[' {
                    depth += 1;
                } else if c == ']' {
                    depth -= 1;
                } else 
            }
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
            if let Some(comma_idx) = _get_comma_idx(&self.s) {
                let left_sub_str = capture.get(1).unwrap().as_str();
                let deep_left: usize = capture.get(2).unwrap().as_str().parse::<usize>().unwrap();
                let deep_right: usize = capture.get(3).unwrap().as_str().parse::<usize>().unwrap();
                let right_sub_str = capture.get(4).unwrap().as_str();

                self.s = vec![
                    SnailFishNumber::_maybe_add_left(left_sub_str, deep_left),
                    "0".to_string(),
                    SnailFishNumber::_maybe_add_at_right(right_sub_str, deep_right)
                ].into_iter().collect::<String>();
            }
        }

        fn split_greater_than_9(&mut self) {
            let capture = VAL_GREATER_THAN_TEN.captures(&self.s).unwrap();
            let left_sub_str = capture.get(1).unwrap().as_str();
            let val = capture.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let right_sub_str = capture.get(3).unwrap().as_str();

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
                println!("\n\n***{}", self.s);
                if let Some(capture) = DEEP_PAIR_RE.captures(&self.s) {
                    self.explode_deeper_than_4();
                } else if let Some(capture) = VAL_GREATER_THAN_TEN.captures(&self.s) {
                    self.split_greater_than_9();
                } else {
                    break
                }
            }
        }
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
    }
}