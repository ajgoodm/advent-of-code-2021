pub mod solutions {
    use std::cmp::{max, min};
    use std::collections::{HashMap, HashSet};

    use lazy_static::lazy_static;

    use crate::AocBufReader;

    lazy_static! {
        static ref ALL_SPACES: HashSet<Point> = vec![
            Point::new(0, 0),
            Point::new(1, 0),
            Point::new(2, 0),
            Point::new(2, 1),
            Point::new(2, 2),
            Point::new(3, 0),
            Point::new(4, 0),
            Point::new(4, 1),
            Point::new(4, 2),
            Point::new(5, 0),
            Point::new(6, 0),
            Point::new(6, 1),
            Point::new(6, 2),
            Point::new(7, 0),
            Point::new(8, 0),
            Point::new(8, 1),
            Point::new(8, 2),
            Point::new(9, 0),
            Point::new(10, 0),
        ].into_iter().collect();

        static ref SPACES_OUTSIDE_ROOMS: HashSet<Point> = vec![
            Point::new(2, 0),
            Point::new(4, 0),
            Point::new(6, 0),
            Point::new(8, 1),
        ].into_iter().collect();

        static ref ROOMS_FOR_AMPHIPODS: HashMap<AmphipodColor, HashSet<Point>> = vec![
            (AmphipodColor::A, vec![Point::new(2, 2), Point::new(2, 2)].into_iter().collect()),
            (AmphipodColor::B, vec![Point::new(4, 2), Point::new(4, 2)].into_iter().collect()),
            (AmphipodColor::C, vec![Point::new(6, 2), Point::new(6, 2)].into_iter().collect()),
            (AmphipodColor::D, vec![Point::new(8, 2), Point::new(8, 2)].into_iter().collect()),
        ].into_iter().collect();

        static ref ROOM_NUMBERS: HashMap<AmphipodColor, usize> = vec![
            (AmphipodColor::A, 2),
            (AmphipodColor::B, 4),
            (AmphipodColor::C, 6),
            (AmphipodColor::D, 8),
        ].into_iter().collect();
    }


    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct Point {
        x: usize,
        y: usize
    }


    impl Point {
        fn new(x: usize, y: usize) -> Point {
            Point { x, y }
        }
    }


    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum AmphipodColor {
        A,
        B,
        C,
        D
    }


    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct Amphipod {
        color: AmphipodColor,
        location: Point,
        must_move: bool,
        can_move: bool
    }


    impl Amphipod {
        fn new(color: AmphipodColor, location: Point) -> Amphipod {
            Amphipod { color, location, must_move: false, can_move: true }
        }

        fn next_space_candidates(&self) -> Vec<Point> {
            let north: Option<Point> = match self.location.y > 0 {
                true => Some(Point::new(self.location.x, self.location.y - 1)),
                false => None
            };
            let west: Option<Point> = match self.location.x > 0 {
                true => Some(Point::new(self.location.x - 1, self.location.y)),
                false => None
            };
            let east = Some(Point::new(self.location.x + 1, self.location.y));
            let south = Some(Point::new(self.location.x, self.location.y + 1));
            vec![north, east, south, west].into_iter().filter(|x| *x != None).map(|p| p.unwrap()).collect()
        }
    }


    #[derive(Debug, PartialEq, Eq, Clone)]
    struct GameState {
        amphipods: HashSet<Amphipod>,
        cost: usize,
    }


    impl GameState {
        fn from_vec(
            A: Vec<Point>, B: Vec<Point>,
            C: Vec<Point>, D: Vec<Point>,
            cost: usize
        )  -> GameState {
            GameState {
                amphipods: vec![
                    Amphipod::new(AmphipodColor::A, A[0]),
                    Amphipod::new(AmphipodColor::A, A[1]),
                    Amphipod::new(AmphipodColor::B, B[0]),
                    Amphipod::new(AmphipodColor::B, B[1]),
                    Amphipod::new(AmphipodColor::C, C[0]),
                    Amphipod::new(AmphipodColor::C, C[1]),
                    Amphipod::new(AmphipodColor::D, D[0]),
                    Amphipod::new(AmphipodColor::D, D[1])
                ].into_iter().collect(),
                cost: cost
            }
        }

        fn space_occupant(&self, space: &Point) -> Option<&Amphipod> {
            let amphipods_in_space: Vec<&Amphipod> =  self.amphipods.iter().filter(|a| a.location == *space).collect();
            match amphipods_in_space.len() {
                0 => None,
                1 => Some(&amphipods_in_space[0]),
                _ => panic!("Too many amphipods in this space! Get a room!")
            }
        }

        fn space_is_occupied_by_another_color(&self, amphipod: &Amphipod, space: &Point) -> bool {
            match self.space_occupant(space) {
                Some(other) => other.color != amphipod.color,
                None => false
            }
        }

        fn get_occupied_spaces(&self) -> HashSet<Point> {
            self.amphipods.iter().map(|a| a.location).collect()
        }

        fn get_unoccupied_spaces(&self) -> HashSet<Point> {
            let all_spaces: HashSet<Point> = ALL_SPACES.iter().map(|p| *p).collect();
            all_spaces.into_iter().filter(|p| !self.get_occupied_spaces().contains(&p)).collect()
        }

        fn can_amphipod_move_to_space(&self, amphipod: &Amphipod, candidate: &Point) -> bool {
            if let Some(occupant) = self.space_occupant(candidate) {
                return false
            }

            for (room_color, room_spaces) in ROOMS_FOR_AMPHIPODS.iter() {
                if room_spaces.contains(candidate) {
                    if *room_color != amphipod.color {
                        // can only move in another's room if it's already there
                        return room_spaces.contains(&amphipod.location)
                    } else {
                        // can only enter its own room if it's not occupied by another color
                        return !room_spaces.iter().any(|space| self.space_is_occupied_by_another_color(amphipod, space))
                    }
                }
            }
            // it's unoccupied hallway
            true
        }

        fn this_amphipods_room_is_occupied_by_other(&self, amphipod: &Amphipod) -> bool {
            ROOMS_FOR_AMPHIPODS.get(&amphipod.color).unwrap().iter().any(
                |space| self.space_is_occupied_by_another_color(amphipod, space)
            )
        }

        fn can_this_amphipod_go_home(&self, amphipod: &Amphipod) -> bool {
            if self.this_amphipods_room_is_occupied_by_other(amphipod) {
                return false
            }

            let min_x: usize = min(*ROOM_NUMBERS.get(&amphipod.color).unwrap(), amphipod.location.x);
            let max_x: usize = max(*ROOM_NUMBERS.get(&amphipod.color).unwrap(), amphipod.location.x);

            !(min_x..=max_x).any(|x| self.space_occupant(&Point::new(x, 0)) != None)
        }

        fn this_amphipods_home(&self, amphipod: &Amphipod) -> Point {
            let first_spot = Point::new(*ROOM_NUMBERS.get(&amphipod.color).unwrap(),2);
            let second_spot = Point::new(*ROOM_NUMBERS.get(&amphipod.color).unwrap(),1);
            if self.space_occupant(&first_spot) != None {
                second_spot
            } else {
                first_spot
            }
        }

        fn get_neighbors(&self) -> Vec<GameState> {
            let mut neighbors: Vec<GameState> = vec![];
            // If any amphipod _must_ move; move it!
            let must_move: Vec<&Amphipod> = self.amphipods.iter().filter(|x| x.must_move).collect();
            // match must_move.len() {
            //     0 => (),
            //     1 => {
            //         let amphipod = must_move[0];
            //         return amphipod.next_space_candidates().into_iter()
            //             .filter(|p| self.can_amphipod_move_to_space(amphipod, p))
            //             .map(|p| {
            //                 // let mut result = self.clone();
            //                 // result.amphipods
            //             })
            //             // .collect()
            //     },
            //     _ => panic!("More than one amphipod must move!")
            // }


            // if any amphipod can go home from this state, move them there.

            // append all other legal moves

            neighbors
        }
    }


    fn part_1_test_input() -> GameState {
        GameState::from_vec(
            vec![Point::new(2, 2), Point::new(8, 2)],
            vec![Point::new(2, 1), Point::new(6, 1)],
            vec![Point::new(4, 1), Point::new(6, 2)],
            vec![Point::new(4, 2), Point::new(8, 1)],
            0
        )
    }


    pub fn part_1() -> usize {
        let start = part_1_test_input();
        1
    }


    #[cfg(test)]
    mod tests {
        use super::*;

        fn test_occupied() {
            let start = part_1_test_input();
            assert_eq!(start.get_occupied_spaces().len(), 8);
            assert_eq!(start.get_unoccupied_spaces().len(), 11);
        }

    }
}