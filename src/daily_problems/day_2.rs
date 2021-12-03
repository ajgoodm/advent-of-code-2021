pub mod solutions {
    use crate::input::read_input::AocBufReader;


    enum Direction {
        Forward,
        Up,
        Down,
    }


    struct Submarine {
        x: isize,
        depth: isize,
        aim: isize,
    }


    impl Submarine {
        fn new() -> Submarine {
            Submarine {
                x: 0, depth: 0, aim: 0
            }
        }

        fn execute_part_1_instruction(&mut self, instruction: (Direction, isize)) {
            let (direction, distance) = instruction;
            match direction {
                Direction::Forward => self.x += distance,
                Direction::Up => self.depth -= distance,
                Direction::Down => self.depth += distance,
            }
        }

        fn execute_part_2_instruction(&mut self, instruction: (Direction, isize)) {
            let (direction, scalar) = instruction;
            match direction {
                Direction::Up => self.aim -= scalar,
                Direction::Down => self.aim += scalar,
                Direction::Forward => {
                    self.x += scalar;
                    self.depth += self.aim * scalar;
                }
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
            submarine.execute_part_1_instruction(instruction);
        }
        
        submarine.x.abs() * submarine.depth.abs()
    }


    pub fn part_2(aoc_reader: AocBufReader) -> isize {
        let instructions = read_input(aoc_reader);
        let mut submarine = Submarine::new();
        for instruction in instructions {
            submarine.execute_part_2_instruction(instruction);
        }
        
        submarine.x.abs() * submarine.depth.abs()
    }


}