use crate::{normalize_rect, Color};

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
    pub fn get_data(&self) -> &[u8] {
        use std::mem::size_of;

        unsafe {
            std::slice::from_raw_parts(
                self.pixels.as_ptr() as *const u8,
                size_of::<u32>() * self.pixels.len(),
            )
        }
    }

    /// Gets a mutable slice over the raw pixel buffer owned by the canvas
    pub fn get_data_mut(&mut self) -> &mut [u8] {
        use std::mem::size_of;

        unsafe {
            std::slice::from_raw_parts_mut(
                self.pixels.as_mut_ptr() as *mut u8,
                size_of::<u32>() * self.pixels.len(),
            )
        }
    }

    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32
    }

    /// Performs a bounds check on the coordinates to ensure they are within
    /// the canvas before setting the pixel. If the coordinates are not inside
    /// the canvas, then nothing is changed
    pub fn set_pixel<C: Color>(&mut self, x: i32, y: i32, color: C) {
        let pixel_color = color.pack();

        if self.in_bounds(x, y) {
            *self.get_pixel_mut(x, y) = pixel_color;
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

        *self.get_pixel_mut(x, y) = pixel_color;
    }

    #[inline]
    pub fn get_index(&self, x: i32, y: i32) -> usize {
        self.width * y as usize + x as usize
    }

    #[inline]
    pub fn get_pixel(&self, x: i32, y: i32) -> &u32 {
        let index = self.get_index(x, y);
        &self.pixels[index]
    }

    #[inline]
    pub fn get_pixel_mut(&mut self, x: i32, y: i32) -> &mut u32 {
        let index = self.get_index(x, y);
        &mut self.pixels[index]
    }

    #[cfg(feature = "image")]
    pub fn save_to_file(&self, file_path: &str) {
        use image::{save_buffer, ColorType};

        // TODO: Return Result instead of expecting

        save_buffer(
            file_path,
            self.get_data(),
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
                *self.get_pixel_mut(x as i32, y as i32) = pixel_color;
            }
        }
    }

    /// Draws a circle at the provided center with the given radius
    pub fn circle<C: Color>(&mut self, center_x: i32, center_y: i32, radius: i32, color: C) {
        // TODO: Anti-Aliasing

        let pixel_color = color.pack();

        // Clip the rectangle to the canvas
        let Some(nr) = normalize_rect(center_x - radius, center_y - radius, radius * 2, radius * 2, self.width as i32, self.height as i32) else {
            // Nothing to render
            return;
        };

        // Iterate over the clipped bounding box of the circle
        for x in nr.x1..=nr.x2 {
            for y in nr.y1..=nr.y2 {
                // Calculate the current point's distance from the center of the circle
                let dx = center_x - x;
                let dy = center_y - y;

                // If the point satisfies the equation for a circle then fill in that
                // pixel with the provided color
                if dx * dx + dy * dy < radius * radius {
                    *self.get_pixel_mut(x, y) = pixel_color;
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
    pub fn rect<C: Color>(&mut self, x: i32, y: i32, width: i32, height: i32, color: C) {
        let pixel_color = color.pack();

        let Some(nr) = normalize_rect(x, y, width, height, self.width as i32, self.height as i32) else {
            // Nothing to render
            return;
        };

        // Iterate through the clipped bounding box of the rect and fill in all the pixels
        for x in nr.x1..=nr.x2 {
            for y in nr.y1..=nr.y2 {
                *self.get_pixel_mut(x, y) = pixel_color;
            }
        }
    }
}
