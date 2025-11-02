#[derive(Clone, Copy)]
pub struct Dimensions {
    pub y: usize,
    pub x: usize,
}

impl Dimensions {
    /// The dimensions define the number of dots on a field
    pub fn from(x: usize, y: usize) -> Self {
        Self { y, x }
    }

    pub fn for_sticks_vertical(self) -> Self {
        Self {
            y: self.y -1,
            x: self.x,
        }
    }

    pub fn for_sticks_horizontal(self) -> Self {
        Self {
            y: self.y,
            x: self.x - 1,
        }
    }

    pub fn for_boxes(self) -> Self {
        Self {
            y: self.y - 1,
            x: self.x - 1,
        }
    }
}
