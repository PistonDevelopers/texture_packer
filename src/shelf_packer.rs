
use {
    Buffer2d,
    Rect,
    Packer,
};

pub struct ShelfPacker<'a, B: 'a + Buffer2d> {
    buf: B,
    width: u32,
    height: u32,
    x: u32,
    y: u32,
    opening_shelf_max_y: u32,
    margin: u32,
}

impl<'a, B: Buffer2d> ShelfPacker<'a, B> {
    pub fn new(mut buf: B) -> ShelfPacker<'a, B> {
        let (w, h) = buf.dimensions();
        ShelfPacker {
            buf: buf,
            width: w,
            height: h,
            x: 0,
            y: 0,
            opening_shelf_max_y: 0,
            margin: 0,
        }
    }

}

impl<'a, B: Buffer2d> Packer<B> for ShelfPacker<'a, B> {
    fn pack(&mut self, buf: &Buffer2d) -> Option<Rect> {
        let (mut buf_width, mut buf_height) = buf.dimensions();
        buf_width += self.margin;
        buf_height += self.margin;
        let mut patched_width = buf_width;
        let mut patched_height = buf_height;

        // If the rectangle is the first rectangle on a new open shelf,
        // store it sideways. This is to minimize the height of the new shelf.
        if self.x == 0 {
            if buf_height > buf_width && buf_height <= self.width {
                patched_width = buf_height;
                patched_height = buf_width;
            }
        }

        // If the rectangle fits upright then store it so.
        // This aims to minimize the wasted surface area between the
        // rectangle top side and the shelf ceiling.
        //
        // Otherwise store the rectangle sideways if possible.
        else {
            if buf_width > buf_height && self.x + buf_height <= self.width {
                patched_width = buf_height;
                patched_height = buf_width;
            } else if self.x + buf_width > self.width {
                // Open a new shelf
                self.x = 0;
                self.y += self.opening_shelf_max_y;
                self.opening_shelf_max_y = 0;
            }
        }

        if self.x + patched_width <= self.width && self.y + patched_height <= self.height {
            if patched_width == buf_width {
                self.buf.patch(self.x, self.y, buf);
            } else {
                self.buf.patch_rotated(self.x, self.y, buf);
            }

            self.x += patched_width;
            if self.opening_shelf_max_y < patched_height {
                self.opening_shelf_max_y = patched_height;
            }

            Some(Rect::new(self.x, self.y,
                           patched_width - self.margin,
                           patched_height - self.margin))
        } else {
            None
        }
    }

    fn buf(&self) -> &B {
        &self.buf
    }

    fn set_margin(&mut self, val: u32) {
        self.margin = val;
    }
}

