pub mod solutions {
    use itertools::Itertools;

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

        fn contains_probe(&self, probe: &Probe) -> bool {
            self.min_x <= probe.sx && probe.sx <= self.max_x && self.min_y <= probe.sy && probe.sy <= self.max_y
        }
    }

    struct Probe {
        t: usize,

        sx: isize,
        sy: isize,

        vx: isize,
        vy: isize,
    }

    impl Probe {
        fn new(vx: isize, vy: isize) -> Probe {
            Probe {
                t: 0, sx: 0, sy: 0, vx, vy
            }
        }

        fn step(&mut self) {
            self.t += 1;
            self.sx += self.vx;
            self.sy += self.vy;

            if self.vx > 0 {
                self.vx -= 1;
            } else if self.vx < 0 {
                self.vx += 1;
            }

            self.vy -= 1;
        }

        fn hits(&mut self, target: &Target) -> bool {
            loop {
                if self.sy < target.min_y || self.sx > target.max_x { return false }
                if target.contains_probe(&self) { return true }
                self.step()
            }
        }

        fn print(&self) {
            println!("({}, {})", self.sx, self.sy);
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


    fn get_min_vx(target: &Target) -> isize {
        (-0.5 + (0.25 + 2.0 * target.min_x as f64).powf(0.5)).floor() as isize
    }


    fn _part_1(input: String) -> usize {
        let target = read_input(input);
        let mut max_vy: isize = isize::MIN;
        for (vx_initial, vy_initial) in (get_min_vx(&target)..target.max_x + 1).cartesian_product(0..target.min_y.abs()) {
            if vy_initial > max_vy && Probe::new(vx_initial, vy_initial).hits(&target) {
                max_vy = vy_initial;
            }

        }

        ((max_vy + 1) * max_vy / 2) as usize
    }

    fn _part_2(input: String) -> usize {
        let target = read_input(input);
        let mut n_valid_paths: usize = 0;
        for (vx_initial, vy_initial) in (get_min_vx(&target)..target.max_x + 1).cartesian_product(-target.min_y.abs()..target.min_y.abs()) {
            if Probe::new(vx_initial, vy_initial).hits(&target) {
                n_valid_paths += 1
            }
        }
        n_valid_paths
    }

    pub fn part_1(mut aoc_reader: AocBufReader) -> usize {
        _part_1(aoc_reader.next().unwrap())
    }

    pub fn part_2(mut aoc_reader: AocBufReader) -> usize {
        _part_2(aoc_reader.next().unwrap())
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


        #[test]
        fn test_pt_1() {
            // println!("{}", _part_1("target area: x=155..182, y=-117..-67".to_string()));
        }


        #[test]
        fn test_pt_2() {
            println!("{}", _part_2("target area: x=155..182, y=-117..-67".to_string()));
        }
    }
}