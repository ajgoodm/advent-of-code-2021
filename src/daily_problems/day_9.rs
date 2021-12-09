pub mod solutions {
    use std::collections::HashSet;

    use crate::AocBufReader;


    struct SeaFloorMap {
        n_rows: usize,
        n_cols: usize,
        depths: Vec<Vec<usize>>
    }


    impl SeaFloorMap {
        fn from_aoc_reader(aoc_reader: AocBufReader) -> SeaFloorMap {
            let mut depths: Vec<Vec<usize>> = aoc_reader.map(|row| {
                row.chars().map(|c| c.to_digit(10u32).unwrap() as usize).collect::<Vec<usize>>()
            }).collect();

            // all rows are the same length
            let row_lengths: HashSet<usize> = depths.iter().map(|row| row.len()).collect();
            assert_eq!(row_lengths.len(), 1);

            SeaFloorMap {
                n_rows: depths.len(),
                n_cols: depths[0].len(),
                depths: depths,
            }
        }

        fn iter_depths(&self) -> impl Iterator<Item = ((usize, usize), &usize)> {
            self.depths.iter().enumerate()
                .flat_map(|(row_idx, row)| {
                    row.iter().enumerate().map(move |(col_idx, depth)| ((row_idx, col_idx), depth))
                })
        }

        fn get_depth(&self, row_idx: usize, col_idx: usize) -> Option<&usize> {
            if row_idx >= self.n_rows || col_idx >= self.n_cols {
                return None
            }
            Some(&self.depths[row_idx][col_idx])
        }

        fn local_minima(&self) -> Vec<((usize, usize), usize)> {
            let mut minima: Vec<((usize, usize), usize)> = vec![];
            for ((row_idx, col_idx), depth) in self.iter_depths() {
                if col_idx > 0 {
                    if let Some(north_neighbor) = self.get_depth(row_idx, col_idx - 1) {
                       if north_neighbor <= depth {continue}
                }
                }
                if let Some(south_neighbor) = self.get_depth(row_idx, col_idx + 1) {
                    if south_neighbor <= depth {continue}
                }
                if row_idx > 0 {
                    if let Some(west_neighbor) = self.get_depth(row_idx - 1, col_idx) {
                        if west_neighbor <= depth {continue}
                }
                }
                if let Some(east_neighbor) = self.get_depth(row_idx + 1, col_idx) {
                    if east_neighbor <= depth {continue}
                }
                minima.push(((row_idx, col_idx), *depth));
            }

            minima
        }
    }


    pub fn part_1(aoc_reader: AocBufReader) -> usize {
        let sea_floor_map = SeaFloorMap::from_aoc_reader(aoc_reader);
        sea_floor_map.local_minima().iter().map(|((_, _), depth)| depth + 1).sum()
    }
}