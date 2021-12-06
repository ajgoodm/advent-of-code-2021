pub mod solutions {
    use std::{collections::{HashMap, HashSet}, alloc::Layout};

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


    fn read_input_pt1(mut aoc_reader: AocBufReader) -> Vec<LanternFish> {
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
        let lantern_fish: Vec<LanternFish> = read_input_pt1(aoc_reader);
        run_simulation(lantern_fish, 80)
    }

    struct LanternFishPopulation {
        subpopulations: HashMap<usize, usize>
    }


    impl LanternFishPopulation {
        fn new() -> LanternFishPopulation {
            let mut sub_populations: HashMap<usize, usize> = HashMap::new();
            for timer in 0..=8 {
                sub_populations.insert(timer, 0);
            }  

            LanternFishPopulation {subpopulations: sub_populations}
        }

        fn add_n_lantern_fishes(&mut self, timer: usize, n: usize) {
            *self.subpopulations.entry(timer).or_insert(0) += n;
        }

        fn add_latern_fish(&mut self, timer: usize) {
            self.add_n_lantern_fishes(timer, 1);
        }

        fn advance_simulation_one_day(&mut self) {
            let mut incoming_population_flux: usize = 0;
            let mut outgoing_population_flux: usize;
            for timer in (0usize..=8).rev() {
                outgoing_population_flux = *self.subpopulations.get(&timer).unwrap();
                self.subpopulations.insert(timer, incoming_population_flux);

                // for the next population, the outgoing will become the incoming
                incoming_population_flux = outgoing_population_flux;
            }

            // the zero population rolls over to 6
            self.add_n_lantern_fishes(6, incoming_population_flux);
            // the zero population has babies
            self.add_n_lantern_fishes(8, incoming_population_flux);
        }

        fn total_population(&self) -> usize {
            self.subpopulations.values().sum()
        }
    }


    fn read_input_pt2(mut aoc_reader: AocBufReader) -> LanternFishPopulation {
        let timers: Vec<usize> =aoc_reader.next().unwrap().split(",").map(
            |usize_str| usize_str.parse::<usize>().unwrap()
        ).collect();
        let mut lantern_fish_population = LanternFishPopulation::new();
        for timer in timers {
            lantern_fish_population.add_latern_fish(timer)
        }
        lantern_fish_population
    }


    pub fn part_2(aoc_reader: AocBufReader) -> usize {
        let mut lantern_fish_population = read_input_pt2(aoc_reader);
        for _ in 0..256 {
            lantern_fish_population.advance_simulation_one_day();
        }
        lantern_fish_population.total_population()
    }
}