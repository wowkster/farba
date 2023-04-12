use farba::{Canvas, RGBAColor};

fn main() {
    let mut canvas = Canvas::new(400, 400);

    canvas.fill(RGBAColor::WHITE);

    // Left eye
    canvas.rect(80, 80, 30, 80, RGBAColor::BLACK);

    // Right eye
    canvas.rect(290, 80, 30, 80, RGBAColor::BLACK);

    // Mouth
    canvas.rect(80, 260, 30, 60, RGBAColor::BLACK);
    canvas.rect(290, 260, 30, 60, RGBAColor::BLACK);
    canvas.rect(80, 290, 240, 30, RGBAColor::BLACK);

    canvas.save_to_file("./examples/rectangle.png");
}
