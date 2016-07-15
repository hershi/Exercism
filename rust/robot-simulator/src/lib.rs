use std::ops::Add;


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

struct Position {
    x : isize,
    y : isize,
}

impl Add<(isize,isize)> for Position {
    type Output = Position;

    fn add(self, rhs : (isize,isize)) -> Position {
        Position {x: self.x + rhs.0, y: self.y + rhs.1}
    }
}

pub struct Robot {
    direction : Direction,
    position : Position,
}

impl Robot {
    pub fn new(x: isize, y: isize, d: Direction) -> Self {
        Robot{direction : d, position : Position{x : x, y: y}}
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
        let movement = MOVEMENTS[self.direction.clone() as usize];
        Robot{direction : self.direction, position: self.position + movement}
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
        (self.position.x, self.position.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
