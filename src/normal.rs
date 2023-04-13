#[derive(Debug, Default)]
pub struct NormalizedRect {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
    pub orig_x1: i32,
    pub orig_y1: i32,
    pub orig_x2: i32,
    pub orig_y2: i32,
}

macro_rules! swap {
    ($a:expr, $b:expr) => {
        let tmp = $a;
        $a = $b;
        $b = tmp;
    };
}

/// The point of this function is to produce two ranges `x1..=x2` and `y1..=y2`
///  that are guaranteed to be safe to iterate over the canvas of size
/// `canvas_width` by `canvas_height` without any boundary checks.
///
/// ```
/// use farba::{Canvas, normalize_rect};
///
/// let canvas = Canvas::new(400, 400);
///
/// if let Some(nr) = normalize_rect(
///     -10,
///     -10,
///     30,
///     40,
///     canvas.get_width() as i32,
///     canvas.get_height() as i32,
/// ) {
///     // (x1, y1) through (x2, y2) are all safely iterable
///     // on the canvas
///     assert_eq!(nr.x1, 0);
///     assert_eq!(nr.x2, 19);
///     assert_eq!(nr.y1, 0);
///     assert_eq!(nr.y2, 29);
/// } else {
///     // Rectangle is invisible cause it's completely out-of-bounds
/// }
pub fn normalize_rect(
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    canvas_width: i32,
    canvas_height: i32,
) -> Option<NormalizedRect> {
    // No need to render an empty rectangle
    if width == 0 || height == 0 {
        return None;
    }

    let mut nr = NormalizedRect::default();

    // Store the original values of x and y
    nr.orig_x1 = x;
    nr.orig_y1 = y;

    // Convert the rectangle from 1-point w,h representation to 2-point representation
    nr.orig_x2 = nr.orig_x1 + width.signum() * (width.abs() - 1);
    if nr.orig_x1 > nr.orig_x2 {
        swap!(nr.orig_x1, nr.orig_x2);
    }
    nr.orig_y2 = nr.orig_y1 + height.signum() * (height.abs() - 1);
    if nr.orig_y1 > nr.orig_y2 {
        swap!(nr.orig_y1, nr.orig_y2);
    }

    // Cull out invisible rectangle since we know x1 <= x2 and y1 <= y2
    if nr.orig_x1 >= canvas_width || nr.orig_x2 < 0 || nr.orig_y1 >= canvas_height || nr.orig_y2 < 0
    {
        return None;
    }

    nr.x1 = nr.orig_x1;
    nr.x2 = nr.orig_x2;
    nr.y1 = nr.orig_y1;
    nr.y2 = nr.orig_y2;

    // Clamp the rectangle to the boundaries
    if nr.x1 < 0 {
        nr.x1 = 0;
    }
    if nr.x2 >= canvas_width {
        nr.x2 = canvas_width - 1;
    }
    if nr.y1 < 0 {
        nr.y1 = 0;
    }
    if nr.y2 >= canvas_height {
        nr.y2 = canvas_height - 1;
    }

    return Some(nr);
}

#[derive(Debug, Default)]
pub struct NormalizedTriangle {
    pub left_x: i32,
    pub right_x: i32,
    pub top_y: i32,
    pub bottom_y: i32,
}

/// The point of this function is to produce two ranges `left_x..=right_x` and
///  `top_y..=bottom_y` that are guaranteed to be safe to iterate over the
/// canvas of size `width` by `height` without any boundary checks.
pub fn normalize_triangle(
    width: usize,
    height: usize,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    x3: i32,
    y3: i32,
) -> Option<NormalizedTriangle> {
    let mut nt = NormalizedTriangle::default();

    // Normalize the x bounds of the triangle
    nt.left_x = x1;
    nt.right_x = x1;

    if nt.left_x > x2 {
        nt.left_x = x2;
    }
    if nt.left_x > x3 {
        nt.left_x = x3;
    }
    if nt.right_x < x2 {
        nt.right_x = x2;
    }
    if nt.right_x < x3 {
        nt.right_x = x3;
    }

    // Clamp x bounds to canvas
    if nt.left_x < 0 {
        nt.left_x = 0;
    }
    if nt.left_x as usize >= width {
        return None;
    }
    if nt.right_x < 0 {
        return None;
    }
    if nt.right_x as usize >= width {
        nt.right_x = (width - 1) as i32;
    }

    // Normalize the y bounds of the triangle
    nt.top_y = y1;
    nt.bottom_y = y1;

    if nt.top_y > y2 {
        nt.top_y = y2;
    }
    if nt.top_y > y3 {
        nt.top_y = y3;
    }
    if nt.bottom_y < y2 {
        nt.bottom_y = y2;
    }
    if nt.bottom_y < y3 {
        nt.bottom_y = y3;
    }

    // Clamp y bounds to canvas
    if nt.top_y < 0 {
        nt.top_y = 0;
    }
    if nt.top_y as usize >= height {
        return None;
    }
    if nt.bottom_y < 0 {
        return None;
    }
    if nt.bottom_y as usize >= height {
        nt.bottom_y = (height - 1) as i32
    }

    Some(nt)
}
