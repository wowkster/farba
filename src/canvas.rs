use core::panic;

use crate::{normalize_rect, normalize_triangle, Color, Vec3};

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
    pub fn get_pixels(&self) -> &[u32] {
        self.pixels.as_slice()
    }

    /// Gets a slice over the raw pixel buffer owned by the canvas
    pub fn get_pixels_mut(&mut self) -> &mut [u32] {
        self.pixels.as_mut_slice()
    }

    /// Gets a slice over the raw pixel buffer owned by the canvas but as bytes
    pub fn get_data(&self) -> &[u8] {
        use std::mem::size_of;

        unsafe {
            std::slice::from_raw_parts(
                self.pixels.as_ptr() as *const u8,
                size_of::<u32>() * self.pixels.len(),
            )
        }
    }

    /// Gets a mutable slice over the raw pixel buffer owned by the canvas but as bytes
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

    /// Draws a triangle with the provided coordinates as vertices
    ///
    /// Vertices may be supplied in any order as they are normalized before drawing
    pub fn triangle<C: Color>(
        &mut self,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        x3: i32,
        y3: i32,
        color: C,
    ) {
        // TODO: Anti-Aliasing

        let pixel_color = color.pack();

        let Some(nt) = normalize_triangle(self.width, self.height, x1, y1, x2, y2, x3, y3) else {
            return;
        };

        let point_in_bounds = |x: i32, y: i32| {
            // Check (v1, v2)
            let z1 = (x2 - x1) * (y - y1) - (y2 - y1) * (x - x1);
            // Check (v2, v3)
            let z2 = (x3 - x2) * (y - y2) - (y3 - y2) * (x - x2);
            // Check (v3, v1)
            let z3 = (x1 - x3) * (y - y3) - (y1 - y3) * (x - x3);

            z1.signum() >= 0 && z2.signum() >= 0 && z3.signum() >= 0
        };

        for x in nt.left_x..=nt.right_x {
            for y in nt.top_y..=nt.bottom_y {
                if point_in_bounds(x, y) {
                    *self.get_pixel_mut(x, y) = pixel_color;
                }
            }
        }
    }

    /// Draws a triangle with the provided coordinates as vertices
    ///
    /// Vertices may be supplied in any order as they are normalized before drawing
    pub fn triangle_with_depth_buffer<C: Color>(
        &mut self,
        v1: Vec3,
        v2: Vec3,
        v3: Vec3,
        color: C,
        depth_buffer: &mut Vec<f32>,
    ) {
        // TODO: Anti-Aliasing

        let pixel_color = color.pack();

        let x1 = v1.x as i32;
        let y1 = v1.y as i32;
        let x2 = v2.x as i32;
        let y2 = v2.y as i32;
        let x3 = v3.x as i32;
        let y3 = v3.y as i32;

        let Some(nt) = normalize_triangle(self.width, self.height, x1, y1, x2, y2, x3, y3) else {
            return;
        };

        let point_in_bounds = |x: i32, y: i32| {
            // Check (v1, v2)
            let z1 = (x2 - x1) * (y - y1) - (y2 - y1) * (x - x1);
            // Check (v2, v3)
            let z2 = (x3 - x2) * (y - y2) - (y3 - y2) * (x - x2);
            // Check (v3, v1)
            let z3 = (x1 - x3) * (y - y3) - (y1 - y3) * (x - x3);

            z1.signum() >= 0 && z2.signum() >= 0 && z3.signum() >= 0
        };

        if depth_buffer.len() != self.width * self.height {
            panic!("Depth buffer was not correct size to match canvas")
        }

        // Here we calculate the z value of the pixel on the plane defined by the 3 points
        // Shamelessly stolen from https://math.stackexchange.com/questions/28043/finding-the-z-value-on-a-plane-with-x-y-values

        // Plane has equation rx+sy+tz=k
        let plane_v1 = v1 - v2;
        let plane_v2 = v1 - v3;

        // (r, s, t) vector
        let plane_normal = Vec3::cross(&plane_v1, &plane_v2);

        // Solve for k
        let k = Vec3::dot(&v1, &plane_normal);

        // Pull out variables
        let Vec3 { x: r, y: s, z: t } = plane_normal;

        // Closure that computes the z value for each pixel and tells us if we
        // should draw there based on the depth buffer

        let width = self.width; // Required for borrow checker :/

        let mut pixel_is_nearer = |x: i32, y: i32| {
            let z = (1.0 / t) * (k - r * x as f32 - s * y as f32);

            let index = width * y as usize + x as usize;

            let should_draw = z < depth_buffer[index];

            if should_draw {
                depth_buffer[index] = z;
            }

            should_draw
        };

        for x in nt.left_x..=nt.right_x {
            for y in nt.top_y..=nt.bottom_y {
                if point_in_bounds(x, y) && pixel_is_nearer(x, y) {
                    *self.get_pixel_mut(x, y) = pixel_color;
                }
            }
        }
    }
}
