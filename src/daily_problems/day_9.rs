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

        fn get_adjacent(&self, row_idx: usize, col_idx: usize) -> Vec<((usize, usize), &usize)> {
            let mut adjacent_pts: Vec<((usize, usize), &usize)> = vec![];
            if col_idx > 0 {
                if let Some(north_neighbor) = self.get_depth(row_idx, col_idx - 1) {
                    adjacent_pts.push(((row_idx, col_idx - 1), north_neighbor));
                }
            }
            if let Some(south_neighbor) = self.get_depth(row_idx, col_idx + 1) {
                adjacent_pts.push(((row_idx, col_idx + 1), south_neighbor));
            }
            if row_idx > 0 {
                if let Some(west_neighbor) = self.get_depth(row_idx - 1, col_idx) {
                    adjacent_pts.push(((row_idx - 1, col_idx), west_neighbor));
                }
            }
            if let Some(east_neighbor) = self.get_depth(row_idx + 1, col_idx) {
                adjacent_pts.push(((row_idx + 1, col_idx), east_neighbor));
            }

            adjacent_pts
        }

        fn local_minima(&self) -> Vec<((usize, usize), usize)> {
            let mut minima: Vec<((usize, usize), usize)> = vec![];
            let mut is_minimum: bool;
            for ((row_idx, col_idx), depth) in self.iter_depths() {
                is_minimum = true;
                for (_, adjacent_depth) in self.get_adjacent(row_idx, col_idx) {
                    if adjacent_depth <= depth {
                        is_minimum = false;
                    }
                }
                if is_minimum {
                    minima.push(((row_idx, col_idx), *depth));
                }
            }

            minima
        }

        fn basin_size(&self, mut row_idx: usize, mut col_idx: usize) -> usize {
            let mut locations_to_explore: Vec<(usize, usize)> = vec![(row_idx, col_idx)];
            let mut explored_locations: HashSet<(usize, usize)> = vec![(row_idx, col_idx)].into_iter().collect();
            let mut locations_in_basin: HashSet<(usize, usize)> = HashSet::new();

            while locations_to_explore.len() > 0 {
                let (row_idx, col_idx) = locations_to_explore.pop().unwrap();
                locations_in_basin.insert((row_idx, col_idx));

                let current_depth = self.get_depth(row_idx, col_idx).unwrap();
                for ((next_row_idx, next_col_idx), next_depth) in self.get_adjacent(row_idx, col_idx) {
                    explored_locations.insert((next_row_idx, next_col_idx));
                    if *next_depth != 9usize && next_depth > current_depth {
                        locations_to_explore.push((next_row_idx, next_col_idx));
                    }
                }
            }

            locations_in_basin.len()
        }
    }


    pub fn part_1(aoc_reader: AocBufReader) -> usize {
        let sea_floor_map = SeaFloorMap::from_aoc_reader(aoc_reader);
        sea_floor_map.local_minima().iter().map(|((_, _), depth)| depth + 1).sum()
    }


    pub fn part_2(aoc_reader: AocBufReader) -> usize {
        let sea_floor_map = SeaFloorMap::from_aoc_reader(aoc_reader);
        let mut basin_sizes: Vec<usize> = sea_floor_map.local_minima().iter()
            .map(|((row_idx, col_idx), _)| sea_floor_map.basin_size(*row_idx, *col_idx))
            .collect();
        basin_sizes.sort_by(|a, b| b.cmp(a));
        basin_sizes[0] * basin_sizes[1] * basin_sizes[2]
    }
}