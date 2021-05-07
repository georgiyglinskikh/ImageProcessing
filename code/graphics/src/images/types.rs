pub struct Position {
    pub x: isize,
    pub y: isize,
}

pub struct Dimension {
    pub width: isize,
    pub height: isize,
}

impl Dimension {
    pub fn fit(&self, inner: Space) -> Space {
        let width = if inner.position.x + inner.size.height >= self.width {
            self.width - inner.position.x
        } else if inner.position.x < 0 {
            inner.size.height + inner.position.x
        } else {
            inner.size.height
        };

        let height = if inner.position.y + inner.size.height >= self.height {
            self.height - inner.position.y
        } else if inner.position.y < 0 {
            inner.size.height + inner.position.y
        } else {
            inner.size.height
        };

        let x = if inner.position.x < 0 {
            0
        } else {
            inner.position.x
        };

        let y = if inner.position.y < 0 {
            0
        } else {
            inner.position.y
        };

        Space::new(x, y, width, height)
    }
}

pub struct Space {
    pub position: Position,
    pub size: Dimension,
}

impl Default for Space {
    fn default() -> Self {
        Space::new(0, 0, 0, 0)
    }
}

impl Space {
    pub fn new(x: isize, y: isize, width: isize, height: isize) -> Self {
        Space {
            position: Position { x, y },
            size: Dimension { width, height },
        }
    }
}
