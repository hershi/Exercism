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
    facing : Direction,
    location : (isize, isize),
}

impl Robot {
    pub fn new(x: isize, y: isize, d: Direction) -> Self {
        Robot{facing : d, location : (x,y)}
    }

    pub fn turn_right(self) -> Self {
        let new_direction = match self.facing {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North
        };
        
        Robot{facing : new_direction, location : self.location}
    }

    pub fn turn_left(self) -> Self {
        let new_direction = match self.facing {
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
            Direction::North => Direction::West 
        };

        Robot{facing : new_direction, location : self.location}
    }

    pub fn advance(self) -> Self {
        let (x,y) = match self.facing {
            Direction::East => (1,0),
            Direction::South => (0,-1),
            Direction::West => (-1,0),
            Direction::North => (0,1) 
        };

        let new_location = (self.location.0 + x, self.location.1 + y);
        Robot{facing : self.facing, location : new_location}
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
        self.location
    }

    pub fn direction(&self) -> &Direction {
        &self.facing
    }
}
