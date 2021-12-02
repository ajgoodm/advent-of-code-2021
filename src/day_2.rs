pub mod day_2 {
    use std::fs::DirEntry;

    use crate::input::read_input::AocBufReader;


    enum Direction {
        Forward,
        Up,
        Down,
    }


    struct Submarine {
        x: isize,
        depth: isize,
    }


    impl Submarine {
        fn new() -> Submarine {
            Submarine {
                x: 0, depth: 0
            }
        }

        fn execute_instruction(&mut self, instruction: (Direction, isize)) {
            let (direction, distance) = instruction;
            match direction {
                Direction::Forward => self.x += distance,
                Direction::Up => self.depth -= distance,
                Direction::Down => self.depth += distance,
            }
        }
    }


    fn parse_instruction(line: String) -> (Direction, isize) {
        let parts: Vec<&str> = line.split(" ").collect();
        let direction: Direction = match parts[0] {
            "forward" => Direction::Forward,
            "down" => Direction::Down,
            "up" => Direction::Up,
            _ => panic!("Unknown direction!")
        };
        (direction, parts[1].parse::<isize>().unwrap())
    }


    fn read_input(aoc_reader: AocBufReader) -> Vec<(Direction, isize)> {
        aoc_reader.map(
            |line| {parse_instruction(line)}
        ).collect::<Vec<(Direction, isize)>>()
    }


    pub fn part_1(aoc_reader: AocBufReader) -> isize {
        let instructions = read_input(aoc_reader);
        let mut submarine = Submarine::new();
        for instruction in instructions {
            submarine.execute_instruction(instruction);
        }
        
        submarine.x.abs() * submarine.depth.abs()
    }


}