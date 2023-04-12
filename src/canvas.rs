use crate::Color;

#[derive(Debug, PartialEq)]
pub struct Canvas {
    pixels: Vec<u32>,
    width: usize,
    height: usize,
}

impl Canvas {
    /// Creates a new Canvas with the specified width and height
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            pixels: vec![0u32; width * height],
            width,
            height,
        }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    /// Allows you to take ownership of the underlying pixel buffer
    pub fn take(self) -> Vec<u32> {
        self.pixels
    }

    /// Gets a slice over the raw pixel buffer owned by the canvas
    pub fn data(&self) -> &[u8] {
        use std::mem::size_of;

        unsafe {
            std::slice::from_raw_parts(
                self.pixels.as_ptr() as *const u8,
                size_of::<u32>() * self.pixels.len(),
            )
        }
    }

    /// Gets a mutable slice over the raw pixel buffer owned by the canvas
    pub fn data_mut(&mut self) -> &mut [u8] {
        use std::mem::size_of;

        unsafe {
            std::slice::from_raw_parts_mut(
                self.pixels.as_mut_ptr() as *mut u8,
                size_of::<u32>() * self.pixels.len(),
            )
        }
    }

    /// Performs a bounds check on the coordinates to ensure they are within
    /// the canvas before setting the pixel. If the coordinates are not inside
    /// the canvas, then nothing is changed
    pub fn set_pixel<C: Color>(&mut self, x: i32, y: i32, color: C) {
        let pixel_color = color.pack();

        let index = self.height as i32 * y + x;

        if index > 0 && index < self.pixels.len() as i32 {
            self.pixels[index as usize] = pixel_color;
        }
    }

    /// Calculates an index into the pixel buffer and tries to directly access
    /// it to set the color of the pixel.
    ///
    /// `(x, y)` must be a valid coordinate within the canvas or else `set_pixel_unchecked`
    /// will panic
    #[inline]
    pub fn set_pixel_unchecked<C: Color>(&mut self, x: i32, y: i32, color: C) {
        let pixel_color = color.pack();

        let index = (self.height as i32 * y + x) as usize;

        self.pixels[index] = pixel_color;
    }

    #[inline]
    fn get_index(&self, x: i32, y: i32) -> usize {
        self.width * y as usize + x as usize
    }

    #[cfg(feature = "image")]
    pub fn save_to_file(&self, file_path: &str) {
        use image::{save_buffer, ColorType};

        // TODO: Return Result instead of expecting

        save_buffer(
            file_path,
            self.data(),
            self.get_width() as u32,
            self.get_height() as u32,
            ColorType::Rgba8,
        )
        .expect("could not save image");
    }

    /// Completely fills the canvas with the specified color
    pub fn fill<C: Color>(&mut self, color: C) {
        let pixel_color = color.pack();

        for y in 0..self.height {
            for x in 0..self.width {
                let index = self.get_index(x as i32, y as i32);

                self.pixels[index] = pixel_color;
            }
        }
    }

    /// Draws a circle at the provided center with the given radius
    pub fn circle<C: Color>(&mut self, center_x: i32, center_y: i32, radius: i32, color: C) {
        // TODO: Optimize by first clipping to the canvas and then iterating the bounded rect
        // TODO: Anti-Aliasing

        let pixel_color = color.pack();

        // Iterate over the bounding box of the circle
        for x in center_x - radius..center_x + radius {
            for y in center_y - radius..center_y + radius {
                // Don't draw pixels outside the screen
                if x < 0 || x > self.width as i32 || y < 0 || y > self.height as i32 {
                    continue;
                }

                // Calculate the current point's distance from the center of the circle
                let dx = center_x - x;
                let dy = center_y - y;

                // If the point satisfies the equation for a circle then fill in that
                // pixel with the provided color
                if dx * dx + dy * dy < radius * radius {
                    let index = self.get_index(x, y);

                    self.pixels[index] = pixel_color;
                }
            }
        }
    }

    /// Draws a rectangle at the provided coordinates with the given width and height
    ///
    /// If width is positive, x will be the left bound of the rectangle, and if it is
    /// negative, then x will be the right bound of the rect
    ///
    /// The same logic follows for height where when height is positive, y will be the
    /// top bound of the rectangle, and when height is negative, y will be the bottom
    /// bound of the rect
    pub fn rect<C: Color>(
        &mut self,
        mut x: i32,
        mut y: i32,
        mut width: i32,
        mut height: i32,
        color: C,
    ) {
        // TODO: Optimize by first clipping to the canvas and then iterating the bounded rect

        let pixel_color = color.pack();

        // If width is negative, flip the sign and move x over so that it is always
        // the left bound when drawing
        if width.signum() < 0 {
            width *= -1;
            x -= width;
        }

        // If height is negative, flip the sign and move y up so that it is always
        // the top bound when drawing
        if height.signum() < 0 {
            height *= -1;
            y -= height
        }

        // Iterate through the bounding box of the rect and fill in all the pixels
        for x in x..x + width {
            for y in y..y + height {
                // Don't draw pixels outside the screen
                if x < 0 || x > self.width as i32 || y < 0 || y > self.height as i32 {
                    continue;
                }

                let index = self.get_index(x, y);

                self.pixels[index] = pixel_color;
            }
        }
    }
}
