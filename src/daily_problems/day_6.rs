pub mod solutions {
    use crate::AocBufReader;

    struct LanternFish {
        timer: usize,
        is_baby: bool,
    }


    impl LanternFish {
        fn new(timer: usize) -> LanternFish {
            LanternFish {timer: timer, is_baby: true}
        }
    }


    fn read_input(mut aoc_reader: AocBufReader) -> Vec<LanternFish> {
        aoc_reader.next().unwrap().split(",").map(
            |usize_str| usize_str.parse::<usize>().unwrap()
        ).map(
            |timer| LanternFish::new(timer)
        ).collect()
    }


    fn run_simulation(mut lantern_fish: Vec<LanternFish>, n_days: usize) -> usize {
        let mut n_new_fish: usize;
        for _ in 0..n_days {
            n_new_fish = 0;
            for fish in lantern_fish.iter_mut() {
                if fish.timer == 0 {
                    n_new_fish += 1;
                    fish.timer = 6;
                    fish.is_baby = false;
                } else {
                    fish.timer -= 1;
                }
            }
            for _ in 0..n_new_fish {
                lantern_fish.push(LanternFish::new(8));
            }
        }
        lantern_fish.len()
    }

    pub fn part_1(aoc_reader: AocBufReader) -> usize {
        let lantern_fish: Vec<LanternFish> = read_input(aoc_reader);
        run_simulation(lantern_fish, 80)
    }

    pub fn part_2(aoc_reader: AocBufReader) -> usize {
        let lantern_fish: Vec<LanternFish> = read_input(aoc_reader);
        run_simulation(lantern_fish, 256)
    }
}