pub mod solutions {
    use std::collections::HashSet;

    use itertools::Itertools;
    use lazy_static::lazy_static;
    use regex::Regex;

    use crate::AocBufReader;

    lazy_static! {
        static ref FOLD_REGEX: Regex = Regex::new("([x,y])=([0-9]*)$").unwrap();
    }

    #[derive(Clone, Copy)]
    enum Axes {
        X,
        Y,
    }


    struct Paper {
        marks: HashSet<(isize, isize)>,
        folds: Vec<(Axes, isize)>
    }


    impl Paper {
        fn from_reader(mut aoc_reader: AocBufReader) -> Paper {
            let mut marks: HashSet<(isize, isize)> = HashSet::new();
            let mut line: String = aoc_reader.next().unwrap();

            while line.len() > 0 {
                let coord: Vec<isize> = line.split(",").into_iter().map(|val| val.parse::<isize>().unwrap()).collect();
                marks.insert((coord[0], coord[1]));
                line = aoc_reader.next().unwrap()
            }

            let mut folds: Vec<(Axes, isize)> = vec![];
            while let Some(line) = aoc_reader.next() {
                let cap = FOLD_REGEX.captures(&line).unwrap();
                match cap.get(1).unwrap().as_str() {
                    "x" => folds.push((Axes::X, cap.get(2).unwrap().as_str().parse::<isize>().unwrap())),
                    "y" => folds.push((Axes::Y, cap.get(2).unwrap().as_str().parse::<isize>().unwrap())),
                    _ => panic!("unknown AXIS!")
                }


            }

            Paper {marks: marks, folds: folds}
        }

        fn _fold_1d(crease: isize, val: isize) -> isize {
            crease + (crease - val)
        }

        fn _fold_along_x(&mut self, fold_x_val: isize) {
            let mut marks_to_add: HashSet<(isize, isize)> = HashSet::new();
            let mut marks_to_remove: HashSet<(isize, isize)> = HashSet::new();

            for mark in &self.marks {
                match mark {
                    (x_val, y_val) => {
                        if *x_val > fold_x_val {
                            marks_to_remove.insert(*mark);
                            marks_to_add.insert((Paper::_fold_1d(fold_x_val, *x_val),  *y_val));
                        }
                    }
                }
            }
            self.marks = self.marks.difference(&marks_to_remove).map(|x| x.to_owned()).collect();
            self.marks = self.marks.union(&marks_to_add).map(|x| x.to_owned()).collect();
        }

        fn _fold_along_y(&mut self, fold_y_val: isize) {
            let mut marks_to_add: HashSet<(isize, isize)> = HashSet::new();
            let mut marks_to_remove: HashSet<(isize, isize)> = HashSet::new();

            for mark in &self.marks {
                match mark {
                    (x_val, y_val) => {
                        if *y_val > fold_y_val {
                            marks_to_remove.insert(*mark);
                            marks_to_add.insert((*x_val, Paper::_fold_1d(fold_y_val, *y_val)));
                        }
                    }
                }
            }
            self.marks = self.marks.difference(&marks_to_remove).map(|x| x.to_owned()).collect();
            self.marks = self.marks.union(&marks_to_add).map(|x| x.to_owned()).collect();
        }

        fn fold(&mut self, axis: Axes, crease_val: isize) {
            match axis {
                Axes::X => self._fold_along_x(crease_val),
                Axes::Y => self._fold_along_y(crease_val)
            }
        }

        fn print(&self) {
            let max_x = self.marks.iter().fold(isize::MIN, |a, (x, y)| a.max(*x));
            let max_y = self.marks.iter().fold(isize::MIN, |a, (x, y)| a.max(*y));

            let mut grid: Vec<Vec<&str>> = vec![];
            for _ in 0..=max_y {
                grid.push((0..=max_x).map(|_| ".").collect());
            }

            for mark in &self.marks {
                match mark {
                    (x_val, y_val) => grid[*y_val as usize][*x_val as usize] = "#"
                }
            }

            for row in grid {
                println!("{}", row.iter().join(""));
            }
        }
    }


    pub fn part_1(aoc_reader: AocBufReader) -> usize {
        let mut paper = Paper::from_reader(aoc_reader);
        let folds = paper.folds.clone();
        match folds[0] {
            (axis, crease_val) => paper.fold(axis, crease_val)
        }

        paper.marks.len()
    }


    pub fn part_2(aoc_reader: AocBufReader) -> usize {
        let mut paper = Paper::from_reader(aoc_reader);
        let folds = paper.folds.clone();
        for fold in folds {
            match fold {
                (axis, crease_val) => paper.fold(axis, crease_val)
            }
        }

        paper.print();
        paper.marks.len()
    }
}