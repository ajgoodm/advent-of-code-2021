pub mod solutions {
    use std::cmp::max;
    use std::collections::HashMap;

    use lazy_static::lazy_static;
    use regex::Regex;

    use crate::AocBufReader;

    lazy_static! {
        static ref INPUT_RE: Regex = Regex::new(
            r"^Player ([0-9]*) starting position: ([0-9]*)$"
        ).unwrap();

        static ref DIRAC_DIE_THREE_ROLL_SUMS: Vec<usize> = vec![
            (1 + 1 + 1),  // all rolls with three 1s
            (1 + 1 + 2),  // all rolls with two 1s
            (1 + 2 + 1),
            (2 + 1 + 1),
            (1 + 1 + 3),
            (1 + 3 + 1),
            (3 + 1 + 1),
            (1 + 2 + 2),  // all rolls with one 1
            (2 + 1 + 2),
            (2 + 2 + 1),
            (1 + 2 + 3),
            (1 + 3 + 2),
            (2 + 1 + 3),
            (3 + 1 + 2),
            (2 + 3 + 1),
            (3 + 2 + 1),
            (1 + 3 + 3),
            (3 + 1 + 3),
            (3 + 3 + 1),
            (2 + 2 + 2),  // all rolls with zero 1s
            (2 + 2 + 3),
            (2 + 3 + 2),
            (3 + 2 + 2),
            (2 + 3 + 3),
            (3 + 2 + 3),
            (3 + 3 + 2),
            (3 + 3 + 3),
        ];

        static ref DIRAC_DIE_ROLL_FREQUENCY: HashMap<usize, usize> = vec![
            (3, 1),
            (4, 3),
            (5, 6),
            (6, 7),
            (7, 6),
            (8, 3),
            (9, 1)
        ].into_iter().collect();

        static ref PART_2_WINNING_SCORE: usize = 21;
    }


    struct DeterministicDie {
        next: usize,
        n_rolls: usize,
    }


    impl DeterministicDie {
        fn new() -> DeterministicDie {
            DeterministicDie { next: 1, n_rolls: 0 }
        }

        fn roll(&mut self) -> usize {
            let previous_val = self.next;
            if previous_val == 100 {
                self.next = 1;
            } else {
                self.next = previous_val + 1;
            }
            self.n_rolls += 1;
            previous_val
        }

        fn roll_n_times(&mut self, n: usize) -> usize {
            let mut sum: usize = 0;
            for _ in 0..n {
                sum += self.roll();
            }
            sum
        }
    }


    #[derive(Debug, Clone, Copy)]
    struct Player {
        id: usize,
        position: usize,
        score: usize
    }


    impl Player {
        fn new(id: usize, position: usize) -> Player {
            Player { id, position, score: 0 }
        }

        fn take_turn(&mut self, mut die: &mut DeterministicDie) {
            let n_spaces_to_move = die.roll_n_times(3);
            self.move_n_spaces(n_spaces_to_move);
        }

        fn move_n_spaces(&mut self, n: usize) {
            let mut next_position = self.position + n;
            while next_position > 10 {
                next_position = next_position - 10;
            }
            self.position = next_position;
            self.score += next_position;
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


    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    struct GameState {
        player_1_position: usize,
        player_1_score: usize,
        player_2_position: usize,
        player_2_score: usize
    }


    impl GameState {
        fn new(
            player_1_position: usize, player_1_score: usize,
            player_2_position: usize, player_2_score: usize
        ) -> GameState {
            GameState { player_1_position, player_1_score, player_2_position, player_2_score }
        }

        fn player_1_roll(&self, roll: usize) -> GameState {
            let mut player_1_next_position: usize = self.player_1_position + roll;
            while player_1_next_position > 10 {
                player_1_next_position -= 10;
            }

            GameState {
                player_1_position: player_1_next_position,
                player_1_score: self.player_1_score + player_1_next_position,
                player_2_position: self.player_2_position,
                player_2_score: self.player_2_score
            }
        }

        fn player_1_wins(&self) -> bool {
            self.player_1_score >= *PART_2_WINNING_SCORE
        }

        fn player_2_roll(&self, roll: usize) -> GameState {
            let mut player_2_next_position: usize = self.player_2_position + roll;
            while player_2_next_position > 10 {
                player_2_next_position -= 10;
            }

            GameState {
                player_1_position: self.player_1_position,
                player_1_score: self.player_1_score,
                player_2_position: player_2_next_position,
                player_2_score: self.player_2_score + player_2_next_position
            }
        }

        fn player_2_wins(&self) -> bool {
            self.player_2_score >= *PART_2_WINNING_SCORE
        }
    }


    struct AllUniverses {
        game_states: HashMap<GameState, usize>,
        player_1_wins: usize,
        player_2_wins: usize,
    }


    impl AllUniverses {
        fn from_reader(aoc_reader: AocBufReader) -> AllUniverses {
            let players = Player::from_reader(aoc_reader);
            assert_eq!(players.len(), 2);
            let player_1 = players[0];
            let player_2 = players[1];

            let game_states: HashMap<GameState, usize> = vec![
                (GameState::new(player_1.position, player_1.score, player_2.position, player_2.score), 1)
            ].into_iter().collect();

            AllUniverses { game_states: game_states, player_1_wins: 0, player_2_wins: 0 }
        }

        fn _update_game_states(&mut self, game_state_updates: HashMap<GameState, isize>) {
            for (game_state, update) in game_state_updates {
                if update > 0 {
                    *self.game_states.entry(game_state).or_insert(0) += update as usize;
                } else if update < 0 {
                    *self.game_states.entry(game_state).or_insert(0) -= (update * -1) as usize;
                }
            }
        }

        fn take_player_1_turn(&mut self) {
            let mut game_state_updates: HashMap<GameState, isize> = HashMap::new();

            for (previous_game_state, cts_previous_state) in self.game_states.iter() {
                *game_state_updates.entry(*previous_game_state).or_insert(0) -= *cts_previous_state as isize; // we're moving all of these game states to new states.
                for (roll, frequency) in DIRAC_DIE_ROLL_FREQUENCY.iter() {
                    let next_game_state = previous_game_state.player_1_roll(*roll);
                    if next_game_state.player_1_wins() {
                        self.player_1_wins += cts_previous_state * frequency;
                    } else {
                        *game_state_updates.entry(next_game_state).or_insert(0) += (cts_previous_state * frequency) as isize;
                    }
                }
            }
            self._update_game_states(game_state_updates);
        }

        fn take_player_2_turn(&mut self) {
            let mut game_state_updates: HashMap<GameState, isize> = HashMap::new();

            for (previous_game_state, cts_previous_state) in self.game_states.iter() {
                *game_state_updates.entry(*previous_game_state).or_insert(0) -= *cts_previous_state as isize; // we're moving all of these game states to new states.
                for (roll, frequency) in DIRAC_DIE_ROLL_FREQUENCY.iter() {
                    let next_game_state = previous_game_state.player_2_roll(*roll);
                    if next_game_state.player_2_wins() {
                        self.player_2_wins += cts_previous_state * frequency;
                    } else {
                        *game_state_updates.entry(next_game_state).or_insert(0) += (cts_previous_state * frequency) as isize;
                    }
                }
            }
            self._update_game_states(game_state_updates);
        }

        fn all_games_won(&self) -> bool {
            self.game_states.iter().all(|(game_state, cts)| *cts == 0)
        }
    }


    fn _part_1(players: Vec<Player>) -> usize {
        assert_eq!(players.len(), 2);
        let mut player_1 = players[0];
        let mut player_2 = players[1];
        let mut die= DeterministicDie::new();

        loop {
            player_1.take_turn(&mut die);
            if player_1.score >= 1000 {
                return player_2.score * die.n_rolls
            }
            player_2.take_turn(&mut die);
            if player_2.score >= 1000 {
                return player_1.score * die.n_rolls
            }
        }
    }


    pub fn part_1(aoc_reader: AocBufReader) -> usize {
        let players = Player::from_reader(aoc_reader);
        _part_1(players)
    }


    fn _part_2(mut all_universes: AllUniverses) -> usize {
        while !all_universes.all_games_won() {
            all_universes.take_player_1_turn();
            all_universes.take_player_2_turn();
        }
        max(all_universes.player_1_wins, all_universes.player_2_wins)
    }


    pub fn part_2(aoc_reader: AocBufReader) -> usize {
        let all_universes = AllUniverses::from_reader(aoc_reader);
        _part_2(all_universes)
    }


    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_static_mappings() {
            let mut roll_frequency: HashMap<usize, usize> = HashMap::new();
            for roll in DIRAC_DIE_THREE_ROLL_SUMS.iter() {
                *roll_frequency.entry(*roll).or_insert(0) += 1;
            }

            for (roll, frequency) in roll_frequency {
                assert_eq!(*DIRAC_DIE_ROLL_FREQUENCY.get(&roll).unwrap(), frequency);
            }
        }
    }
}