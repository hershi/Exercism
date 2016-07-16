use std::ops::Sub;

pub struct ChessPosition {
    x : i8,
    y : i8
}

const BOARD_SIZE : i8 = 7;

impl ChessPosition {
    pub fn new(x : i8, y :i8) -> Result<ChessPosition, &'static str> {
        if x < 0 || y < 0 || x > BOARD_SIZE || y > BOARD_SIZE {
            Err("Invalid Position")
        }
        else {
            Ok(ChessPosition { x : x, y : y })
        }
    }
}

// Implementing this over a reference rather than passing ownership is
// (I'm assming) not really contributing to any performance improvement sicne
// the value is only 16 bits (theoretically it's even less performant, since the
// reference would need 32/64-bit, but with alignment and/or compiler optimizations
// it's hard to say for sure without RE-ing the resulting assembly)
//
// The reason I insisted on doing this for a reference was to learn how to do it,
// since the vanilla implementation takes ownership of the object, which:
// 1. Seemed odd/surprising to me (since you don't expect substraction to change its
// operands
// 2. Requires cloningthe operands, which is less performant if they are big structs
impl <'a> Sub<&'a ChessPosition> for &'a ChessPosition {
    type Output = (i8, i8);
    fn sub(self, rhs : &ChessPosition) -> Self::Output {
        (self.x - rhs.x, self.y - rhs.y)
    }
}

pub struct Queen {
    position : ChessPosition,
}

impl Queen {
    pub fn new(pos : ChessPosition) -> Queen {
        Queen { position : pos }
    }

    pub fn can_attack(&self, other : &Queen) -> bool {
        let (x_diff, y_diff) = &self.position - &other.position;
        x_diff == 0 || y_diff == 0 || x_diff.abs() == y_diff.abs() 
    }
}