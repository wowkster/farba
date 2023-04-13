// Demonstration of 3d rendering using triangles and depth buffering
// https://machinethink.net/blog/3d-rendering-without-shaders/
// https://paroj.github.io/gltut/Basics/Intro%20Graphics%20and%20Rendering.html
// https://erkaman.github.io/posts/fast_triangle_rasterization.html
// https://www.scratchapixel.com/lessons/3d-basic-rendering/perspective-and-orthographic-projection-matrix/projection-matrix-GPU-rendering-pipeline-clipping.html

#![allow(unused)]

use farba::{Canvas, Mat3, RGBAColor, Vec3};

const CANVAS_WIDTH: usize = 400;
const CANVAS_HEIGHT: usize = 400;

fn main() {
    let canvas = Canvas::new(CANVAS_WIDTH, CANVAS_HEIGHT);
    let model = Model::create_cube();
    let camera = Camera::new();

    #[cfg(feature = "image")]
    render_frame_sequence(canvas, model, camera);

    #[cfg(feature = "window")]
    render_window(canvas, model, camera);

    #[cfg(any(
        not(any(feature = "image", feature = "window")),
        all(feature = "image", feature = "window")
    ))]
    compile_error!(
        "For this example, enable either the \"image\" feature to render a png sequence \
         or the \"window\" feature to render an animated window (not both)"
    );
}

#[cfg(feature = "image")]
fn render_frame_sequence(mut canvas: Canvas, mut model: Model, camera: Camera) {
    std::fs::create_dir_all("./examples/3d_cube").expect("Could not create directory");

    for t in 0..180 {
        render_frame(t as f32, &mut canvas, &mut model, &camera);

        canvas.save_to_file(&format!("./examples/3d_cube/{t}.png"));
    }
}

#[cfg(feature = "window")]
fn render_window(mut canvas: Canvas, mut model: Model, camera: Camera) {
    use farba::Color;
    use minifb::{Key, Window, WindowOptions};

    let mut window = Window::new(
        "3D Cube Example - ESC to exit",
        CANVAS_WIDTH,
        CANVAS_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut t = 0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        render_frame(t as f32, &mut canvas, &mut model, &camera);

        // minifb uses a weird ARGB ordering instead of the standard ABGR ordering
        let pixels: Vec<u32> = canvas
            .get_pixels()
            .iter()
            .map(|pixel| {
                ((pixel.blue() as u32 & 0xFF) << (8 * 0))
                    | ((pixel.green() as u32 & 0xFF) << (8 * 1))
                    | ((pixel.red() as u32 & 0xFF) << (8 * 2))
                    | ((pixel.alpha() as u32 & 0xFF) << (8 * 3))
            })
            .collect();

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&pixels, CANVAS_WIDTH, CANVAS_HEIGHT)
            .unwrap();

        t += 1
    }
}

#[derive(Debug, Clone)]
struct Triangle3d {
    vertices: [Vec3; 3],
    normal: Vec3,
    color: RGBAColor,
}

#[derive(Debug, Clone)]
struct Model {
    triangles: Vec<Triangle3d>,
    origin: Vec3,
    position: Vec3,
    scale: Vec3,
    rotation: Vec3,
}

impl Model {
    /// Creates a cube mesh by manually defining every single individual vertex
    fn create_cube() -> Model {
        Model {
            triangles: vec![
                // Face 1
                Triangle3d {
                    vertices: [
                        Vec3::new(-1.0, 1.0, -1.0),
                        Vec3::new(1.0, -1.0, -1.0),
                        Vec3::new(-1.0, -1.0, -1.0),
                    ],
                    normal: Vec3::new(0.0, 0.0, -1.0),
                    color: RGBAColor::CYAN,
                },
                Triangle3d {
                    vertices: [
                        Vec3::new(-1.0, 1.0, -1.0),
                        Vec3::new(1.0, 1.0, -1.0),
                        Vec3::new(1.0, -1.0, -1.0),
                    ],
                    normal: Vec3::new(0.0, 0.0, -1.0),
                    color: RGBAColor::CYAN,
                },
                // Face 2
                Triangle3d {
                    vertices: [
                        Vec3::new(1.0, 1.0, -1.0),
                        Vec3::new(1.0, -1.0, 1.0),
                        Vec3::new(1.0, -1.0, -1.0),
                    ],
                    normal: Vec3::new(1.0, 0.0, 0.0),
                    color: RGBAColor::RED,
                },
                Triangle3d {
                    vertices: [
                        Vec3::new(1.0, 1.0, -1.0),
                        Vec3::new(1.0, 1.0, 1.0),
                        Vec3::new(1.0, -1.0, 1.0),
                    ],
                    normal: Vec3::new(1.0, 0.0, 0.0),
                    color: RGBAColor::RED,
                },
                // Face 3
                Triangle3d {
                    vertices: [
                        Vec3::new(1.0, 1.0, 1.0),
                        Vec3::new(-1.0, -1.0, 1.0),
                        Vec3::new(1.0, -1.0, 1.0),
                    ],
                    normal: Vec3::new(0.0, 0.0, 1.0),
                    color: RGBAColor::BLUE,
                },
                Triangle3d {
                    vertices: [
                        Vec3::new(1.0, 1.0, 1.0),
                        Vec3::new(-1.0, 1.0, 1.0),
                        Vec3::new(-1.0, -1.0, 1.0),
                    ],
                    normal: Vec3::new(0.0, 0.0, 1.0),
                    color: RGBAColor::BLUE,
                },
                // Face 4
                Triangle3d {
                    vertices: [
                        Vec3::new(-1.0, 1.0, 1.0),
                        Vec3::new(-1.0, -1.0, -1.0),
                        Vec3::new(-1.0, -1.0, 1.0),
                    ],
                    normal: Vec3::new(-1.0, 0.0, 0.0),
                    color: RGBAColor::MAGENTA,
                },
                Triangle3d {
                    vertices: [
                        Vec3::new(-1.0, 1.0, 1.0),
                        Vec3::new(-1.0, 1.0, -1.0),
                        Vec3::new(-1.0, -1.0, -1.0),
                    ],
                    normal: Vec3::new(-1.0, 0.0, 0.0),
                    color: RGBAColor::MAGENTA,
                },
                // Face 5
                Triangle3d {
                    vertices: [
                        Vec3::new(1.0, 1.0, 1.0),
                        Vec3::new(-1.0, 1.0, -1.0),
                        Vec3::new(-1.0, 1.0, 1.0),
                    ],
                    normal: Vec3::new(0.0, 1.0, 0.0),
                    color: RGBAColor::GREEN,
                },
                Triangle3d {
                    vertices: [
                        Vec3::new(1.0, 1.0, 1.0),
                        Vec3::new(1.0, 1.0, -1.0),
                        Vec3::new(-1.0, 1.0, -1.0),
                    ],
                    normal: Vec3::new(0.0, 1.0, 0.0),
                    color: RGBAColor::GREEN,
                },
                // Face 6
                Triangle3d {
                    vertices: [
                        Vec3::new(1.0, -1.0, 1.0),
                        Vec3::new(-1.0, -1.0, 1.0),
                        Vec3::new(-1.0, -1.0, -1.0),
                    ],
                    normal: Vec3::new(0.0, -1.0, 0.0),
                    color: RGBAColor::YELLOW,
                },
                Triangle3d {
                    vertices: [
                        Vec3::new(1.0, -1.0, 1.0),
                        Vec3::new(-1.0, -1.0, -1.0),
                        Vec3::new(1.0, -1.0, -1.0),
                    ],
                    normal: Vec3::new(0.0, -1.0, 0.0),
                    color: RGBAColor::YELLOW,
                },
            ],
            origin: Vec3::ZERO,
            position: Vec3::ZERO,
            scale: Vec3::new(1.0, 1.0, 1.0),
            rotation: Vec3::new(0.0, 0.0, 0.0),
        }
    }
}

struct Camera {
    position: Vec3,
    rotation: Vec3,
    look_at: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            position: Vec3::new(0.0, 1.0, -2.0),
            rotation: Vec3::new(15f32.to_radians(), 0.0, 0.0),
            look_at: Vec3::new(0.0, 0.0, 1.0),
        }
    }
}

fn render_frame(t: f32, canvas: &mut Canvas, model: &mut Model, camera: &Camera) {
    canvas.fill(RGBAColor::from_rgb(200, 200, 200));

    model.rotation.y = (t * 4.0).to_radians();
    model.rotation.x = (t * 2.0).to_radians();
    model.rotation.z = (t * 1.0).to_radians();

    let projected_triangles = transform_and_project(model, camera);

    let mut depth_buffer: Vec<f32> = vec![f32::INFINITY; CANVAS_WIDTH * CANVAS_HEIGHT];

    for triangle in projected_triangles {
        canvas.triangle_with_depth_buffer(
            triangle.vertices[0],
            triangle.vertices[1],
            triangle.vertices[2],
            triangle.color,
            &mut depth_buffer,
        )
    }
}

fn transform_and_project(model: &Model, camera: &Camera) -> Vec<Triangle3d> {
    let mut triangles = model.triangles.clone();

    let rotation_matrix = Mat3::rotate_z(model.rotation.z)
        * Mat3::rotate_y(model.rotation.y)
        * Mat3::rotate_x(model.rotation.x);

    // Convert triangles to world space
    triangles.iter_mut().for_each(|triangle| {
        // Rotate the normal vector
        // triangle.normal = rotation_matrix * triangle.normal;

        // Apply transformations to the vertices
        triangle.vertices.iter_mut().for_each(|vertex| {
            // Move center of model to origin (if origin is not already (0, 0, 0))
            *vertex -= model.origin;

            // Multiply vertex (component-wise) by the model scale
            *vertex *= model.scale;

            // Multiply the vertex by the model's rotation matrix
            *vertex = rotation_matrix * *vertex;

            // Move the vertex from local coordinates to the model's world coordinates
            *vertex += model.position;
        });
    });

    let camera_rotation_matrix = Mat3::rotate_z(-camera.rotation.z)
        * Mat3::rotate_y(-camera.rotation.y)
        * Mat3::rotate_x(-camera.rotation.x);

    // Convert world space to camera space
    triangles.iter_mut().for_each(|triangle| {
        // Apply transformations to the vertices
        triangle.vertices.iter_mut().for_each(|vertex| {
            // Move everything in the world opposite to the camera, i.e. if the
            // camera moves to the left, everything else moves to the right.
            *vertex -= camera.position;

            // Likewise, you can perform rotations as well. If the camera rotates
            // to the left with angle alpha, everything else rotates away from the
            // camera to the right with angle -alpha.
            *vertex = camera_rotation_matrix * *vertex

            // TODO: Implement camera look_at
        });
    });

    // TODO: Cull triangles who's normals are facing in the same direction as the camera using dot product
    // TODO: Cull triangles completely outside the viewing frustum
    // TODO: Clip triangles that are partially outside the viewing frustum by cutting them into 2 triangles

    // Project triangles to 2 pixel coordinates
    triangles.iter_mut().for_each(|triangle| {
        triangle.vertices.iter_mut().for_each(|vertex| {
            // 2d Projection
            vertex.x /= (vertex.z + 10.0) * 0.1;
            vertex.y /= (vertex.z + 10.0) * 0.1;

            // Mirror across x axis so that we are not upside down
            vertex.y *= -1.0;

            // Scale up to pixel space
            vertex.x *= CANVAS_WIDTH as f32 / 8.0;
            vertex.y *= CANVAS_HEIGHT as f32 / 8.0;

            // Translate (0, 0) to be in the center of the screen
            vertex.x += CANVAS_WIDTH as f32 / 2.0;
            vertex.y += CANVAS_HEIGHT as f32 / 2.0;
        })
    });

    triangles
}
