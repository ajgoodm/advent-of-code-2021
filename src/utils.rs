use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref LOWER_CASE_RE: Regex = Regex::new(r"^[a-z]*$").unwrap();
}


pub mod conversion {
    pub fn binary_string_to_usize(binary: String) -> usize {
        binary.chars().rev().enumerate().map(|(idx, c)| {
                match c {
                    '0' => 0,
                    '1' => 2usize.pow(idx as u32),
                    _ => panic!("unexpected char parsing binary")
                }
            }
        ).sum::<usize>()
    }
}


pub mod str_utils {
    use super::LOWER_CASE_RE;

    pub fn is_lower_case(string: &String) -> bool {
        match LOWER_CASE_RE.find(string) {
            Some(_) => true,
            None => false
        }
    }
}


pub mod stats {
    pub fn f64_avg_usize(vals: &Vec<usize>) -> f64 {
        vals.iter().sum::<usize>() as f64 / vals.len() as f64
    }
}


#[cfg(test)]
mod tests {
    use super::str_utils::*;

    #[test]
    fn test_is_lower() {
        assert_eq!(is_lower_case(&"A".to_string()), false);
        assert_eq!(is_lower_case(&"".to_string()), true);
        assert_eq!(is_lower_case(&"Aa".to_string()), false);
        assert_eq!(is_lower_case(&"abc".to_string()), true);
    }
}