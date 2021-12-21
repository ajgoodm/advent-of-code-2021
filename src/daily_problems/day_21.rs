pub mod solutions {
    use lazy_static::lazy_static;
    use regex::Regex;

    use crate::AocBufReader;

    lazy_static! {
        static ref INPUT_RE: Regex = Regex::new(
            r"^Player ([0-9]*) starting position: ([0-9]*)$"
        ).unwrap();
    }


    struct Player {
        id: usize,
        position: usize
    }


    impl Player {
        fn new(id: usize, position: usize) -> Player {
            Player { id, position }
        }

        fn from_reader(aoc_reader: AocBufReader) -> Vec<Player> {
            let mut players: Vec<Player> = Vec::new();
            for line in aoc_reader {
                let capture = INPUT_RE.captures(&line).unwrap();
                players.push(
                    Player::new(
                        capture.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                        capture.get(2).unwrap().as_str().parse::<usize>().unwrap()
                    )
                );
            }
            players
        }
    }


    fn _part_1(players: Vec<Player>) -> usize {
        1
    }


    pub fn part_1(aoc_reader: AocBufReader) -> usize {
        let players = Player::from_reader(aoc_reader);
        _part_1(players)
    }
}