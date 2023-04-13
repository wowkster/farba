use farba::{Canvas, RGBAColor};

fn main() {
    let mut canvas = Canvas::new(400, 400);

    canvas.fill(RGBAColor::WHITE);

    canvas.triangle(100, 300, 200, 100, 300, 300, RGBAColor::RED);

    canvas.save_to_file("./examples/triangle.png");
}
