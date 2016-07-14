// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    direction : Direction,
    position : (isize, isize),
}

impl Robot {
    pub fn new(x: isize, y: isize, d: Direction) -> Self {
        Robot{direction : d, position : (x,y)}
    }

    pub fn turn_right(self) -> Self {
        let new_direction = match self.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North
        };
        
        Robot{direction : new_direction, position : self.position}
    }

    pub fn turn_left(self) -> Self {
        let new_direction = match self.direction {
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
            Direction::North => Direction::West 
        };

        Robot{direction : new_direction, position : self.position}
    }

    pub fn advance(self) -> Self {
        let (x,y) = match self.direction {
            Direction::East => (1,0),
            Direction::South => (0,-1),
            Direction::West => (-1,0),
            Direction::North => (0,1) 
        };

        let (a,b) = self.position;
        Robot{direction : self.direction, position : (x + a, y + b)}
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |robot, c| {
            match c {
                'R' => robot.turn_right(),
                'L' => robot.turn_left(),
                'A' => robot.advance(),
                _ => robot
            }
        })
    }

    pub fn position(&self) -> (isize, isize) {
        self.position
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
