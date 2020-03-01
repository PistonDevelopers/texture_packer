/// Defines a rectangle in pixels with the origin at the top-left of the texture atlas.
#[derive(Copy, Clone, Debug)]
pub struct Rect {
    /// Horizontal position the rectangle begins at.
    pub x: u32,
    /// Vertical position the rectangle begins at.
    pub y: u32,
    /// Width of the rectangle.
    pub w: u32,
    /// Height of the rectangle.
    pub h: u32,
}

impl Rect {
    /// Create a new [Rect] based on a position and its width and height.
    pub fn new(x: u32, y: u32, w: u32, h: u32) -> Rect {
        Rect { x, y, w, h }
    }

    /// Create a new [Rect] based on two points spanning the rectangle.
    pub fn new_with_points(x1: u32, y1: u32, x2: u32, y2: u32) -> Rect {
        Rect {
            x: x1,
            y: y1,
            w: x2 - x1 + 1,
            h: y2 - y1 + 1,
        }
    }

    /// Get the top coordinate of the rectangle.
    #[inline(always)]
    pub fn top(&self) -> u32 {
        self.y
    }

    /// Get the bottom coordinate of the rectangle.
    #[inline(always)]
    pub fn bottom(&self) -> u32 {
        self.y + self.h - 1
    }

    /// Get the left coordinate of the rectangle.
    #[inline(always)]
    pub fn left(&self) -> u32 {
        self.x
    }

    /// Get the right coordinate of the rectangle.
    #[inline(always)]
    pub fn right(&self) -> u32 {
        self.x + self.w - 1
    }

    /// Get the area of the rectangle.
    #[inline(always)]
    pub fn area(&self) -> u32 {
        self.w * self.h
    }

    /// Check if this rectangle intersects with another.
    pub fn intersects(&self, other: &Rect) -> bool {
        self.left() < other.right()
            && self.right() > other.left()
            && self.top() < other.bottom()
            && self.bottom() > other.top()
    }

    /// Check if this rectangle contains another.
    pub fn contains(&self, other: &Rect) -> bool {
        self.left() <= other.left()
            && self.right() >= other.right()
            && self.top() <= other.top()
            && self.bottom() >= other.bottom()
    }

    /// Check if this rectangle contains a point. Includes the edges of the rectangle.
    pub fn contains_point(&self, x: u32, y: u32) -> bool {
        self.left() <= x && self.right() >= x && self.top() <= y && self.bottom() >= y
    }

    /// Check if a point falls on the rectangle's boundaries.
    pub fn is_outline(&self, x: u32, y: u32) -> bool {
        x == self.left() || x == self.right() || y == self.top() || y == self.bottom()
    }

    /// Split two rectangles into non-overlapping regions.
    pub fn crop(&self, other: &Rect) -> Vec<Rect> {
        if !self.intersects(other) {
            return vec![*self];
        }

        let inside_x1 = if other.left() < self.left() {
            self.left()
        } else {
            other.left()
        };

        let inside_y1 = if other.top() < self.top() {
            self.top()
        } else {
            other.top()
        };

        let inside_x2 = if other.right() > self.right() {
            self.right()
        } else {
            other.right()
        };

        let inside_y2 = if other.bottom() > self.bottom() {
            self.bottom()
        } else {
            other.bottom()
        };

        //
        // *******************
        // *    | r3  |      *
        // *    |     |      *
        // *    +++++++      *
        // * r1 +     +      *
        // *    +     +  r2  *
        // *    +++++++      *
        // *    |     |      *
        // *    | r4  |      *
        // *******************
        //
        let mut result = Vec::new();

        let r1 = Rect::new_with_points(self.left(), self.top(), inside_x1, self.bottom());
        if r1.area() > 0 {
            result.push(r1);
        }

        let r2 = Rect::new_with_points(inside_x2, self.top(), self.right(), self.bottom());
        if r2.area() > 0 {
            result.push(r2);
        }

        let r3 = Rect::new_with_points(inside_x1, self.top(), inside_x2, inside_y1);
        if r3.area() > 0 {
            result.push(r3);
        }

        let r4 = Rect::new_with_points(inside_x1, inside_y2, inside_x2, self.bottom());
        if r4.area() > 0 {
            result.push(r4);
        }

        result
    }
}
