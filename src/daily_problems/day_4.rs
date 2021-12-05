pub mod solutions {
    use crate::input::read_input::AocBufReader;

    struct BingoBoard {
        board: [[usize; 5]; 5],
        called: [[bool; 5]; 5]
    }


    impl BingoBoard {
        fn new(board: [[usize; 5]; 5]) -> BingoBoard {
            BingoBoard {
                board: board, called: [[false; 5]; 5]
            }
        }

        fn mark_number(&mut self, number: usize) {
            for row_idx in 0usize..5 {
                for col_idx in 0usize..5 {
                    if self.board[row_idx][col_idx] == number {
                        self.called[row_idx][col_idx] = true
                    }
                }
            }
        }

        fn won(&self) -> bool {
            for row_idx in 0usize..5 {
                if self.called[row_idx][..].iter().all(|x| *x) {
                    return true
                }
            }

            for col_idx in 0usize..5 {
                if self.called[..].iter().map(|row| row[col_idx]).all(|x| x) {
                    return true
                }
            }

            false
        }

        fn sum_unmarked(&self) -> usize {
            let mut sum: usize = 0;
            for row_idx in 0usize..5 {
                for col_idx in 0usize..5 {
                    if !self.called[row_idx][col_idx] {
                        sum += self.board[row_idx][col_idx]
                    }
                }
            }

            sum
        }
    }


    fn parse_board(aoc_reader: &mut AocBufReader) -> BingoBoard {
        let mut board: [[usize; 5]; 5] = [[0usize; 5]; 5];
        for row_idx in 0usize..5 {
            let data: Vec<usize> = aoc_reader.next().unwrap()
                .split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect();
            
            for col_idx in 0usize..5 {
                board[row_idx][col_idx] = data[col_idx]
            }
        }

        BingoBoard::new(board)
    }


    fn read_input(mut aoc_reader: AocBufReader) -> (Vec<usize>, Vec<BingoBoard>) {
        let numbers: Vec<usize> = aoc_reader.next().unwrap()
            .split(",").map(|x| x.parse::<usize>().unwrap()).collect();

        let mut boards: Vec<BingoBoard> = vec![];
        while let Some(_) = aoc_reader.next() {
            boards.push(parse_board(&mut aoc_reader));
        }

        (numbers, boards)
    }


    pub fn part_1(aoc_reader: AocBufReader) -> usize {
        let (numbers, mut boards): (Vec<usize>, Vec<BingoBoard>) = read_input(aoc_reader);
        for number in numbers {
            for board in boards.iter_mut() {
                board.mark_number(number);
                if board.won() {
                    return number * board.sum_unmarked()
                }
            }
        }

        panic!("no board won!");
    }


    pub fn part_2(aoc_reader: AocBufReader) -> usize {
        let (numbers, mut boards): (Vec<usize>, Vec<BingoBoard>) = read_input(aoc_reader);
        let mut board_took_mth_place_with_val: Vec<Option<(usize,usize)>> = vec![None; boards.len()];

        let mut mth_place: usize = 1;
        for number in numbers {
            for (nth_board, board) in boards.iter_mut().enumerate() {
                match board_took_mth_place_with_val[nth_board] {
                    Some(_) => (),  // this board already won, don't keep marking it
                    None => {
                        board.mark_number(number);
                        if board.won() {
                            board_took_mth_place_with_val[nth_board] = Some((mth_place, number));
                            mth_place += 1;
                        }
                    }
                }
            }
        }

        let mut last_place_board_index: Option<usize> = None;
        let mut last_place: Option<usize> = None;
        for (board_idx, place_and_val) in board_took_mth_place_with_val.iter().enumerate() {
            match place_and_val {
                Some((place, _)) => {
                    match last_place {
                        Some(previous_last_place) => if *place > previous_last_place {
                            last_place = Some(*place);
                            last_place_board_index = Some(board_idx)
                        },
                        None => last_place = Some(*place)
                    }
                },
                None => ()
            }
        }

        match last_place_board_index {
            Some(idx) => {
                let (_, final_val) = board_took_mth_place_with_val[idx].unwrap();
                return boards[idx].sum_unmarked() * final_val
            },
            None => panic!("No board won?")
        }
    }
}
