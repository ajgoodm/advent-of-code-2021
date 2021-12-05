pub mod solutions {
    use std::collections::HashSet;

    use crate::input::read_input::AocBufReader;

    struct BingoBoard {
        id: usize,
        board: [[usize; 5]; 5],
        called: [[bool; 5]; 5]
    }


    impl BingoBoard {
        fn new(id: usize, board: [[usize; 5]; 5]) -> BingoBoard {
            BingoBoard {
                id: id, board: board, called: [[false; 5]; 5]
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


    fn parse_board(id: usize, aoc_reader: &mut AocBufReader) -> BingoBoard {
        let mut board: [[usize; 5]; 5] = [[0usize; 5]; 5];
        for row_idx in 0usize..5 {
            let data: Vec<usize> = aoc_reader.next().unwrap()
                .split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect();
            
            for col_idx in 0usize..5 {
                board[row_idx][col_idx] = data[col_idx]
            }
        }

        BingoBoard::new(id, board)
    }


    fn read_input(mut aoc_reader: AocBufReader) -> (Vec<usize>, Vec<BingoBoard>) {
        let numbers: Vec<usize> = aoc_reader.next().unwrap()
            .split(",").map(|x| x.parse::<usize>().unwrap()).collect();

        let mut boards: Vec<BingoBoard> = vec![];
        let mut board_id: usize = 0;
        while let Some(_) = aoc_reader.next() {
            boards.push(parse_board(board_id, &mut aoc_reader));
            board_id += 1;
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
        let mut winning_board_ids: HashSet<usize> = HashSet::new();

        let mut last_winning_return_value: Option<usize> = None;
        for number in numbers {
            for (board_id, board) in boards.iter_mut().enumerate() {
                if !winning_board_ids.contains(&board_id) {
                    board.mark_number(number);
                    if board.won() {
                        winning_board_ids.insert(board_id);
                        last_winning_return_value = Some(board.sum_unmarked() * number)
                    }
                }
            }
        }

        last_winning_return_value.unwrap()
    }
}
