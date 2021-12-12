pub mod solutions {
    use std::collections::HashSet;

    use itertools::Itertools;

    use crate::AocBufReader;


    struct Octopi {
        grid: Vec<Vec<usize>>,
        n_flashes: usize,
        // Did all of the octopi flash this round?
        nsync: bool // Bye, Bye, Bye; bonus for dereferencing this val *nsync
    }


    impl Octopi {
        fn from_reader(aoc_reader: AocBufReader) -> Octopi {
            let mut grid: Vec<Vec<usize>> = Vec::new();
            for row in aoc_reader {
                grid.push(row.chars().map(|c| c.to_digit(10).unwrap() as usize).collect());
            }

            Octopi {grid: grid, n_flashes: 0, nsync: false}
        }

        fn get_adjacent(&mut self, row_idx: usize,  col_idx: usize) -> Vec<(usize, usize)> {
            (-1isize..=1).cartesian_product(-1isize..=1).filter(|(drow, dcol)| {
                !(*drow == 0 && *dcol ==0)
            }).map(
                |(drow, dcol)| (row_idx as isize + drow, col_idx as isize + dcol)
            ).filter(
                |(new_row, new_col)| {
                    (*new_row >= 0 && *new_row < 10) && (*new_col >= 0 && *new_col < 10)
                }
            ).map(|(new_row, new_col)| (new_row as usize, new_col as usize)).collect()
        }

        fn step(&mut self) {
            self.nsync = false;

            let mut octopi_to_step: Vec<(usize, usize)> = vec![];
            let mut flashed: HashSet<(usize, usize)> = HashSet::new();
            let mut val: usize;

            for row_idx in 0..self.grid.len() {
                for col_idx in 0..self.grid.len() {
                    octopi_to_step.push((row_idx, col_idx));
                    while octopi_to_step.len() > 0 {
                        let curr_coord = octopi_to_step.pop().unwrap();
                        if flashed.contains(&curr_coord) {continue}
                        match curr_coord {  // Rust doesn't support destructuring tuples...?
                            (cur_row, cur_col) => {
                                val = self.grid[cur_row][cur_col];
                                if val == 9 {
                                    self.grid[cur_row][cur_col] = 0;
                                    self.n_flashes += 1;
                                    flashed.insert((cur_row, cur_col));
                                    octopi_to_step.extend(
                                        self.get_adjacent(cur_row, cur_col).iter().filter(|coord| {!flashed.contains(coord)})
                                    );
                                } else {
                                    self.grid[cur_row][cur_col] = val + 1;
                                }
                            }
                        }
                    }                
                }
            }
            if flashed.len() == 100 {self.nsync = true;}  // It's Gonna Be Me
        }
    }


    pub fn part_1(aoc_reader: AocBufReader) -> usize {
        let mut octopi = Octopi::from_reader(aoc_reader);
        for _ in 0..100 {
            octopi.step()
        }

        octopi.n_flashes
    }


    pub fn part_2(aoc_reader: AocBufReader) -> usize {
        let mut octopi = Octopi::from_reader(aoc_reader);
        let mut step_count: usize = 0;
        while !octopi.nsync {
            octopi.step();
            step_count += 1;
        }

        step_count
    }
}