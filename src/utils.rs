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