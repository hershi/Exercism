#[derive(Debug, PartialEq)]
enum TriangleType {
    Isosceles,
    Equilateral,
    Scalene
}

pub struct Triangle {
    triangle_type : TriangleType,
}

impl Triangle {
    pub fn build(sides : [u32; 3]) -> Result<Triangle, String> {
        if !is_valid_triangle(&sides) {
            Err("Invalid Triangle".to_string())
        }
        else if sides.iter().skip(1).all(|&x| x == sides[0]) {
            Ok(Triangle { triangle_type : TriangleType::Equilateral })
        }
        else if sides[0] == sides[1] || sides[1] == sides[2] || sides[2] == sides[0] {
            Ok(Triangle { triangle_type : TriangleType::Isosceles })
        }
        else {
            Ok(Triangle { triangle_type : TriangleType::Scalene })
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.triangle_type == TriangleType::Equilateral
    }
    pub fn is_isosceles(&self) -> bool { 
        self.triangle_type == TriangleType::Isosceles
    }
    pub fn is_scalene(&self) -> bool {
        self.triangle_type == TriangleType::Scalene
    }

}

fn is_valid_triangle(sides : &[u32; 3]) -> bool {
    sides[0] + sides[1] >= sides[2] &&
    sides[1] + sides[2] >= sides[0] &&
    sides[2] + sides[0] >= sides[1] &&
    sides.iter().all(|&x| x != 0)
}
