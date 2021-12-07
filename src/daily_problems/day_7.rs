pub mod solutions {
    use std::cmp::min;

    use crate::AocBufReader;
    use crate::utils::stats::f64_avg_usize;


    fn required_fuel_pt1(positions: &Vec<usize>, candidate: usize) -> usize {
        positions.iter().map(|pos| {
            match *pos >= candidate {
                true => pos - candidate,
                false => candidate - pos
            }
        }).sum::<usize>()
    }


    fn required_fuel_pt2(positions: &Vec<usize>, candidate: usize) -> usize {
        positions.iter().map(|pos| {
            let diff: isize = *pos as isize - candidate as isize;
            if diff > 0 {
                let d = diff as usize;
                // formula for a triangular number
                (d * (d + 1)) / 2 as usize
            } else if diff < 0 {
                let d = (diff * -1) as usize;
                (d * (d + 1)) / 2 as usize
            } else {
                0
            }
        }).sum::<usize>()
    }


    pub fn part_1(mut aoc_reader: AocBufReader) -> usize {
        let positions: Vec<usize> = aoc_reader.next().unwrap()
            .split(",").map(|x| x.parse::<usize>().unwrap()).collect();

        let min_position = *positions.iter().min().unwrap();
        let max_position = *positions.iter().max().unwrap();

        let mut min_cost: Option<usize> = None;
        let mut cost: usize;
        for position in  min_position..=max_position {
            cost = required_fuel_pt1(&positions, position);
            match min_cost {
                Some(prior_min) => if cost <= prior_min {min_cost = Some(cost)},
                None => min_cost = Some(cost)
            }
        }

        min_cost.unwrap()
    }


    pub fn part_2(mut aoc_reader: AocBufReader) -> usize {
        let positions: Vec<usize> = aoc_reader.next().unwrap()
            .split(",").map(|x| x.parse::<usize>().unwrap()).collect();

        let min_position = *positions.iter().min().unwrap();
        let max_position = *positions.iter().max().unwrap();

        let mut min_cost: Option<usize> = None;
        let mut cost: usize;
        for position in  min_position..=max_position {
            cost = required_fuel_pt2(&positions, position);
            match min_cost {
                Some(prior_min) => if cost <= prior_min {min_cost = Some(cost)},
                None => min_cost = Some(cost)
            }
        }

        min_cost.unwrap()
    }
}