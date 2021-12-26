pub mod solutions {
    use std::cmp::{max, min};
    use std::collections::HashSet;

    use itertools::Itertools;
    use itertools::Zip;
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
            assert!(max >= min);
            InclusiveRange { min, max }
        }

        fn explode(&self, other: &InclusiveRange) -> Vec<InclusiveRange> {
            if other.max < self.min || other.min > self.max {
                return vec![self.clone()]
            }

            let mut subranges: Vec<InclusiveRange> = vec![];
            if other.min > self.min && other.min <= self.max {
                subranges.push(InclusiveRange::new(self.min, other.min - 1));
            }

            if other.max < self.max {
                if subranges.len() == 1 {
                    subranges.push(
                        InclusiveRange::new(subranges[0].max + 1, other.max)
                    );
                } else {
                    subranges.push(
                        InclusiveRange::new(self.min, other.max)
                    );
                }
            }

            if subranges.len() > 0 {
                subranges.push(
                    InclusiveRange::new(subranges.last().unwrap().max + 1, self.max)
                )
            } else {
                subranges.push(InclusiveRange::new(self.min, self.max))
            }

            subranges
        }
    }


    #[derive(Debug, PartialEq, Eq)]
    struct Cube {
        on: bool,
        x: InclusiveRange,
        y: InclusiveRange,
        z: InclusiveRange,
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
            other.x.min > self.x.max || other.y.min > self.y.max || other.z.min > self.z.max ||
            other.x.max < self.x.min || other.y.max < self.y.min || other.z.max < self.z.min
        }

        fn intersection(&self, other: &Cube, on: bool) -> Option<Cube> {
            if self.is_disjoint(other) {
                None
            } else {
                let cube = Cube::new(
                    on,
                    max(self.x.min, other.x.min),
                    min(self.x.max, other.x.max),
                    max(self.y.min, other.y.min),
                    min(self.y.max, other.y.max),
                    max(self.z.min, other.z.min),
                    min(self.z.max, other.z.max),
                );
                Some(cube)
            }
        }

        fn print(&self) {
            println!(
                "x:{}-{}, y:{}-{}, z:{}-{}",
                self.x.min, self.x.max,
                self.y.min, self.y.max,
                self.z.min, self.z.max
            );
        }

        fn explode_into(&self, other: &Cube) -> Vec<Cube> {
            let mut shards: Vec<Cube> = vec![];
            for x_range in self.x.explode(&other.x) {
                for y_range in self.y.explode(&other.y) {
                    for z_range in self.z.explode(&other.z) {
                        shards.push(
                            Cube::new(
                                self.on,
                                x_range.min, x_range.max,
                                y_range.min, y_range.max,
                                z_range.min, z_range.max
                            )
                        )
                    }
                }
            }

            let mut shards_to_keep: Vec<Cube> = vec![];
            for shard in shards.into_iter() {
                if shard.is_disjoint(other) {
                    shards_to_keep.push(shard);
                }
            }

            shards_to_keep
        }

        fn boundary_volume(&self) -> isize {
            (self.x.max - self.x.min + 1) * (self.y.max - self.y.min + 1) * (self.z.max - self.z.min + 1)
        }
    }


    fn read_input(aoc_reader: AocBufReader) -> Vec<Cube> {
        aoc_reader.into_iter().map(|line| Cube::from_string(line)).collect()
    }


    fn explode_existing_cubes(existing_cubes: &Vec<Cube>, new_cube: Cube) -> Vec<Cube> {
        let mut exploded_cubes: Vec<Cube> = vec![];
        for existing_cube in existing_cubes.iter() {
            exploded_cubes.extend(existing_cube.explode_into(&new_cube))
        }
        if new_cube.on {
            exploded_cubes.push(new_cube)
        }

        println!("there are {} cubes", exploded_cubes.len());

        exploded_cubes
    }


    pub fn part_1(aoc_reader: AocBufReader) -> usize {
        // test in bound values: 590784
        let cubes = read_input(aoc_reader);

        let mut exploded_cubes: Vec<Cube> = vec![];
        for cube in cubes {
            exploded_cubes = explode_existing_cubes(&exploded_cubes, cube);
        }

        exploded_cubes.into_iter().map(|cube| cube.boundary_volume() as usize).sum()
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
                &Cube::from_string("on x=2..2,y=2..2,z=2..2".to_string()), true), None
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
        fn test_volume() {
            assert_eq!(Cube::from_string("on x=10..12,y=10..12,z=10..12".to_string()).boundary_volume(), 27);
            assert_eq!(Cube::from_string("on x=11..13,y=11..13,z=11..13".to_string()).boundary_volume(), 27);
            assert_eq!(
                Cube::from_string("on x=10..12,y=10..12,z=10..12".to_string()).intersection(
                    &Cube::from_string("on x=11..13,y=11..13,z=11..13".to_string()), true
                ).unwrap().boundary_volume(),
                8
            );
            assert_eq!(Cube::from_string("on x=1..1,y=1..1,z=1..1".to_string()).boundary_volume(), 1);
        }

        #[test]
        fn test_explode_range() {
            let sub_range = InclusiveRange::new(0, 4);

            assert_eq!(
                sub_range.explode(&InclusiveRange::new(5, 6)),
                vec![sub_range]
            );

            assert_eq!(
                sub_range.explode(&InclusiveRange::new(-1, 0)),
                vec![
                    InclusiveRange::new(0, 0),
                    InclusiveRange::new(1, 4)
                ]
            );

            assert_eq!(
                sub_range.explode(&InclusiveRange::new(0, 0)),
                vec![
                    InclusiveRange::new(0, 0),
                    InclusiveRange::new(1, 4)
                ]
            );

            assert_eq!(
                sub_range.explode(&InclusiveRange::new(1, 2)),
                vec![
                    InclusiveRange::new(0, 0),
                    InclusiveRange::new(1, 2),
                    InclusiveRange::new(3, 4)
                ]
            );


            assert_eq!(
                sub_range.explode(&InclusiveRange::new(3, 5)),
                vec![
                    InclusiveRange::new(0, 2),
                    InclusiveRange::new(3, 4),
                ]
            );

            assert_eq!(
                sub_range.explode(&InclusiveRange::new(4, 4)),
                vec![
                    InclusiveRange::new(0, 3),
                    InclusiveRange::new(4, 4),
                ]
            );



        }

        #[test]
        fn test_explode_cubes() {
            let cubes = Cube::from_string("on x=0..3,y=0..3,z=0..3".to_string()).explode_into(
                &Cube::from_string("on x=4..5,y=4..5,z=4..5".to_string())
            );
            assert_eq!(cubes.len(), 1);
            assert_eq!(cubes[0].boundary_volume(), 64);


            assert_eq!(
                Cube::from_string("on x=0..3,y=0..3,z=0..3".to_string()).explode_into(
                    &Cube::from_string("on x=0..3,y=0..3,z=0..3".to_string())
                ).len(), 0
            );

            assert_eq!(
                Cube::from_string("on x=0..2,y=0..2,z=0..2".to_string()).explode_into(
                    &Cube::from_string("off x=1..1,y=1..1,z=1..1".to_string())
                ).len(), 26
            );
        }
    }
}