pub mod solutions {
    use std::cmp::{max, min};

    use lazy_static::lazy_static;
    use regex::Regex;

    use crate::AocBufReader;

    lazy_static! {
        static ref INPUT_RE: Regex = Regex::new(
            r"^([fon]*) x=(-?[0-9]*)\.\.(-?[0-9]*),y=(-?[0-9]*)\.\.(-?[0-9]*),z=(-?[0-9]*)\.\.(-?[0-9]*)"
        ).unwrap();
    }


    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct InclusiveRange {
        min: isize,
        max: isize,
    }


    impl InclusiveRange {
        fn new(min: isize, max: isize) -> InclusiveRange {
            InclusiveRange { min, max }
        }
    }


    #[derive(Debug, PartialEq, Eq)]
    struct Cube {
        on: bool,
        x: InclusiveRange,
        y: InclusiveRange,
        z: InclusiveRange,
        sub_cubes: Vec<Box<Cube>>,
    }


    impl Cube {
        fn new(
            on: bool,
            min_x: isize, max_x: isize,
            min_y: isize, max_y: isize,
            min_z: isize, max_z: isize
        ) -> Cube {
            Cube {
                on: on,
                x: InclusiveRange::new(min_x, max_x),
                y: InclusiveRange::new(min_y, max_y),
                z: InclusiveRange::new(min_z, max_z),
                sub_cubes: vec![]
            }
        }

        fn from_string(s: String) -> Cube {
            let capture = INPUT_RE.captures(&s).unwrap();
            let on_off = capture.get(1).unwrap().as_str();
            let cube_is_on: bool;
            match on_off {
                "on" => cube_is_on = true,
                "off" => cube_is_on = false,
                _ => panic!("UP IS DOWN AHHH")
            }

            Cube::new(
                cube_is_on,
                capture.get(2).unwrap().as_str().parse::<isize>().unwrap(),
                capture.get(3).unwrap().as_str().parse::<isize>().unwrap(),
                capture.get(4).unwrap().as_str().parse::<isize>().unwrap(),
                capture.get(5).unwrap().as_str().parse::<isize>().unwrap(),
                capture.get(6).unwrap().as_str().parse::<isize>().unwrap(),
                capture.get(7).unwrap().as_str().parse::<isize>().unwrap(),
            )
        }

        fn is_disjoint(&self, other: &Cube) -> bool {
            other.x.min >= self.x.max || other.y.min >= self.y.max || other.z.min >= self.z.max ||
            other.x.max <= self.x.min || other.y.max <= self.y.min || other.z.max <= self.z.min
        }

        fn is_contained_by(&self, other: &Cube) -> bool {
            other.x.min <= self.x.min && other.y.min <= self.y.min && other.z.min <= self.z.min &&
            other.x.max >= self.x.max && other.y.max >= self.y.max && other.z.max >= self.z.max
        }

        fn intersection(&self, other: &Cube, on: bool) -> Option<Cube> {
            if self.is_disjoint(other) {
                None
            } else {
                let mut cube = Cube::new(
                    on,
                    max(self.x.min, other.x.min),
                    min(self.x.max, other.x.max),
                    max(self.y.min, other.y.min),
                    min(self.y.max, other.y.max),
                    max(self.z.min, other.z.min),
                    min(self.z.max, other.z.max),
                );
                let sub_cubes: Vec<Box<Cube>> = self.sub_cubes.iter()
                    .map(|s| s.intersection(other, on))
                    .filter(|s| *s != None)
                    .map(|s| Box::new(s.unwrap()))
                    .collect();
                cube.sub_cubes = sub_cubes;
                Some(cube)
            }
        }

        fn difference(&self, other: &Cube) -> Option<Cube> {
            if self.is_contained_by(other) {
                None
            } else {
                let mut cube = Cube {
                    sub_cubes: vec![],
                    ..*self
                };
                let sub_cubes: Vec<Box<Cube>> = self.sub_cubes.iter()
                    .map(|s| s.difference(other))
                    .filter(|s| *s != None)
                    .map(|s| Box::new(s.unwrap()))
                    .collect();
                if let Some(intersection) = cube.intersection(other, false) {
                    cube.sub_cubes.push(Box::new(intersection));
                }
                cube.sub_cubes = sub_cubes;
                Some(cube)
            }
        }

        fn merge_cubes(mut self, merged_cubes: &mut Vec<Box<Cube>>) {
            // TODO
            for cube in merged_cubes {
                if let Some(intersection) = self.intersection(cube, self.on) {
                    cube.sub_cubes.push(Box::new(intersection));
                }
            }
        }

        fn boundary_volume(&self) -> isize {
            (self.x.max - self.x.min + 1) * (self.y.max - self.y.min + 1) * (self.z.max - self.z.min + 1)
        }

        fn total_volume(&self) -> isize {
            let sub_cube_volumes: isize = self.sub_cubes.iter().map(|cube| cube.total_volume()).sum();
            match self.on {
                true => self.boundary_volume() + sub_cube_volumes,
                false => sub_cube_volumes - self.boundary_volume()
            }
        }
    }


    fn read_input(aoc_reader: AocBufReader) -> Vec<Cube> {
        aoc_reader.into_iter().map(|line| Cube::from_string(line)).collect()
    }


    fn merge_cubes(cubes: Vec<Cube>) -> Vec<Box<Cube>> {
        let mut merged_cubes: Vec<Box<Cube>> = vec![];
        for cube in cubes {
            cube.merge_cubes(&mut merged_cubes);
        }
        merged_cubes
    }


    pub fn part_1(aoc_reader: AocBufReader) -> usize {
        let cubes = read_input(aoc_reader);
        let merged_cubes: Vec<Box<Cube>> = merge_cubes(cubes);
        merged_cubes.into_iter().map(|cube| cube.total_volume() as usize).sum()
    }


    #[cfg(test)]
    mod tests {
        use super::*;


        #[test]
        fn test_from_string() {
            assert_eq!(
                Cube::from_string("on x=-10..12,y=10..12,z=10..12".to_string()),
                Cube::new(true, -10, 12, 10, 12, 10, 12)
            );
            assert_eq!(
                Cube::from_string("off x=1..22,y=3..4,z=5..6".to_string()),
                Cube::new(false, 1, 22, 3, 4, 5, 6)
            );
        }


        #[test]
        fn test_intersection() {
            assert_eq!(
                Cube::from_string("on x=0..1,y=0..1,z=0..1".to_string()).intersection(
                &Cube::from_string("on x=1..2,y=1..2,z=1..2".to_string()), true), None
            );

            assert_eq!(
                Cube::from_string("on x=0..2,y=0..1,z=0..1".to_string()).intersection(
                &Cube::from_string("on x=1..3,y=0..1,z=0..1".to_string()), true), Some(
                Cube::from_string("on x=1..2,y=0..1,z=0..1".to_string()))
            );

            assert_eq!(
                Cube::from_string("on x=0..2,y=0..1,z=0..1".to_string()).intersection(
                &Cube::from_string("on x=1..3,y=0..1,z=0..1".to_string()), false), Some(
                Cube::from_string("off x=1..2,y=0..1,z=0..1".to_string()))
            );
        }

        #[test]
        fn test_total_volume() {
            let mut cube = Cube::from_string("on x=0..9,y=0..9,z=0..9".to_string());
            assert_eq!(cube.total_volume(), 1000);

            cube.sub_cubes.push(
                Box::new(Cube::from_string("off x=0..2,y=0..2,z=0..2".to_string()))
            );
            assert_eq!(cube.total_volume(), 1000 - 27);

            cube.sub_cubes[0].sub_cubes.push(
                Box::new(Cube::from_string("on x=0..1,y=0..1,z=0..1".to_string()))
            );
            assert_eq!(cube.total_volume(), 1000 - 27 + 8);
        }
    }
}