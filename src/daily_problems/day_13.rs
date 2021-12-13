pub mod solutions {
    use std::collections::HashSet;

    use lazy_static::lazy_static;
    use regex::Regex;

    use crate::AocBufReader;

    lazy_static! {
        static ref FOLD_REGEX: Regex = Regex::new("([x,y])=([0-9])*").unwrap();
    }


    enum Axes {
        X,
        Y,
    }


    struct Paper {
        marks: HashSet<(usize, usize)>,
        folds: Vec<(Axes, usize)>
    }


    impl Paper {
        fn from_reader(mut aoc_reader: AocBufReader) -> Paper {
            let mut marks: HashSet<(usize, usize)> = HashSet::new();
            let mut line: String = aoc_reader.next().unwrap();

            while line.len() > 0 {
                let coord: Vec<usize> = line.split(",").into_iter().map(|val| val.parse::<usize>().unwrap()).collect();
                marks.insert((coord[0], coord[1]));
                line = aoc_reader.next().unwrap()
            }

            let mut folds: Vec<(Axes, usize)> = vec![];
            while let Some(line) = aoc_reader.next() {
                let cap = FOLD_REGEX.captures(&line).unwrap();
                match cap.get(1).unwrap().as_str() {
                    "x" => folds.push((Axes::X, cap.get(2).unwrap().as_str().parse::<usize>().unwrap())),
                    "y" => folds.push((Axes::Y, cap.get(2).unwrap().as_str().parse::<usize>().unwrap())),
                    _ => panic!("unknown AXIS!")
                }


            }

            Paper {marks: marks, folds: folds}
        }
    }


    pub fn part_1(aoc_reader: AocBufReader) -> usize {
        let paper = Paper::from_reader(aoc_reader);
        1
    }
}