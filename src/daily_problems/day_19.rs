pub mod solutions {
    use std::iter::Scan;

    use lazy_static::lazy_static;
    use regex::Regex;

    use crate::{AocBufReader, input::read_input};

    lazy_static! {
        static ref SCANNER_HEADER: Regex = Regex::new(
            r"^--- scanner ([0-9]+) ---$"
        ).unwrap();
    }


    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    struct Point {
        x: isize,
        y: isize,
        z: isize,
    }


    impl Point {
        fn new(x: isize, y: isize, z: isize) -> Point {
            Point { x, y, z }
        }
    }


    struct Scanner {
        id: usize,
        probes: Vec<Point>,
    }


    impl Scanner {
        fn from_aoc_iter_mut(mut aoc_reader: &mut AocBufReader) -> Option<Scanner> {
            if let Some(header) = aoc_reader.next() {
                let scanner_id: usize = SCANNER_HEADER.captures(&header).unwrap().get(1)
                    .unwrap().as_str().parse().unwrap();
                let mut probes: Vec<Point> = vec![];
                loop {
                    let line = aoc_reader.next();
                    if line == None || line.as_ref().unwrap().len() == 0 {
                        break
                    }
                    let vals: Vec<isize> = line.unwrap().split(",").map(|s| s.parse().unwrap()).collect();
                    probes.push(Point::new(vals[0], vals[0], vals[0]));
                }

                Some(Scanner { id: scanner_id, probes: probes })
            } else {
                None
            }
        }
    }


    fn read_input(mut aoc_reader: AocBufReader) -> Vec<Scanner> {
        let mut scanners: Vec<Scanner> = vec![];
        while let Some(scanner) = Scanner::from_aoc_iter_mut(&mut aoc_reader) {
            scanners.push(scanner)
        }
        scanners
    }


    pub fn part_1(aoc_reader: AocBufReader) -> usize {
        let scanners: Vec<Scanner> = read_input(aoc_reader);
        1
    }
}