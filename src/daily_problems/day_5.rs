pub mod solutions {
    use std::cmp::{min, max};
    use std::collections::HashMap;

    use itertools::Zip;
    use lazy_static::lazy_static;
    use regex::Regex;

    use crate::input::read_input::AocBufReader;

    struct LineSegment {
        p1: (usize, usize),
        p2: (usize, usize),
    }


    impl LineSegment {
        fn part_1_touched_points(&self) -> Vec<(usize, usize)> {
            let (p1x, p1y) = self.p1;
            let (p2x, p2y) = self.p2;

            if p1x == p2x {
                let min_y = min(p1y, p2y);
                let max_y = max(p1y, p2y);
                return (min_y..(max_y + 1)).map(
                    |y_idx| (p1x, y_idx)
                ).collect()
            } else if p1y == p2y {
                let min_x = min(p1x, p2x);
                let max_x = max(p1x, p2x);
                return (min_x..(max_x + 1)).map(
                    |x_idx| (x_idx, p1y)
                ).collect()
            } else {
                return vec![]
            }
        }

        fn part_2_touched_points(&self) -> Vec<(usize, usize)> {
            let (p1x, p1y) = self.p1;
            let (p2x, p2y) = self.p2;

            if p1x == p2x {
                let min_y = min(p1y, p2y);
                let max_y = max(p1y, p2y);
                return (min_y..(max_y + 1)).map(
                    |y_idx| (p1x, y_idx)
                ).collect()
            } else if p1y == p2y {
                let min_x = min(p1x, p2x);
                let max_x = max(p1x, p2x);
                return (min_x..(max_x + 1)).map(
                    |x_idx| (x_idx, p1y)
                ).collect()
            } else {
                let xrange: Vec<usize>;
                if p1x <= p2x {
                    xrange = (p1x..=p2x).collect();
                } else {
                    xrange = (p2x..=p1x).rev().collect();
                }

                let yrange: Vec<usize>;
                if p1y <= p2y {
                    yrange = (p1y..=p2y).collect();
                } else {
                    yrange = (p2y..=p1y).rev().collect();
                }

                return Zip::new((xrange, yrange)).collect()
            }
        }
    }


    fn read_input(aoc_reader: AocBufReader) -> Vec<LineSegment> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^([0-9]*),([0-9]*) -> ([0-9]*),([0-9]*)$").unwrap();
        }

        aoc_reader.map(|line|{
            let cap = RE.captures(&line).unwrap();
            LineSegment {
                p1: (
                    cap.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                    cap.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                ),
                p2: (
                    cap.get(3).unwrap().as_str().parse::<usize>().unwrap(),
                    cap.get(4).unwrap().as_str().parse::<usize>().unwrap(),
                ),
            }
        }).collect::<Vec<LineSegment>>()
    }


    pub fn part_1(aoc_reader: AocBufReader) -> usize {
        let line_segments: Vec<LineSegment> = read_input(aoc_reader);
        let mut touched_points: HashMap<(usize, usize), usize> = HashMap::new();
        for line in line_segments {
            for point in line.part_1_touched_points() {
                *touched_points.entry(point).or_insert(0) += 1;
            }
        }

        touched_points.values().map(|x| {
            match *x > 1 {
                true => 1,
                false => 0
            }
        }).sum()
    }

    pub fn part_2(aoc_reader: AocBufReader) -> usize {
        let line_segments: Vec<LineSegment> = read_input(aoc_reader);
        let mut touched_points: HashMap<(usize, usize), usize> = HashMap::new();
        for line in line_segments {
            for point in line.part_2_touched_points() {
                *touched_points.entry(point).or_insert(0) += 1;
            }
        }

        touched_points.values().map(|x| {
            match *x > 1 {
                true => 1,
                false => 0
            }
        }).sum()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_part_2_touched_points() {
            assert_eq!((LineSegment {p1: (0, 0), p2: (1, 1)}).part_2_touched_points(), vec![(0, 0), (1, 1)]);
            assert_eq!((LineSegment {p1: (1, 1), p2: (0, 0)}).part_2_touched_points(), vec![(1, 1), (0, 0)]);
            assert_eq!((LineSegment {p1: (0, 0), p2: (0, 0)}).part_2_touched_points(), vec![(0, 0)]);
            assert_eq!((LineSegment {p1: (0, 0), p2: (2, 0)}).part_2_touched_points(), vec![(0, 0), (1, 0), (2, 0)]);
        }
    }
}