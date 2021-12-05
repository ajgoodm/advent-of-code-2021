pub mod solutions {
    use std::cmp::{min, max};
    use std::collections::HashMap;

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
}