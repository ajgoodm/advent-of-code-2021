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


pub mod stats {
    pub fn f64_avg_usize(vals: &Vec<usize>) -> f64 {
        vals.iter().sum::<usize>() as f64 / vals.len() as f64
    }
}