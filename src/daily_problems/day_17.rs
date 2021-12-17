pub mod solutions {
    use crate::AocBufReader;

    use lazy_static::lazy_static;
    use regex::Regex;

    lazy_static! {
        static ref TARGET_RE: Regex = Regex::new(
            r"^target area: x=(-?[0-9]*)..(-?[0-9]*), y=(-?[0-9]*)..(-?[0-9]*)$"
        ).unwrap();
    }


    #[derive(PartialEq, Eq, Debug)]
    struct Target {
        min_x: isize,
        min_y: isize,
        max_x: isize,
        max_y: isize
    }

    impl Target {
        fn new(min_x: isize, min_y: isize, max_x: isize, max_y: isize) -> Target {
            Target { min_x, min_y, max_x, max_y }
        }
    }

    struct Cannon {
        t: usize,

        sx: isize,
        sy: isize,

        vx: isize,
        vy: isize,
    }

    impl Cannon {
        fn new(vx: isize, vy: isize) -> Cannon {
            Cannon {
                t: 0, sx: 0, sy: 0, vx, vy
            }
        }
    }

    fn read_input(input: String) -> Target {
        let cap = TARGET_RE.captures(&input).unwrap();
        Target::new(
            cap.get(1).unwrap().as_str().parse::<isize>().unwrap(),
            cap.get(3).unwrap().as_str().parse::<isize>().unwrap(),
            cap.get(2).unwrap().as_str().parse::<isize>().unwrap(),
            cap.get(4).unwrap().as_str().parse::<isize>().unwrap(),
        )

    }

    fn _part_1(input: String) -> usize {
        let target = read_input(input);


        1
    }

    pub fn part_1(mut aoc_reader: AocBufReader) -> usize {
        _part_1(aoc_reader.next().unwrap())
    }


    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_read_input() {
            assert_eq!(
                read_input("target area: x=20..30, y=-10..-5".to_string()),
                Target::new(20, -10, 30, -5)
            );

            assert_eq!(
                read_input("target area: x=155..182, y=-117..-67".to_string()),
                Target::new(155, -117, 182, -67)
            );
        }
    }
}