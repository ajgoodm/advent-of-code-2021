pub mod solutions {
    use std::collections::HashSet;

    use crate::AocBufReader;

    #[derive(Clone, Copy, Hash, PartialEq, Eq)]
    struct CavernLocation {
        row_idx: usize,
        col_idx: usize
    }


    impl CavernLocation {
        fn new(row_idx: usize, col_idx: usize) -> CavernLocation {
            CavernLocation { row_idx: row_idx, col_idx: col_idx }
        }
    }

    struct CavernMap {
        risk_levels: Vec<Vec<usize>>,
        visited: Vec<Vec<bool>>,
        cost: Vec<Vec<usize>>,
    }


    impl CavernMap {
        fn new(risk_levels: Vec<Vec<usize>>) -> CavernMap {
            let n_rows = risk_levels.len();
            let n_cols = risk_levels[0].len();

            let visted = vec![vec![false; n_cols]; n_rows];
            let cost = vec![vec![usize::MAX; n_cols]; n_rows];

            CavernMap {
                risk_levels: risk_levels,
                visited: visted,
                cost: cost            
            }
        }

        fn from_reader_pt1(aoc_reader: AocBufReader) -> CavernMap {
            let mut risk_levels: Vec<Vec<usize>> = vec![];
            for line in aoc_reader {
                risk_levels.push(
                    line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect::<Vec<usize>>()
                );
            }
            CavernMap::new(risk_levels)
        }

        /// Takes the sum and rolls it over if it is more than 9.
        /// This is very close to (but subtly different than) the modulo operator.
        fn increase_risk_level(val: usize, increase: usize) -> usize {
            let mut new_val = val + increase;
            while new_val > 9 {
                new_val = new_val - 9
            }
            new_val
        }

        fn from_reader_pt2(aoc_reader: AocBufReader) -> CavernMap {
            let mut risk_levels: Vec<Vec<usize>> = vec![];
            for line in aoc_reader {
                let mut risk_level_line: Vec<usize> = vec![];
                for repeat in 0..5 {
                    risk_level_line.extend(
                        line.chars().map(|c| {
                            CavernMap::increase_risk_level(
                                c.to_digit(10).unwrap() as usize,
                                repeat
                            )
                        })
                    )
                }
                risk_levels.push(risk_level_line);
            }

            let mut new_lines: Vec<Vec<usize>> = vec![];
            for repeat in 1..5 {
                for line in risk_levels.iter() {
                    new_lines.push(
                        line.iter().map(|x| CavernMap::increase_risk_level(*x, repeat)).collect()
                    )
                }
            }
            risk_levels.extend(new_lines);

            CavernMap::new(risk_levels)
        }

        /// Returns the maximum row_idx, col_idx pair as a CavernLocation
        /// (the bounds of the map)
        fn south_east_location(&self) -> CavernLocation {
            CavernLocation::new(
                self.risk_levels.len() - 1, self.risk_levels[0].len() - 1
            )
        }

        fn get_neighbors(&self, location: &CavernLocation) -> Vec<CavernLocation> {
            let mut neighbors: Vec<CavernLocation> = vec![];
            if location.row_idx > 0 {neighbors.push(CavernLocation::new(location.row_idx - 1, location.col_idx))}
            if location.row_idx < self.south_east_location().row_idx {neighbors.push(CavernLocation::new(location.row_idx + 1, location.col_idx))}
            if location.col_idx > 0 {neighbors.push(CavernLocation::new(location.row_idx, location.col_idx - 1))}
            if location.col_idx < self.south_east_location().col_idx {neighbors.push(CavernLocation::new(location.row_idx, location.col_idx + 1))}

            neighbors
        }

        fn get_visited(&self, location: &CavernLocation) -> bool {
            self.visited[location.row_idx][location.col_idx]
        }

        fn set_visited(&mut self, location: &CavernLocation) {
            self.visited[location.row_idx][location.col_idx] = true;
        }

        fn get_cost(&self, location: &CavernLocation) -> usize {
            self.cost[location.row_idx][location.col_idx]
        }

        fn set_cost(&mut self, location: &CavernLocation, new_cost: usize) {
            if !(new_cost > self.get_cost(location)) {
                self.cost[location.row_idx][location.col_idx] = new_cost;
            }
        }

        fn get_risk_level(&self, location: &CavernLocation) -> usize {
            self.risk_levels[location.row_idx][location.col_idx]
        }

        /// Returns the next candidate unvisisted node with the lowest cost
        /// and removes it from the queue.
        fn get_current_location_from_neighbors(&self, unvisited_nodes: &mut HashSet<CavernLocation>) -> CavernLocation {
            let mut next_location: Option<CavernLocation> = None;
            let mut lowest_cost: Option<usize> = None;
            for neighbor in unvisited_nodes.iter() {
                if let Some(current_lowest_cost) = lowest_cost {
                    let candidate_cost = self.get_cost(neighbor);
                    if candidate_cost < current_lowest_cost { 
                        lowest_cost = Some(candidate_cost);
                        next_location = Some(*neighbor);
                    }
                } else {
                    lowest_cost = Some(self.get_cost(neighbor));
                    next_location = Some(*neighbor);
                }
            }

            unvisited_nodes.remove(&next_location.unwrap());
            next_location.unwrap()
        }

        fn least_risky_path(mut self, start: CavernLocation, end: CavernLocation) -> usize {
            self.set_visited(&start);
            self.set_cost(&start, 0); // we never enter the start

            let mut current_location: CavernLocation;
            let mut unvisited_nodes: HashSet<CavernLocation> = vec![start].into_iter().collect();

            while unvisited_nodes.len() > 0 {
                current_location = self.get_current_location_from_neighbors(&mut unvisited_nodes);

                if current_location == end { return self.get_cost(&current_location) }
                let current_cost = self.get_cost(&current_location);

                let neighbors: Vec<CavernLocation> = self.get_neighbors(&current_location)
                    .into_iter()
                    .filter(|location| !self.get_visited(location))
                    .collect();

                for neighbor in neighbors.iter() {
                    let neighbor_risk_level = self.get_risk_level(neighbor);
                    self.set_cost(neighbor, neighbor_risk_level + current_cost);
                }

                self.set_visited(&current_location);
                unvisited_nodes.extend(neighbors);
            }

            panic!("oh no!");
        }
    }



    pub fn part_1(aoc_reader: AocBufReader) -> usize {
        let cavern_map = CavernMap::from_reader_pt1(aoc_reader);
        let start = CavernLocation::new(0, 0);
        let end = cavern_map.south_east_location();
        cavern_map.least_risky_path(start, end)
    }


    pub fn part_2(aoc_reader: AocBufReader) -> usize {
        let cavern_map = CavernMap::from_reader_pt2(aoc_reader);
        let start = CavernLocation::new(0, 0);
        let end = cavern_map.south_east_location();
        cavern_map.least_risky_path(start, end)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_increase_risk_level() {
            assert_eq!(CavernMap::increase_risk_level(1, 0), 1);
            assert_eq!(CavernMap::increase_risk_level(9, 0), 9);
            assert_eq!(CavernMap::increase_risk_level(9, 1), 1);
        }
    }

}