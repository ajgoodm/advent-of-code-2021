pub mod solutions {
    use std::collections::HashSet;
    use std::hash::Hash;

    use itertools::Itertools;

    use crate::AocBufReader;
    use crate::utils::conversion::binary_bool_vec_to_usize;


    struct ImageBounds {
        min_row: isize,
        min_col: isize,
        max_row: isize,
        max_col: isize
    }


    impl ImageBounds {
        fn new(min_row: isize, min_col: isize, max_row: isize, max_col: isize)  -> ImageBounds {
            ImageBounds { min_row, min_col, max_row, max_col }
        }
    }


    #[derive(Debug, PartialEq, Eq, Hash)]
    struct Point {
        row: isize,
        column: isize,
    }


    impl Point {
        fn new(row: isize, column: isize) -> Point {
            Point { row, column }
        }

        fn self_and_neighbors(&self) -> Vec<Point> {
            (-1isize..=1).cartesian_product(-1isize..=1).map(|(drow, dcol)| {
                Point::new(self.row + drow, self.column + dcol)
            }).collect::<Vec<Point>>()
        }
    }


    struct Image {
        enhancement_algorithm: Vec<bool>,
        light_pixels: Option<HashSet<Point>>,
        dark_pixels: Option<HashSet<Point>>,
        flicker: bool,
    }


    impl Image {
        fn new(enhancement_algorithm: Vec<bool>, light_pixels: HashSet<Point>, flicker: bool) -> Image {
            Image { enhancement_algorithm, light_pixels: Some(light_pixels), dark_pixels: None, flicker }
        }

        fn light_pixels_and_neighbors(&self) -> HashSet<Point> {
            let mut light_pixels_and_neighbors: HashSet<Point> = HashSet::new();
            for pixel in self.light_pixels.as_ref().unwrap().iter() {
                light_pixels_and_neighbors.extend(pixel.self_and_neighbors())
            }
            light_pixels_and_neighbors
        }

        fn dark_pixels_and_neighbors(&self) -> HashSet<Point> {
            let mut dark_pixels_and_neighbors: HashSet<Point> = HashSet::new();
            for pixel in self.dark_pixels.as_ref().unwrap().iter() {
                dark_pixels_and_neighbors.extend(pixel.self_and_neighbors())
            }
            dark_pixels_and_neighbors
        }

        fn get_algorithm_key(&self, pixel: &Point) -> usize {
            if let Some(light_pixels) = self.light_pixels.as_ref() {
                return binary_bool_vec_to_usize(
                    pixel.self_and_neighbors().into_iter().map(|p| {
                        light_pixels.contains(&p)
                    }).collect::<Vec<bool>>()
                )
            }
            if let Some(dark_pixels) = self.dark_pixels.as_ref() {
                return binary_bool_vec_to_usize(
                    pixel.self_and_neighbors().into_iter().map(|p| {
                        !dark_pixels.contains(&p)
                    }).collect::<Vec<bool>>()
                )
            }

            panic!("AHHHHHH!");
        }

        fn step(&mut self) {
            if self.flicker {
                // This is in the scenario where the whole universe flickers every other step
                if self.light_pixels != None {
                    // we are currently tracking light pixels, but won't be able to next round
                    // there will be infinitely many.
                    let mut new_dark_pixels: HashSet<Point> = HashSet::new();
                    for pixel in self.light_pixels_and_neighbors() {
                        if !self.enhancement_algorithm[self.get_algorithm_key(&pixel)] {
                            new_dark_pixels.insert(pixel);
                        }
                    }
                    self.dark_pixels = Some(new_dark_pixels);
                    self.light_pixels = None;
                } else {
                    let mut new_light_pixels: HashSet<Point> = HashSet::new();
                    for pixel in self.dark_pixels_and_neighbors() {
                        if self.enhancement_algorithm[self.get_algorithm_key(&pixel)] {
                            new_light_pixels.insert(pixel);
                        }
                    }
                    self.light_pixels = Some(new_light_pixels);
                    self.dark_pixels = None;
                }
            } else {
                // We are in the more stable case where there are always finitely many light pixels.
                let mut new_light_pixels: HashSet<Point> = HashSet::new();
                for pixel in self.light_pixels_and_neighbors() {
                    if self.enhancement_algorithm[self.get_algorithm_key(&pixel)] {
                        new_light_pixels.insert(pixel);
                    }
                }
                self.light_pixels = Some(new_light_pixels)
            }
        }

        fn print(&self) {
            let min_row: isize = self.light_pixels.as_ref().unwrap().iter().map(|p| p.row).min().unwrap();
            let min_col: isize = self.light_pixels.as_ref().unwrap().iter().map(|p| p.column).min().unwrap();
            let max_row: isize = self.light_pixels.as_ref().unwrap().iter().map(|p| p.row).max().unwrap();
            let max_col: isize = self.light_pixels.as_ref().unwrap().iter().map(|p| p.column).max().unwrap();

            for row in min_row..=max_row {
                println!(
                    "{}",
                    (min_col..=max_col).map(|col| {
                        match self.light_pixels.as_ref().unwrap().contains(&Point::new(row, col)) {
                            true => '#',
                            false => '.'
                        }
                    }).collect::<String>()
                );
            }
        }
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

        // If the first character of the enhancement algorithm is `#`,
        // All pixels (infinitely many in the abyss) will turn on
        // during the first step. For the solution to be finite (on even steps),
        // the last character _must_ be `.`
        let flicker: bool = enhancement_algorithm[0];
        Image::new(enhancement_algorithm, light_pixels, flicker)
    }


    pub fn part_1(aoc_reader: AocBufReader) -> usize {
        let mut image = read_input(aoc_reader);
        image.step();
        image.step();
        image.light_pixels.as_ref().unwrap().len()
    }


    pub fn part_2(aoc_reader: AocBufReader) -> usize {
        let mut image = read_input(aoc_reader);
        for _ in 0..50 {
            image.step();
        }
        image.light_pixels.as_ref().unwrap().len()
    }


    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_self_and_neighbors() {
            assert_eq!(
                Point::new(1, 1).self_and_neighbors(),
                vec![
                    Point::new(0, 0), Point::new(0, 1), Point::new(0, 2),
                    Point::new(1, 0), Point::new(1, 1), Point::new(1, 2),
                    Point::new(2, 0), Point::new(2, 1), Point::new(2, 2),
                ]
            );
        }
    }
}