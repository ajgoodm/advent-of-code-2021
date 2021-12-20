pub mod solutions {
    use std::collections::HashSet;

    use crate::{AocBufReader, input::read_input};


    #[derive(Debug, PartialEq, Eq, Hash)]
    struct Point {
        row: isize,
        column: isize,
    }


    impl Point {
        fn new(row: isize, column: isize) -> Point {
            Point { row, column }
        }
    }


    struct Image {
        enhancement_algorithm: Vec<bool>,
        light_pixels: HashSet<Point>
    }


    fn read_input(mut aoc_reader: AocBufReader) -> Image {
        let algorithm_line = aoc_reader.next().unwrap();

        let mut enhancement_algorithm: Vec<bool> = vec![];
        for c in algorithm_line.chars() {
            match c {
                '.' => enhancement_algorithm.push(false),
                '#' => enhancement_algorithm.push(true),
                _ => panic!("Unknown character!")
            }
        }

        aoc_reader.next();  // blank line
        let mut light_pixels: HashSet<Point> = HashSet::new();
        let mut row: isize = 0;
        while let Some(line) = aoc_reader.next() {
            for (column, c) in line.chars().enumerate() {
                match c {
                    '.' => (),
                    '#' => { light_pixels.insert(Point::new(row, column as isize)); },
                    _ => panic!("Unknown character!")
                }
            }
            row += 1;
        }
        Image { enhancement_algorithm, light_pixels }
    }


    pub fn part_1(aoc_reader: AocBufReader) -> usize {
        let mut image = read_input(aoc_reader);
        1
    }
}