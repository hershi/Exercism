pub struct PascalsTriangle{
    triangle : Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut triangle = Vec::with_capacity(row_count as usize);

        if row_count == 0 { return PascalsTriangle{triangle : triangle}; }
        
        triangle.push(vec![1]);

        for i in 2..row_count + 1 {
            let new_line = PascalsTriangle::generate_row(&triangle[(i - 2) as usize]);
            triangle.push(new_line);
        }

        PascalsTriangle{triangle : triangle}
    }

    fn generate_row(prev_row: &[u32]) -> Vec<u32> {
        // Pre-allocate the correct size
        let mut row = Vec::with_capacity(prev_row.len() + 1);

        // Each row except for the first, is made up of a 1 at each side and
        // in between them, the sum of each pair of numbers from the previous
        // row (this sum technique doesn't work for the 1's at the sides, since
        // they are a sum of only a single element)
        row.push(1);
        for pair in prev_row.windows(2) {
            row.push(pair[0] + pair[1]);
        }
        row.push(1);

        row
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        // Since rows is a non-mutating function, we need to clone the rows before
        // returning
        self.triangle.clone()
    }
}
