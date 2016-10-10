pub struct PascalsTriangle{
    triangle : Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: usize) -> Self {
        let mut triangle = Vec::with_capacity(row_count);

        if row_count == 0 { return PascalsTriangle{triangle : triangle}; }
        
        // At least one row - push the first row to the triangle
        triangle.push(vec![1]);

        // For the remainnig rows, generate each row based on the previous row
        for i in 1..row_count {
            let new_line = PascalsTriangle::generate_row(&triangle[(i - 1)]);
            triangle.push(new_line);
        }

        PascalsTriangle{triangle : triangle}
    }

    fn generate_row(prev_row: &[u32]) -> Vec<u32> {
        // Each row except for the first, is made up of a 1 at each side and
        // in between them, the sum of each pair of numbers from the previous
        // row (this sum technique doesn't work for the 1's at the sides, since
        // they are a sum of only a single element)
        std::iter::once(1)
            .chain( prev_row.windows(2).map(|w| w.iter().sum()) )
            .chain(std::iter::once(1))
            .collect()
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        // Since rows is a non-mutating function, we need to clone the rows before
        // returning
        self.triangle.clone()
    }
}
