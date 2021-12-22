pub mod solutions {
    use lazy_static::lazy_static;
    use regex::Regex;

    use crate::AocBufReader;

    lazy_static! {
        static ref INPUT_RE: Regex = Regex::new(
            r"^([fon]*) x=([0-9]*)\.\.([0-9]*),y=([0-9]*)\.\.([0-9]*),z=([0-9]*)\.\.([0-9]*)"
        ).unwrap();
    }


    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct InclusiveRange {
        min: usize,
        max: usize,
    }


    impl InclusiveRange {
        fn new(min: usize, max: usize) -> InclusiveRange {
            InclusiveRange { min, max }
        }
    }


    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct Cube {
        on: bool,
        x: InclusiveRange,
        y: InclusiveRange,
        z: InclusiveRange,
    }


    impl Cube {
        fn new(
            on: bool,
            min_x: usize, max_x: usize,
            min_y: usize, max_y: usize,
            min_z: usize, max_z: usize
        ) -> Cube {
            Cube {
                on: on,
                x: InclusiveRange::new(min_x, max_x),
                y: InclusiveRange::new(min_y, max_y),
                z: InclusiveRange::new(min_z, max_z)
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
                capture.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                capture.get(3).unwrap().as_str().parse::<usize>().unwrap(),
                capture.get(4).unwrap().as_str().parse::<usize>().unwrap(),
                capture.get(5).unwrap().as_str().parse::<usize>().unwrap(),
                capture.get(6).unwrap().as_str().parse::<usize>().unwrap(),
                capture.get(7).unwrap().as_str().parse::<usize>().unwrap(),
            )
        }
    }


    fn read_input(aoc_reader: AocBufReader) -> Vec<Cube> {
        aoc_reader.into_iter().map(|line| Cube::from_string(line)).collect()
    }


    pub fn part_1(aoc_reader: AocBufReader) -> usize {
        let cubes = read_input(aoc_reader);
        1
    }


    #[cfg(test)]
    mod tests {
        use super::*;


        #[test]
        fn test_from_string() {
            assert_eq!(
                Cube::from_string("on x=10..12,y=10..12,z=10..12".to_string()),
                Cube::new(true, 10, 12, 10, 12, 10, 12)
            );
            assert_eq!(
                Cube::from_string("off x=1..22,y=3..4,z=5..6".to_string()),
                Cube::new(false, 1, 22, 3, 4, 5, 6)
            );
        }
    }
}