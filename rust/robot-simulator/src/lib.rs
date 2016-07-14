// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

// Ordered list of directions, in clockwise ordering
// Relies on having the same ordering as the enum variants for Direction, because
// we use the ordinal of the enum variants to index into the array
// I'll attempt to create a macro that takes in the variant names and creates the enum
// as well as the corresponding array (in a later iteration)
const DIRECTIONS : [Direction; 4] = [Direction::North, Direction::East, Direction::South, Direction::West];

// This array has the corresponding movements effect for each direction
// Similar to the above, this has correspondence to the order of directios int he enum
// specification (first element corresponds to the first direction variant, etc.)
const MOVEMENTS : [(isize,isize);4] = [(0,1),(1,0),(0,-1),(-1,0)];

pub struct Robot {
    direction : Direction,
    position : (isize, isize),
}

impl Robot {
    pub fn new(x: isize, y: isize, d: Direction) -> Self {
        Robot{direction : d, position : (x,y)}
    }

    pub fn turn_right(self) -> Self {
        let index = ((self.direction as usize) + 1) % DIRECTIONS.len();
        let new_direction = DIRECTIONS[index].clone();
        
        Robot{direction : new_direction, position : self.position}
    }

    pub fn turn_left(self) -> Self {
        let index = ((self.direction as usize) + 3) % DIRECTIONS.len();
        let new_direction = DIRECTIONS[index].clone();

        Robot{direction : new_direction, position : self.position}
    }

    pub fn advance(self) -> Self {
        let (x,y) = MOVEMENTS[self.direction.clone() as usize];
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
