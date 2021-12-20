pub mod solutions {
    use std::collections::{HashMap, HashSet};

    use lazy_static::lazy_static;
    use regex::Regex;

    use crate::AocBufReader;

    lazy_static! {
        static ref SCANNER_HEADER: Regex = Regex::new(
            r"^--- scanner ([0-9]+) ---$"
        ).unwrap();
    }


    enum Rotation {
        X_CW,
        Y_CW,
        Z_CW,
    }


    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
    struct Point {
        x: isize,
        y: isize,
        z: isize,
    }


    impl Point {
        fn new(x: isize, y: isize, z: isize) -> Point {
            Point { x, y, z }
        }

        fn manhattan_distance(&self, other: &Point) -> usize {
            (self.x - other.x).abs() as usize
            + (self.y - other.y).abs() as usize
            + (self.z - other.z).abs() as usize
        }

        fn displacement(&self, other: &Point) -> Point {
            Point::new(other.x - self.x, other.y - self.y, other.z - self.z)
        }

        fn add(&self, other: &Point) -> Point {
            Point::new(self.x + other.x, self.y + other.y, self.z + other.z)
        }

        fn rotate_about_x_cw(&self) -> Point {
            Point::new(
                self.x, self.z, -self.y
            )
        }

        fn rotate_about_y_cw(&self) -> Point {
            Point::new(
                -self.z, self.y, self.x
            )
        }

        fn rotate_about_z_cw(&self) -> Point {
            Point::new(
                self.y, -self.x, self.z
            )
        }

        fn relative_orientations() -> Vec<Vec<Rotation>> {
            vec![
                vec![],  // facing x+
                vec![Rotation::X_CW],
                vec![Rotation::X_CW, Rotation::X_CW],
                vec![Rotation::X_CW, Rotation::X_CW, Rotation::X_CW],
                vec![Rotation::Y_CW],  // facing z+
                vec![Rotation::Y_CW, Rotation::Z_CW],
                vec![Rotation::Y_CW, Rotation::Z_CW, Rotation::Z_CW],
                vec![Rotation::Y_CW, Rotation::Z_CW, Rotation::Z_CW, Rotation::Z_CW],
                vec![Rotation::Y_CW, Rotation::Y_CW],  // facing x-
                vec![Rotation::Y_CW, Rotation::Y_CW, Rotation::X_CW],
                vec![Rotation::Y_CW, Rotation::Y_CW, Rotation::X_CW, Rotation::X_CW],
                vec![Rotation::Y_CW, Rotation::Y_CW, Rotation::X_CW, Rotation::X_CW, Rotation::X_CW],
                vec![Rotation::Y_CW, Rotation::Y_CW, Rotation::Y_CW],  // facing z-
                vec![Rotation::Y_CW, Rotation::Y_CW, Rotation::Y_CW, Rotation::Z_CW],
                vec![Rotation::Y_CW, Rotation::Y_CW, Rotation::Y_CW, Rotation::Z_CW, Rotation::Z_CW],
                vec![Rotation::Y_CW, Rotation::Y_CW, Rotation::Y_CW, Rotation::Z_CW, Rotation::Z_CW, Rotation::Z_CW],
                vec![Rotation::Z_CW],  // facing y-
                vec![Rotation::Z_CW, Rotation::Y_CW],
                vec![Rotation::Z_CW, Rotation::Y_CW, Rotation::Y_CW],
                vec![Rotation::Z_CW, Rotation::Y_CW, Rotation::Y_CW, Rotation::Y_CW],
                vec![Rotation::Z_CW, Rotation::Z_CW, Rotation::Z_CW],  // facing y+
                vec![Rotation::Z_CW, Rotation::Z_CW, Rotation::Z_CW, Rotation::Y_CW],
                vec![Rotation::Z_CW, Rotation::Z_CW, Rotation::Z_CW, Rotation::Y_CW, Rotation::Y_CW],
                vec![Rotation::Z_CW, Rotation::Z_CW, Rotation::Z_CW, Rotation::Y_CW, Rotation::Y_CW, Rotation::Y_CW],
            ]
        }

        fn apply_rotation(p: &Point, rotations: &Vec<Rotation>) -> Point {
            let mut new_point = *p;
            for r in rotations {
                match r {
                    Rotation::X_CW => new_point = new_point.rotate_about_x_cw(),
                    Rotation::Y_CW => new_point = new_point.rotate_about_y_cw(),
                    Rotation::Z_CW => new_point = new_point.rotate_about_z_cw()
                }
            }
            new_point
        }
    }


    struct Scanner {
        id: usize,
        probes: HashSet<Point>,
    }


    impl Scanner {
        fn from_aoc_iter_mut(aoc_reader: &mut AocBufReader) -> Option<Scanner> {
            if let Some(header) = aoc_reader.next() {
                let scanner_id: usize = SCANNER_HEADER.captures(&header).unwrap().get(1)
                    .unwrap().as_str().parse().unwrap();
                let mut probes: HashSet<Point> = HashSet::new();
                loop {
                    let line = aoc_reader.next();
                    if line == None || line.as_ref().unwrap().len() == 0 {
                        break
                    }
                    let vals: Vec<isize> = line.unwrap().split(",").map(|s| s.parse().unwrap()).collect();
                    probes.insert(Point::new(vals[0], vals[1], vals[2]));
                }

                Some(Scanner { id: scanner_id, probes: probes })
            } else {
                None
            }
        }

        fn matches_other(&self, other: &Scanner) -> Option<Vec<Point>> {
            for rotation in Point::relative_orientations() {
                let reoriented_points: Vec<Point> = other.probes.iter().map(|p| {
                    Point::apply_rotation(p, &rotation)
                }).collect();
                let mut displacements: HashMap<Point, usize> = HashMap::new();
                for point in self.probes.iter() {
                    for reoriented_point in reoriented_points.iter() {
                        let displacement: Point = reoriented_point.displacement(&point);
                        *displacements.entry(displacement).or_insert(0) += 1;
                        if *displacements.get(&displacement).unwrap() >= 12 {
                            return Some(
                                reoriented_points.into_iter().map(|p| p.add(&displacement)).collect()
                            )
                        }
                    }
                }
            }
            None
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
        let mut oriented_scanner_net: Scanner = Scanner {
            id: 0,
            probes: scanners[0].probes.iter().map(|p| *p).collect()
        };
        let mut oriented_scanners: HashSet<usize> = vec![scanners[0].id].into_iter().collect();

        while oriented_scanners.len() < scanners.len() {
            for scanner in scanners.iter() {
                if oriented_scanners.contains(&scanner.id) {
                    continue
                }
                if let Some(pts) = &oriented_scanner_net.matches_other(scanner) {
                    oriented_scanner_net.probes.extend(pts);
                    oriented_scanners.insert(scanner.id);
                }
            }
        }

        oriented_scanner_net.probes.len()
    }

    pub fn part_2(aoc_reader: AocBufReader) -> usize {
        let scanners: Vec<Scanner> = read_input(aoc_reader);
        let mut oriented_scanner_net: Scanner = Scanner {
            id: 0,
            probes: scanners[0].probes.iter().map(|p| *p).collect()
        };
        let mut oriented_scanners: HashSet<usize> = vec![scanners[0].id].into_iter().collect();

        while oriented_scanners.len() < scanners.len() {
            for scanner in scanners.iter() {
                if oriented_scanners.contains(&scanner.id) {
                    continue
                }
                if let Some(pts) = &oriented_scanner_net.matches_other(scanner) {
                    oriented_scanner_net.probes.extend(pts);
                    oriented_scanners.insert(scanner.id);
                }
            }
        }

        let mut max_dist: usize = usize::MIN;
        for point_1 in oriented_scanner_net.probes.iter() {
            for point_2 in oriented_scanner_net.probes.iter() {
                let d = point_1.manhattan_distance(point_2);
                if d > max_dist {
                    max_dist = d
                }
            }
        }

        max_dist
    }
}